use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self,  Mint, Token, TokenAccount},
    token_interface::TokenInterface,
};


use crate::constants::{CROWDFUNDING_SEED, SALE_PHASE_SEED, USER_PURCHASE_SEED};
use super::{CrowdfundingInfo, SalePhase, UserPurchase};
use super::errors::CrowdfundingError;

use raydium_cp_swap::{
    cpi,
    program::RaydiumCpSwap,
    states::{AmmConfig, ObservationState, PoolState},
};

#[derive(Accounts)]
#[instruction(shares_to_buy: u64, id: u64)]
pub struct UsdtToWsol<'info> {
    // -------- 全局与期数 --------
    /// CHECK: 众筹项目信息账户
    #[account(
        mut,
        seeds = [CROWDFUNDING_SEED],
        bump,
        constraint = crowdfunding_info.initialized @ CrowdfundingError::Unauthorized,
    )]
    pub crowdfunding_info: Account<'info, CrowdfundingInfo>,


    /// CHECK: 销售期信息账户
    #[account(
        mut,
        seeds = [SALE_PHASE_SEED, &id.to_le_bytes()],
        bump,
        constraint = sale_phase.active @ CrowdfundingError::PhaseSoldOut,
    )]
    pub sale_phase: Account<'info, SalePhase>,

    /// CHECK: 用户购买记录账户
    #[account(
        init,
        payer = user,
        seeds = [
            USER_PURCHASE_SEED,
            user.key().as_ref(),
            &id.to_le_bytes(),
            &sale_phase.sold_shares.to_le_bytes(),
        ],
        bump,
        space = 8 + core::mem::size_of::<UserPurchase>()
    )]
    pub user_purchase: Account<'info, UserPurchase>,

    // -------- 购买者 --------
    #[account(mut)]
    pub user: Signer<'info>,

    // -------- usdt 侧 --------
    /// 用户的 WSOL 账户
    #[account(
        mut,
        constraint = user_usdt_token_account.mint == crowdfunding_info.usdt_mint_account @ CrowdfundingError::UsdtMintAccountIsNotMatch,
        constraint = user_usdt_token_account.owner == user.key() @ CrowdfundingError::Unauthorized,
    )]
    pub user_usdt_token_account: Account<'info, TokenAccount>,


    //用户的wsol token账户
    #[account(
        mut,
        constraint = user_wsol_token_account.mint == crowdfunding_info.wsol_mint_account @ CrowdfundingError::UsdtMintAccountIsNotMatch,
        constraint = user_wsol_token_account.owner == user.key() @ CrowdfundingError::Unauthorized,
    )]
    pub user_wsol_token_account: Account<'info, TokenAccount>,


    // -------- Raydium CP Swap 程序 --------
    pub cp_swap_program: Program<'info, RaydiumCpSwap>,

    /// CHECK: Raydium 权限账户
    #[account(
        seeds = [raydium_cp_swap::AUTH_SEED.as_bytes()],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

        // -------- usdt -> wsol 兑换池配置 --------
        /// usdt-wsol 兑换池的 AMM 配置
    #[account(mut)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// usdt-wsol 兑换池状态
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// usdt 输入金库
    #[account(mut)]
    pub input_vault: Box<Account<'info, TokenAccount>>,

    /// wsol 输出金库
    #[account(mut)]
    pub output_vault: Box<Account<'info, TokenAccount>>,

    /// usdt 输入代币程序
    pub input_token_program: Interface<'info, TokenInterface>,

    /// wsol 输出代币程序
    pub output_token_program: Interface<'info, TokenInterface>,

    /// usdt 输入代币 Mint
    pub input_token_mint: Box<Account<'info, Mint>>,

    /// wsol 输出代币 Mint
    pub output_token_mint: Box<Account<'info, Mint>>,

    /// usdt-wsol 兑换池的观察状态
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

impl<'info> UsdtToWsol<'info> {
    pub fn process(&mut self, shares_to_buy: u64, id: u64,user_superior_address: Pubkey) -> Result<()> {
        
        // let crowdfunding = &mut self.crowdfunding_info;
        // ========== 第一阶段：验证和计算（只读） ==========
        let now = Clock::get()?.unix_timestamp;
        
        // 验证输入参数
        require!(shares_to_buy > 0, CrowdfundingError::InvalidShareAmount);
        require!(id <= self.crowdfunding_info.phase_count as u64, CrowdfundingError::InvalidPhaseId);
        require!(now >= self.sale_phase.start_time, CrowdfundingError::PhaseNotStarted);
        require!(
            self.sale_phase.sold_shares + shares_to_buy <= self.sale_phase.max_shares, 
            CrowdfundingError::PhaseSoldOut
        );

        // 计算需要的 usdt 数量
        let usdt_amount = self.sale_phase
            .price_per_share
            .checked_mul(shares_to_buy)
            .ok_or(ProgramError::InvalidArgument)?;
        
        // 验证用户 usdt 余额
        require!(
            self.user_usdt_token_account.amount >= usdt_amount, 
            CrowdfundingError::InvalidShareAmount
        );

        // 记录兑换前余额
        let wsol_before = self.user_wsol_token_account.amount;

        // 预先获取需要的值，避免后续借用冲突
        let token_per_share = self.crowdfunding_info.token_per_share;
        let vesting_days = self.crowdfunding_info.vesting_days;

        

        
{
        // CPI 2: Raydium swap（usdt -> wsol）
        
            let cpi_accounts = cpi::accounts::Swap {
                payer: self.user.to_account_info(),  
                authority: self.authority.to_account_info(),
                amm_config: self.amm_config.to_account_info(),
                pool_state: self.pool_state.to_account_info(),
                input_token_account: self.user_usdt_token_account.to_account_info(),
                output_token_account: self.user_wsol_token_account.to_account_info(),
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

            cpi::swap_base_input(cpi_ctx, usdt_amount, 0)?;
 
        }
        
        // 重新加载账户以获取最新余额
        self.user_wsol_token_account.reload()?;
 
        
        // 计算兑换结果
        let wsol_after = self.user_wsol_token_account.amount;
        let wsol_received = wsol_after
            .checked_sub(wsol_before)
            .ok_or(CrowdfundingError::ArithmeticOverflow)?;
        


        // ========== 第三阶段：更新状态（短时间持有 &mut） ==========
        {
            let phase = &mut self.sale_phase;
            // 
            let user_purchase = &mut self.user_purchase;

            // 计算对应的 Token 数量
            let token_amount = shares_to_buy
                .checked_mul(token_per_share)
                .ok_or(ProgramError::InvalidArgument)?;

            // 更新用户购买记录
            user_purchase.user = self.user.key();
            user_purchase.phase_id = phase.phase_id;
            user_purchase.purchase_id = phase.sold_shares;
            user_purchase.shares = shares_to_buy;
            user_purchase.token_amount = token_amount;
            user_purchase.claimed_amount = 0;
            user_purchase.purchase_time = now;
            user_purchase.vesting_days = vesting_days;
            user_purchase.wsol_amount = wsol_received;
            user_purchase.superior_address = user_superior_address;

            // 更新销售期信息
            phase.sold_shares = phase.sold_shares
                .checked_add(shares_to_buy)
                .ok_or(CrowdfundingError::ArithmeticOverflow)?;


            let crowdfunding = &mut self.crowdfunding_info;
        //  更新全局众筹信息
         crowdfunding.sold_shares = crowdfunding.sold_shares
         .checked_add(shares_to_buy)
         .ok_or(CrowdfundingError::ArithmeticOverflow)?;
            
        }

        // ========== 第四阶段：记录wsol购买日志 ==========
        msg!("用户: {}", self.user.key());
        msg!("期数: {}", id);
        msg!("份额数量: {}", shares_to_buy);
        msg!("WSOL 支付: {}", wsol_received);
        msg!("Token 数量: {}", shares_to_buy * token_per_share);
        msg!("购买时间: {}", now);

        Ok(())
    }
}