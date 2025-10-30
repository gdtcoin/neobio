use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Burn, Mint, Token, TokenAccount, Transfer},
    token_interface::TokenInterface,
};

use crate::constants::{NFT_MINING_SYSTEM_SEED, ORDER_INFO_SEED};
use super::{NftMiningSystem, OrderInfo};
use super::errors::NftStakingError;

use raydium_cp_swap::{
    cpi,
    program::RaydiumCpSwap,
    states::{AmmConfig, ObservationState, PoolState},
};

#[derive(Accounts)]
pub struct GdtcToBio<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// 系统 PDA（已初始化）
    #[account(
        mut,
        seeds = [NFT_MINING_SYSTEM_SEED],
        bump,
        constraint = nft_mining_system.is_initialized @ NftStakingError::Unauthorized,
    )]
    pub nft_mining_system: Account<'info, NftMiningSystem>,

    /// 用户状态账户
    #[account(
        mut,
        seeds = [ORDER_INFO_SEED, &order_info.order_info_index.to_le_bytes()],
        bump,
        constraint = order_info.user_address == user.key() @ NftStakingError::Unauthorized,
    )]
    pub order_info: Account<'info, OrderInfo>,

    // -------- 用户 GDTC 账户（输入）--------
    #[account(
        mut,
        constraint = user_gdtc_account.owner == user.key() @ NftStakingError::Unauthorized,
        constraint = user_gdtc_account.mint == nft_mining_system.gdtc_mint @ NftStakingError::InvalidTokenMint,
    )]
    pub user_gdtc_account: Box<Account<'info, TokenAccount>>,

    // -------- 用户 BIO 账户（输出）--------
    #[account(
        mut,
        constraint = user_bio_account.owner == user.key() @ NftStakingError::Unauthorized,
        constraint = user_bio_account.mint == nft_mining_system.bio_mint @ NftStakingError::InvalidTokenMint,
    )]
    pub user_bio_account: Box<Account<'info, TokenAccount>>,

    //黑洞地址的bio账户
    #[account(
        mut,
        constraint = black_hole_bio_account.owner == nft_mining_system.black_hole_address @ NftStakingError::Unauthorized,
        constraint = black_hole_bio_account.mint == nft_mining_system.bio_mint @ NftStakingError::InvalidTokenMint,
    )]
    pub black_hole_bio_account: Account<'info, TokenAccount>,



    // -------- 全网分红账户 bio_mint --------
    #[account(
        mut,
        constraint = pool_address_bio_mint.mint == nft_mining_system.bio_mint @ NftStakingError::InvalidBioMint,
        constraint = pool_address_bio_mint.owner == nft_mining_system.pool_address @ NftStakingError::InvalidPoolAddress,
    )]
    pub pool_address_bio_mint: Account<'info, TokenAccount>,

    // -------- GDTC Mint 账户 --------
    #[account(
        mut,
        constraint = gdtc_mint.key() == nft_mining_system.gdtc_mint @ NftStakingError::InvalidTokenMint,
    )]
    pub gdtc_mint: Box<Account<'info, Mint>>,

    // -------- BIO Mint 账户 --------
    #[account(
        mut,
        constraint = bio_mint.key() == nft_mining_system.bio_mint @ NftStakingError::InvalidTokenMint,
    )]
    pub bio_mint: Box<Account<'info, Mint>>,

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
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// GDTC-BIO 兑换池状态
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// GDTC 输入金库
    #[account(mut)]
    pub input_vault: Box<Account<'info, TokenAccount>>,

    /// BIO 输出金库
    #[account(mut)]
    pub output_vault: Box<Account<'info, TokenAccount>>,

    /// GDTC 输入代币程序
    pub input_token_program: Interface<'info, TokenInterface>,

    /// BIO 输出代币程序
    pub output_token_program: Interface<'info, TokenInterface>,

    /// GDTC 输入代币 Mint
    pub input_token_mint: Box<Account<'info, Mint>>,

    /// BIO 输出代币 Mint
    pub output_token_mint: Box<Account<'info, Mint>>,

    /// GDTC-BIO 兑换池的观察状态
    #[account(mut)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    // -------- 系统程序 --------
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> GdtcToBio<'info> {
    pub fn process(&mut self) -> Result<()> {
       
        let gdtc_amount = self.order_info.remaining_gdtc;
        require!(gdtc_amount > 0, NftStakingError::InsufficientBalance);
        require!(gdtc_amount <= self.user_gdtc_account.amount, NftStakingError::InsufficientBalance);

        // 验证用户状态
        require!(
            self.order_info.burn_gdtc, 
            NftStakingError::Unauthorized
        );

        require!(
            !self.order_info.burn_bio, 
            NftStakingError::Unauthorized
        );

        // 记录兑换前 BIO 余额
        let bio_before = self.user_bio_account.amount;

   

        // CPI: Raydium swap（GDTC -> BIO
        {
            let cpi_accounts = cpi::accounts::Swap {
                payer: self.user.to_account_info(),
                authority: self.authority.to_account_info(),
                amm_config: self.amm_config.to_account_info(),
                pool_state: self.pool_state.to_account_info(),
                input_token_account: self.user_gdtc_account.to_account_info(),
                output_token_account: self.user_bio_account.to_account_info(),
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

            cpi::swap_base_input(cpi_ctx, gdtc_amount, 0)?;
        }

        // 重新加载账户以获取最新余额
        self.user_bio_account.reload()?;
        
        // 计算兑换结果
        let bio_after = self.user_bio_account.amount;
        let bio_received = bio_after
            .checked_sub(bio_before)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        
        // 给全网分红转5%
        let transfer_amount = bio_received * 5 / 45;
        let cpi_accounts = Transfer {
            from: self.user_bio_account.to_account_info(),
            to: self.pool_address_bio_mint.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, transfer_amount)?;

        let bio_to_burn = bio_received * 40 / 45;

       
        if bio_to_burn > 0 {
            let cpi_accounts = Transfer {
                from: self.user_bio_account.to_account_info(),
                to: self.black_hole_bio_account.to_account_info(),
                authority: self.user.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, bio_to_burn)?;
        }else {
            return Err(NftStakingError::Unauthorized.into());
        }

        // 重新加载账户以获取最终余额
        self.user_bio_account.reload()?;

        // 更新用户状态
        {
            let user = &mut self.order_info;
            user.bio_amount = user.bio_amount
                .checked_add(bio_received)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            user.burn_bio = true;
            user.remaining_gdtc = user.remaining_gdtc
                .checked_sub(gdtc_amount)
                .ok_or(ProgramError::ArithmeticOverflow)?;
        }

        // 记录操作日志
        msg!("用户: {}", self.user.key());
        msg!("GDTC 支付: {}", gdtc_amount);
        msg!("BIO 获得: {}", bio_received);
        msg!("BIO 销毁 (50%): {}", bio_to_burn);
        msg!("BIO 剩余: {}", bio_received - bio_to_burn);

        Ok(())
    }
}
