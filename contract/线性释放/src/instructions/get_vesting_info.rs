use anchor_lang::prelude::*;
use crate::state::{VestingSchedule, VestingPeriod};

/// 释放计划信息结构体
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VestingInfo {
    pub creator: Pubkey,
    pub beneficiary: Pubkey,
    pub mint: Pubkey,
    pub total_amount: u64,
    pub claimed_amount: u64,
    pub claimable_amount: u64,
    pub locked_amount: u64,
    pub start_time: i64,
    pub vesting_period: VestingPeriod,
    pub period_count: u32,
    pub amount_per_period: u64,
    pub completed_periods: u32,
    pub created_at: i64,
    pub progress: u8,
    pub is_fully_vested: bool,
    pub next_release_time: Option<i64>,
}

/// 查询释放计划详细信息
pub fn get_vesting_info(ctx: Context<GetVestingInfo>) -> Result<VestingInfo> {
    let vesting_schedule = &ctx.accounts.vesting_schedule;
    let clock = Clock::get()?;

    let claimable_amount = vesting_schedule.get_claimable_amount(clock.unix_timestamp)?;
    let locked_amount = vesting_schedule.get_locked_amount(clock.unix_timestamp)?;
    let progress = vesting_schedule.get_vesting_progress(clock.unix_timestamp);
    let is_fully_vested = vesting_schedule.is_fully_vested(clock.unix_timestamp);
    let completed_periods = vesting_schedule.get_completed_periods(clock.unix_timestamp);
    let next_release_time = vesting_schedule.get_next_release_time(clock.unix_timestamp);

    let info = VestingInfo {
        creator: vesting_schedule.creator,
        beneficiary: vesting_schedule.beneficiary,
        mint: vesting_schedule.mint,
        total_amount: vesting_schedule.total_amount,
        claimed_amount: vesting_schedule.claimed_amount,
        claimable_amount,
        locked_amount,
        start_time: vesting_schedule.start_time,
        vesting_period: vesting_schedule.vesting_period,
        period_count: vesting_schedule.period_count,
        amount_per_period: vesting_schedule.amount_per_period,
        completed_periods,
        created_at: vesting_schedule.created_at,
        progress,
        is_fully_vested,
        next_release_time,
    };

    let period_name = match vesting_schedule.vesting_period {
        VestingPeriod::Daily => "daily",
        VestingPeriod::Monthly => "monthly",
        VestingPeriod::Yearly => "yearly",
        VestingPeriod::Linear => "linear",
    };

    msg!(
        "Vesting info: {} release, Progress {}%, Completed periods: {}/{}, Claimable: {}",
        period_name,
        progress,
        completed_periods,
        vesting_schedule.period_count,
        claimable_amount
    );
    Ok(info)
}

/// 查询释放计划信息的账户验证
#[derive(Accounts)]
pub struct GetVestingInfo<'info> {
    /// 释放计划账户
    pub vesting_schedule: Account<'info, VestingSchedule>,
}
