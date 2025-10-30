use anchor_lang::prelude::*;
use crate::state::VestingSchedule;

/// 查询可提取金额
pub fn get_claimable_amount(ctx: Context<GetClaimableAmount>) -> Result<u64> {
    let vesting_schedule = &ctx.accounts.vesting_schedule;
    let clock = Clock::get()?;
    
    let claimable_amount = vesting_schedule.get_claimable_amount(clock.unix_timestamp)?;
    msg!("Claimable amount: {}", claimable_amount);
    Ok(claimable_amount)
}

/// 查询可提取金额的账户验证
#[derive(Accounts)]
pub struct GetClaimableAmount<'info> {
    /// 释放计划账户
    pub vesting_schedule: Account<'info, VestingSchedule>,
}
