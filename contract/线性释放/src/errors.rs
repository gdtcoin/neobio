use anchor_lang::prelude::*;

/// 自定义错误类型
#[error_code]
pub enum VestingError {
    #[msg("Invalid amount: must be greater than 0")]
    InvalidAmount,
    
    #[msg("Invalid duration: must be greater than 0")]
    InvalidDuration,
    
    #[msg("Invalid start time: must be in the future")]
    InvalidStartTime,
    
    #[msg("Nothing to claim")]
    NothingToClaim,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Unauthorized: only beneficiary can claim")]
    Unauthorized,
    
    #[msg("Unauthorized: only beneficiary can claim2")]
    Unauthorized2,
    
    #[msg("Vesting schedule not found")]
    VestingScheduleNotFound,
    
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Invalid period count: must be greater than 0")]
    InvalidPeriodCount,

    #[msg("Invalid Vesting Period")]
    InvalidVestingPeriod,
}
