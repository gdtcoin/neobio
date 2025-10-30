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
pub struct GdtcToBio<'info> {
    // -------- 全局信息 --------
    /// CHECK: 众筹项目信息账户
    #[account(
        mut,
        seeds = [CROWDFUNDING_SEED],
        bump,
        constraint = crowdfunding_info.initialized @ CrowdfundingError::Unauthorized,
    )]
    pub crowdfunding_info: Account<'info, CrowdfundingInfo>,

    //    /// CHECK: 销售期信息账户
    //    #[account(
    //     mut,
    //     seeds = [SALE_PHASE_SEED, &id.to_le_bytes()],
    //     bump,
    //     constraint = sale_phase.active @ CrowdfundingError::PhaseSoldOut,
    // )]
    // pub sale_phase: Account<'info, SalePhase>,

    /// CHECK: 用户购买记录账户
    #[account(
       mut,
        seeds = [
            USER_PURCHASE_SEED,
            user.key().as_ref(),
            &id.to_le_bytes(),
            &user_purchase.purchase_id.to_le_bytes(),
        ],
        bump
    )]
    pub user_purchase: Account<'info, UserPurchase>,


    // -------- 用户信息 --------
    #[account(mut)]
    pub user: Signer<'info>,

    // -------- GDTC 侧（输入）--------

    //用户的gdtc token账户
    #[account(
        mut,
        constraint = user_gdtc_token_account.mint == crowdfunding_info.gdtc_mint_account @ CrowdfundingError::UsdtMintAccountIsNotMatch,
        constraint = user_gdtc_token_account.owner == user.key() @ CrowdfundingError::Unauthorized,
    )]
    pub user_gdtc_token_account: Account<'info, TokenAccount>,

    //用户的bio token账户

    #[account(
        mut,
        constraint = user_bio_token_account.mint == crowdfunding_info.bio_mint_account @ CrowdfundingError::UsdtMintAccountIsNotMatch,
        constraint = user_bio_token_account.owner == user.key() @ CrowdfundingError::Unauthorized,
    )]
    pub user_bio_token_account: Account<'info, TokenAccount>,


    //BIO 黑洞地址token account

    #[account(
        mut,
        constraint = bio_blackhole_token_account.mint == crowdfunding_info.bio_mint_account @ CrowdfundingError::UsdtMintAccountIsNotMatch,
        constraint = bio_blackhole_token_account.owner == crowdfunding_info.gdtc_blackhole_address @ CrowdfundingError::Unauthorized,
    )]
    pub bio_blackhole_token_account: Account<'info, TokenAccount>,

    // -------- BIO 侧（输出）--------


    // -------- Raydium CP Swap 程序 --------
    pub cp_swap_program: Program<'info, RaydiumCpSwap>,

    /// CHECK: Raydium 权限账户
    #[account(
        seeds = [raydium_cp_swap::AUTH_SEED.as_bytes()],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    // -------- GDTC -> BIO 兑换池配置 --------
    /// GDTC-BIO 兑换池的 AMM 配置
    #[account(mut)]
    pub gdtc_bio_amm_config: Box<Account<'info, AmmConfig>>,

    /// GDTC-BIO 兑换池状态
    #[account(mut)]
    pub gdtc_bio_pool_state: AccountLoader<'info, PoolState>,

    /// GDTC 输入金库
    #[account(mut)]
    pub gdtc_bio_input_vault: Account<'info, TokenAccount>,

    /// BIO 输出金库
    #[account(mut)]
    pub gdtc_bio_output_vault: Account<'info, TokenAccount>,

    /// GDTC 输入代币程序
    pub gdtc_bio_input_token_program: Interface<'info, TokenInterface>,

    /// BIO 输出代币程序
    pub gdtc_bio_output_token_program: Interface<'info, TokenInterface>,

    /// GDTC 输入代币 Mint
    pub gdtc_bio_input_token_mint: Box<Account<'info, Mint>>,

    /// BIO 输出代币 Mint
    pub gdtc_bio_output_token_mint: Box<Account<'info, Mint>>,

    /// GDTC-BIO 兑换池的观察状态
    #[account(mut)]
    pub gdtc_bio_observation_state: AccountLoader<'info, ObservationState>,

    // -------- 系统程序 --------
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> GdtcToBio<'info> {
    pub fn process(&mut self, shares_to_buy: u64, id: u64) -> Result<()> {

       
        require!(self.user_purchase.burn_gdtc, CrowdfundingError::InvalidShareAmount);
        require!(self.user_purchase.remaining_gdtc > 0, CrowdfundingError::InvalidShareAmount);
        require!(!self.user_purchase.burn_bio, CrowdfundingError::InvalidShareAmount);

        let bio_before = self.user_bio_token_account.amount;
        // ========== CPI 2: Raydium swap（GDTC -> BIO ==========
        {
           
            let cpi_accounts = cpi::accounts::Swap {
                payer: self.user.to_account_info(),
                authority: self.authority.to_account_info(),
                amm_config: self.gdtc_bio_amm_config.to_account_info(),
                pool_state: self.gdtc_bio_pool_state.to_account_info(),
                input_token_account: self.user_gdtc_token_account.to_account_info(),
                output_token_account: self.user_bio_token_account.to_account_info(),
                input_vault: self.gdtc_bio_input_vault.to_account_info(),
                output_vault: self.gdtc_bio_output_vault.to_account_info(),
                input_token_program: self.gdtc_bio_input_token_program.to_account_info(),
                output_token_program: self.gdtc_bio_output_token_program.to_account_info(),
                input_token_mint: self.gdtc_bio_input_token_mint.to_account_info(),
                output_token_mint: self.gdtc_bio_output_token_mint.to_account_info(),
                observation_state: self.gdtc_bio_observation_state.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(
                self.cp_swap_program.to_account_info(),
                cpi_accounts
            );

            cpi::swap_base_input(cpi_ctx, self.user_purchase.remaining_gdtc, 0)?;
        }


        self.user_bio_token_account.reload()?;

        let bio_after = self.user_bio_token_account.amount;

        let bio_amount = bio_after - bio_before;
        msg!("BIO 兑换数量: {:?}", bio_amount);

        // ========== CPI 3: 销毁部分 BIO 转帐到黑洞地址 ==========
      
        if bio_amount > 0 {
            let cpi_accounts = Transfer {
                from: self.user_bio_token_account.to_account_info(),
                to: self.bio_blackhole_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, bio_amount)?;
        }else{
            return Err(CrowdfundingError::InvalidShareAmount.into());
        }

        let user_purchase = &mut self.user_purchase;

        user_purchase.bio_amount = bio_amount;
        user_purchase.burn_bio = true;


        // ========== 第四阶段：记录gdtc购买日志 ==========
        msg!("用户: {}", self.user.key());
        msg!("期数: {}", id);
        msg!("份额数量: {}", shares_to_buy);
        msg!("GDTC 支付: {}", self.user_purchase.remaining_gdtc);
        msg!("BIO 获得: {}", bio_amount);
        //销毁bio 数量
        msg!("销毁bio 数量: {:?}", bio_amount);
        msg!("购买时间: {}", self.user_purchase.purchase_time);
       
    
        Ok(())
    }
}
