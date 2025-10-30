use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::VestingSchedule;
use crate::errors::VestingError;
use crate::utils::format_timestamp;

/// 取消释放计划（仅创建者可用，将剩余代币退还给创建者）
pub fn cancel_vesting(ctx: Context<CancelVesting>) -> Result<()> {
    let vesting_schedule = &ctx.accounts.vesting_schedule;
    let clock = Clock::get()?;

    // 计算需要退还的金额（总金额 - 已提取金额）
    let remaining_amount = vesting_schedule.total_amount
        .checked_sub(vesting_schedule.claimed_amount)
        .ok_or(VestingError::MathOverflow)?;

    if remaining_amount > 0 {
        // 生成PDA签名种子
        let seeds = &[
            b"vesting",
            vesting_schedule.creator.as_ref(),
            vesting_schedule.beneficiary.as_ref(),
            vesting_schedule.mint.as_ref(),
            &[ctx.bumps.vesting_schedule],
        ];
        let signer_seeds = &[&seeds[..]];

        // 将剩余代币退还给创建者
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.creator_token_account.to_account_info(),
                authority: ctx.accounts.vesting_schedule.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, remaining_amount)?;
    }

    msg!(
        "Vesting schedule cancelled. Returned {} tokens to creator at {}",
        remaining_amount,
        format_timestamp(clock.unix_timestamp)
    );
    Ok(())
}

/// 取消释放计划的账户验证（仅创建者可用）
#[derive(Accounts)]
pub struct CancelVesting<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    /// 释放计划账户
    #[account(
        mut,
        constraint = vesting_schedule.creator == creator.key() @ VestingError::Unauthorized,
        seeds = [b"vesting", creator.key().as_ref(), vesting_schedule.beneficiary.as_ref(), vesting_schedule.mint.as_ref()],
        bump,
        close = creator
    )]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    
    /// 托管代币账户
    #[account(
        mut,
        constraint = vault_token_account.mint == vesting_schedule.mint @ VestingError::InvalidTokenMint,
        seeds = [b"vault", vesting_schedule.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    /// 创建者的代币账户（用于退还剩余代币）
    #[account(
        mut,
        constraint = creator_token_account.mint == vesting_schedule.mint @ VestingError::InvalidTokenMint,
        constraint = creator_token_account.owner == creator.key() @ VestingError::Unauthorized
    )]
    pub creator_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}
