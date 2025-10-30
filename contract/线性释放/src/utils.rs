use anchor_lang::prelude::*;
use crate::state::VestingSchedule;
use crate::errors::VestingError;

/// 计算可提取金额的辅助函数
pub fn calculate_claimable_amount(vesting_schedule: &VestingSchedule, current_time: i64) -> Result<u64> {
    vesting_schedule.get_claimable_amount(current_time)
}

/// 验证释放参数（旧版本，保持兼容性）
pub fn validate_vesting_params(
    total_amount: u64,
    start_time: i64,
    duration: i64,
    current_time: i64,
) -> Result<()> {
    require!(total_amount > 0, VestingError::InvalidAmount);
    require!(duration > 0, VestingError::InvalidDuration);
    require!(start_time >= current_time, VestingError::InvalidStartTime);
    Ok(())
}

/// 验证释放参数（新版本，支持周期性释放）
pub fn validate_vesting_params_v2(
    total_amount: u64,
    start_time: i64,
    period_count: u32,
    current_time: i64,
) -> Result<()> {
    require!(total_amount > 0, VestingError::InvalidAmount);
    require!(period_count > 0, VestingError::InvalidPeriodCount);
    require!(start_time >= current_time, VestingError::InvalidStartTime);
    Ok(())
}

/// 生成释放计划的PDA种子
pub fn get_vesting_schedule_seeds<'a>(
    creator: &'a Pubkey,
    beneficiary: &'a Pubkey,
    mint: &'a Pubkey,
) -> [&'a [u8]; 4] {
    [
        b"vesting",
        creator.as_ref(),
        beneficiary.as_ref(),
        mint.as_ref(),
    ]
}

/// 生成托管账户的PDA种子
pub fn get_vault_seeds<'a>(vesting_schedule_key: &'a Pubkey) -> [&'a [u8]; 2] {
    [b"vault", vesting_schedule_key.as_ref()]
}

/// 计算释放进度百分比
pub fn calculate_vesting_progress(
    start_time: i64,
    duration: i64,
    current_time: i64,
) -> u8 {
    if current_time < start_time {
        return 0;
    }
    
    if current_time >= start_time + duration {
        return 100;
    }

    let elapsed_time = current_time - start_time;
    let progress = (elapsed_time as u128 * 100) / duration as u128;
    progress.min(100) as u8
}

/// 格式化时间戳为可读字符串（用于日志）
pub fn format_timestamp(timestamp: i64) -> String {
    format!("Unix timestamp: {}", timestamp)
}

/// 验证代币账户所有权
pub fn validate_token_account_owner(
    token_account: &anchor_spl::token::TokenAccount,
    expected_owner: &Pubkey,
) -> Result<()> {
    require!(
        token_account.owner == *expected_owner,
        VestingError::Unauthorized
    );
    Ok(())
}

/// 验证代币账户mint
pub fn validate_token_account_mint(
    token_account: &anchor_spl::token::TokenAccount,
    expected_mint: &Pubkey,
) -> Result<()> {
    require!(
        token_account.mint == *expected_mint,
        VestingError::InvalidTokenMint
    );
    Ok(())
}

/// 检查代币账户余额是否足够
pub fn check_sufficient_balance(
    token_account: &anchor_spl::token::TokenAccount,
    required_amount: u64,
) -> Result<()> {
    require!(
        token_account.amount >= required_amount,
        VestingError::InsufficientBalance
    );
    Ok(())
}
