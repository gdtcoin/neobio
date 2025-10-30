use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::VestingSchedule;
use crate::errors::VestingError;

/// 提取已释放的代币
pub fn claim(ctx: Context<Claim>) -> Result<()> {
    let clock = Clock::get()?;

    // 计算可提取金额
    let claimable_amount = ctx.accounts.vesting_schedule.get_claimable_amount(clock.unix_timestamp)?;

    require!(claimable_amount > 0, VestingError::NothingToClaim);



    //如果可领数量+已领数量大于总金额，则返回错误
    require!(ctx.accounts.vesting_schedule.claimed_amount + claimable_amount <= ctx.accounts.vesting_schedule.total_amount, VestingError::MathOverflow);

    // 生成PDA签名种子
    let seeds = &[
        b"vesting",
        ctx.accounts.vesting_schedule.creator.as_ref(),
        ctx.accounts.vesting_schedule.beneficiary.as_ref(),
        ctx.accounts.vesting_schedule.mint.as_ref(),
        &[ctx.bumps.vesting_schedule],
    ];
    let signer_seeds = &[&seeds[..]];

    // 从托管账户转移代币给受益人
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.beneficiary_token_account.to_account_info(),
            authority: ctx.accounts.vesting_schedule.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, claimable_amount)?;

    // 更新已提取金额
    ctx.accounts.vesting_schedule.claimed_amount = ctx.accounts.vesting_schedule.claimed_amount
        .checked_add(claimable_amount)
        .ok_or(VestingError::MathOverflow)?;

    msg!(
        "Claimed {} tokens. Progress: {}%",
        claimable_amount,
        ctx.accounts.vesting_schedule.get_vesting_progress(clock.unix_timestamp)
    );
    Ok(())
}

/// 提取代币的账户验证
#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    
    /// 释放计划账户
    #[account(
        mut,
        constraint = vesting_schedule.beneficiary == beneficiary.key() @ VestingError::Unauthorized,
        seeds = [
    b"vesting",
    vesting_schedule.creator.as_ref(),
    vesting_schedule.beneficiary.as_ref(),
    vesting_schedule.mint.as_ref(),
  ],
        bump
    )]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    
    /// 托管代币账户
    #[account(
       mut,
        constraint = vault_token_account.mint == vesting_schedule.mint @ VestingError::InvalidTokenMint,
        constraint = vault_token_account.owner == vesting_schedule.key() @ VestingError::Unauthorized
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    /// 受益人的代币账户
    #[account(
        mut,
        constraint = beneficiary_token_account.mint == vesting_schedule.mint @ VestingError::InvalidTokenMint,
        constraint = beneficiary_token_account.owner == beneficiary.key() @ VestingError::Unauthorized
    )]
    pub beneficiary_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}
