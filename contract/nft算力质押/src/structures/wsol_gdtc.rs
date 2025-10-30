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
pub struct WsolGdtc<'info> {
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

    // -------- 用户 WSOL 账户 --------
  
    #[account(
        mut,
        constraint = user_wsol_account.owner == user.key() @ NftStakingError::Unauthorized,
        constraint = user_wsol_account.mint == nft_mining_system.wsol_mint @ NftStakingError::InvalidTokenMint,
    )]
    pub user_wsol_account: Box<Account<'info, TokenAccount>>,

    // -------- 系统 GDTC 账户 --------
    #[account(
        mut,
        constraint = user_gdtc_account.owner == user.key() @ NftStakingError::Unauthorized,
        constraint = user_gdtc_account.mint == nft_mining_system.gdtc_mint @ NftStakingError::InvalidTokenMint,
    )]
    pub user_gdtc_account: Box<Account<'info, TokenAccount>>,


    //黑洞地址的gdtc账户
    #[account(
        mut,
        constraint = black_hole_gdtc_account.owner == nft_mining_system.black_hole_address @ NftStakingError::Unauthorized,
        constraint = black_hole_gdtc_account.mint == nft_mining_system.gdtc_mint @ NftStakingError::InvalidTokenMint,
    )]
    pub black_hole_gdtc_account: Account<'info, TokenAccount>,

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

    // -------- 系统程序 --------
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> WsolGdtc<'info> {
    pub fn process(&mut self) -> Result<()> {
    
        // 获取用户转入的 WSOL 数量
        let wsol_amount = self.order_info.transfer_wsol_amount;
        // 验证用户 WSOL 余额
        require!(
            self.user_wsol_account.amount >= wsol_amount, 
            NftStakingError::InsufficientBalance
        );

        // 记录兑换前 GDTC 余额
        let gdtc_before = self.user_gdtc_account.amount;

    
        // CPI 2: Raydium swap（WSOL -> GDTC）
        {
            let cpi_accounts = cpi::accounts::Swap {
                payer: self.user.to_account_info(),
                authority: self.authority.to_account_info(),
                amm_config: self.amm_config.to_account_info(),
                pool_state: self.pool_state.to_account_info(),
                input_token_account: self.user_wsol_account.to_account_info(),
                output_token_account: self.user_gdtc_account.to_account_info(),
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

            cpi::swap_base_input(cpi_ctx, wsol_amount, 0)?;
 
        }

        // 重新加载账户以获取最新余额
        self.user_gdtc_account.reload()?;
        
        // 计算兑换结果
        let gdtc_after = self.user_gdtc_account.amount;
        let gdtc_received = gdtc_after
            .checked_sub(gdtc_before)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        
        // 计算需要销毁的 GDTC 数量的 40/85 比例
        let gdtc_to_burn = gdtc_received * 40 / 85;

        // CPI 3: 销毁部分 GDTC（40/85）,给黑洞地址转账
        if gdtc_to_burn > 0 {
            let cpi_accounts = Transfer {
                from: self.user_gdtc_account.to_account_info(),
                to: self.black_hole_gdtc_account.to_account_info(),
                authority: self.user.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, gdtc_to_burn)?;
        }

        // 重新加载账户以获取最终余额
        self.user_gdtc_account.reload()?;

        // 更新用户状态
        {
            let user = &mut self.order_info;
            user.gdtc_amount = user.gdtc_amount
                .checked_add(gdtc_received)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            user.burn_gdtc = true;
            user.remaining_gdtc = user.remaining_gdtc
                .checked_add(gdtc_received - gdtc_to_burn)
                .ok_or(ProgramError::ArithmeticOverflow)?;
        }

        // 记录操作日志
        msg!("用户: {}", self.user.key());
        msg!("WSOL 支付: {}", wsol_amount);
        msg!("GDTC 获得: {}", gdtc_received);
        msg!("GDTC 销毁 (50%): {}", gdtc_to_burn);
        msg!("GDTC 剩余: {}", gdtc_received - gdtc_to_burn);

        Ok(())
    }
}