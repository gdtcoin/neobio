use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Burn, Mint, Token, TokenAccount, Transfer},
    token_interface::TokenInterface,
};

use crate::constants::{CROWDFUNDING_SEED, USER_PURCHASE_SEED};
use super::{CrowdfundingInfo, SalePhase, UserPurchase};
use super::errors::CrowdfundingError;

use raydium_cp_swap::{
    cpi,
    program::RaydiumCpSwap,
    states::{AmmConfig, ObservationState, PoolState},
};

#[derive(Accounts)]
#[instruction(shares_to_buy: u64, id: u64)]
pub struct WsolGdtc<'info> {
    // -------- 全局与期数 --------
    /// CHECK: 众筹项目信息账户
    #[account(
        mut,
        seeds = [CROWDFUNDING_SEED],
        bump,
        constraint = crowdfunding_info.initialized @ CrowdfundingError::Unauthorized,
    )]
    pub crowdfunding_info: Account<'info, CrowdfundingInfo>,


    /// CHECK: 用户购买记录账户
    #[account(
        mut,
        seeds = [
            USER_PURCHASE_SEED,
            user.key().as_ref(),
            &id.to_le_bytes(),
            &user_purchase.purchase_id.to_le_bytes(),
        ],
        bump,
        constraint = user_purchase.user == user.key() @ CrowdfundingError::Unauthorized,
    )]
    pub user_purchase: Account<'info, UserPurchase>,

    // -------- 购买者 --------
    #[account(mut)]
    pub user: Signer<'info>,

    // -------- WSOL 侧 --------
    /// 用户的 WSOL 账户
    #[account(
        mut,
        constraint = user_wsol_account.mint == crowdfunding_info.wsol_mint_account @ CrowdfundingError::UsdtMintAccountIsNotMatch,
        constraint = user_wsol_account.owner == user.key() @ CrowdfundingError::Unauthorized,
    )]
    pub user_wsol_account: Account<'info, TokenAccount>,

    //用户的gdtc token账户
    #[account(
        mut,
        constraint = user_gdtc_token_account.mint == crowdfunding_info.gdtc_mint_account @ CrowdfundingError::UsdtMintAccountIsNotMatch,
        constraint = user_gdtc_token_account.owner == user.key() @ CrowdfundingError::Unauthorized,
    )]
    pub user_gdtc_token_account: Account<'info, TokenAccount>,

    //gdtc 黑洞地址token account
    #[account(
        mut,
        constraint = gdtc_blackhole_token_account.mint == crowdfunding_info.gdtc_mint_account @ CrowdfundingError::UsdtMintAccountIsNotMatch,
        constraint = gdtc_blackhole_token_account.owner == crowdfunding_info.gdtc_blackhole_address @ CrowdfundingError::Unauthorized,
    )]
    pub gdtc_blackhole_token_account: Account<'info, TokenAccount>,

    // -------- Raydium CP Swap 程序 --------
    pub cp_swap_program: Program<'info, RaydiumCpSwap>,

    /// CHECK: Raydium 权限账户
    #[account(
        seeds = [raydium_cp_swap::AUTH_SEED.as_bytes()],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    // -------- WSOL -> GDTC 兑换池配置 --------
    /// WSOL-GDTC 兑换池的 AMM 配置
    #[account(mut)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// WSOL-GDTC 兑换池状态
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// WSOL 输入金库
    #[account(mut)]
    pub input_vault: Box<Account<'info, TokenAccount>>,

    /// GDTC 输出金库
    #[account(mut)]
    pub output_vault: Box<Account<'info, TokenAccount>>,

    /// WSOL 输入代币程序
    pub input_token_program: Interface<'info, TokenInterface>,

    /// GDTC 输出代币程序
    pub output_token_program: Interface<'info, TokenInterface>,

    /// WSOL 输入代币 Mint
    pub input_token_mint: Box<Account<'info, Mint>>,

    /// GDTC 输出代币 Mint
    pub output_token_mint: Box<Account<'info, Mint>>,

    /// WSOL-GDTC 兑换池的观察状态
    #[account(mut)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    //增加后台签名
    /// CHECK: 项目签名账户
    #[account(mut,
        constraint = project_signer.key() == crowdfunding_info.project_signer @ CrowdfundingError::Unauthorized,
    )]
    pub project_signer: Signer<'info>,

    // -------- 系统程序 --------
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> WsolGdtc<'info> {
    pub fn process(&mut self, shares_to_buy: u64, id: u64) -> Result<()> {
        
        // let crowdfunding = &mut self.crowdfunding_info;
        // ========== 第一阶段：验证和计算（只读） ==========
        let now = Clock::get()?.unix_timestamp;
        
        // 验证输入参数
        require!(shares_to_buy > 0, CrowdfundingError::InvalidShareAmount);
        require!(id <= self.crowdfunding_info.phase_count as u64, CrowdfundingError::InvalidPhaseId);
      

       
        // 验证用户 WSOL 余额
        require!(
            self.user_wsol_account.amount >= self.user_purchase.wsol_amount, 
            CrowdfundingError::InvalidShareAmount
        );

        // 记录兑换前余额
        let gdtc_before = self.user_gdtc_token_account.amount;



        
{
        // CPI 2: Raydium swap（WSOL -> GDTC）
        

            let cpi_accounts = cpi::accounts::Swap {
                payer: self.user.to_account_info(),  
                authority: self.authority.to_account_info(),
                amm_config: self.amm_config.to_account_info(),
                pool_state: self.pool_state.to_account_info(),
                input_token_account: self.user_wsol_account.to_account_info(),
                output_token_account: self.user_gdtc_token_account.to_account_info(),
                input_vault: self.input_vault.to_account_info(),
                output_vault: self.output_vault.to_account_info(),
                input_token_program: self.input_token_program.to_account_info(),
                output_token_program: self.output_token_program.to_account_info(),
                input_token_mint: self.input_token_mint.to_account_info(),
                output_token_mint: self.output_token_mint.to_account_info(),
                observation_state: self.observation_state.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(
                self.cp_swap_program.to_account_info(),
                cpi_accounts
            );

            cpi::swap_base_input(cpi_ctx, self.user_purchase.wsol_amount, 0)?;
 
        
        }
        
        // 重新加载账户以获取最新余额
        self.user_gdtc_token_account.reload()?;
        // self.crowdfunding_info.reload()?;
        
        // 计算兑换结果
        let gdtc_after = self.user_gdtc_token_account.amount;
        let gdtc_received = gdtc_after
            .checked_sub(gdtc_before)
            .ok_or(CrowdfundingError::ArithmeticOverflow)?;
        
        // 计算需要销毁的 GDTC 数量（50%）
        let gdtc_to_burn = gdtc_received / 2;

        // CPI 3: 销毁部分 GDTC（50%）
        if gdtc_to_burn > 0 {


        //用户销毁gdtc_to_burn 的gdtc ,向黑洞地址转帐
        {
            let cpi_accounts = Transfer {
                from: self.user_gdtc_token_account.to_account_info(),
                to: self.gdtc_blackhole_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, gdtc_to_burn)?;
        }

    }

        // // 重新加载账户以获取最终余额
        self.user_gdtc_token_account.reload()?;

        if self.user_purchase.burn_gdtc {
            return Err(CrowdfundingError::AlreadyBurned.into());
        }

        // ========== 第三阶段：更新状态（短时间持有 &mut） ==========
        {
      
            let user_purchase = &mut self.user_purchase;
            // 更新用户购买记录
            user_purchase.gdtc_amount = gdtc_received;
            user_purchase.burn_gdtc = true;
            user_purchase.remaining_gdtc = gdtc_received - gdtc_to_burn;
       
        }

        // ========== 第四阶段：记录gdtc购买日志 ==========
        msg!("用户: {}", self.user.key());
        msg!("期数: {}", id);
        msg!("份额数量: {}", shares_to_buy);
        msg!("WSOL 支付: {}", self.user_purchase.wsol_amount);
        msg!("GDTC 获得: {}", gdtc_received);
        msg!("GDTC 销毁 (50%): {}", gdtc_to_burn);

        Ok(())
    }
}