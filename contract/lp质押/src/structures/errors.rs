use anchor_lang::prelude::*;

/// 质押项目错误类型
#[error_code]
pub enum StakingError {
    #[msg("Invalid stake type provided. The stake type must correspond to an existing pool.")]
    InvalidStakeType,

    #[msg("Invalid stake staked info index.")]
    InvalidStakedInfoIndex,

    #[msg("Insufficient token account balance.")]
    TokenAccountBalanceInsufficient,

    #[msg("Failed to fetch system clock.")]
    ClockUnavailable,

    #[msg("User token account mint does not match staking token mint.")]
    MintAccountIsNotMatch,

    #[msg("Arithmetic overflow occurred.")]
    Overflow,

    #[msg("Arithmetic underflow occurred.")]
    Underflow,

    #[msg("User has already staked and cannot stake again.")]
    UserAlreadyStaked,

    #[msg("User has no staking to cancel.")]
    NoStakingToCancel,

    #[msg("Staking period has not matured yet.")]
    StakingNotMatured,

    #[msg("No rewards available to claim.")]
    NoRewardsToClaim,
    #[msg("Max available to claim.")]
    MaxRewardsToClaim,

    #[msg("Insufficient reward account balance.")]
    InsufficientRewardBalance, // 奖励账户余额不足

    #[msg("No Staking available to claim.")]
    NoStakingToClaimRewards,

    #[msg("UserSuperiorTokenAccount  does not match.")]
    UserSuperiorTokenAccountIsNotMatch,

    #[msg("User address  does not match.")]
    UserAccountIsNotMatch,

    #[msg("User need cliam rewards.")]
    NeedCliamRewards,

    #[msg("The provided staking instance is not a valid staking instance for this contract.")]
    InvalidStakingInstance,

    #[msg("The staking has ended for this instance.")]
    StakingEnded,

    #[msg("The provided user instance is not a valid user instance for this contract.")]
    InvalidUserInstance,

    #[msg("Pda address  does not match.")]
    PdaAccountIsNotMatch,

    // 新增错误类型
    #[msg("User is not initialized")]
    UserNotInitialized,

    #[msg("Staking instance is not initialized")]
    StakingInstanceNotInitialized,

    #[msg("Pool is not active")]
    PoolNotActive,

    #[msg("Stake amount below minimum")]
    StakeAmountBelowMinimum,

    #[msg("Stake amount above maximum")]
    StakeAmountAboveMaximum,

    #[msg("Staking period not ended")]
    StakingPeriodNotEnded,

    #[msg("Invalid reward calculation")]
    InvalidRewardCalculation,

    #[msg("Insufficient pool liquidity")]
    InsufficientPoolLiquidity,

    #[msg("Reward rate too high")]
    RewardRateTooHigh,

    #[msg("Invalid time parameters")]
    InvalidTimeParameters,

    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Invalid stake status")]
    InvalidStakeStatus,

    #[msg("Pool is full")]
    PoolIsFull,

    #[msg("Invalid configuration")]
    InvalidConfiguration,

    #[msg("Reward distribution failed")]
    RewardDistributionFailed,

    #[msg("User not found")]
    UserNotFound,

    #[msg("Pool not found")]
    PoolNotFound,

    #[msg("Invalid stake duration")]
    InvalidStakeDuration,

    #[msg("Stake already cancelled")]
    StakeAlreadyCancelled,

    #[msg("Cannot cancel active stake")]
    CannotCancelActiveStake,

    #[msg("Invalid reward claim")]
    InvalidRewardClaim,

    #[msg("Reward calculation overflow")]
    RewardCalculationOverflow,

    #[msg("Pool configuration error")]
    PoolConfigurationError,

    #[msg("User statistics error")]
    UserStatisticsError,

    #[msg("Invalid pool state")]
    InvalidPoolState,

    #[msg("Stake amount zero")]
    StakeAmountZero,

    #[msg("Invalid user address")]
    InvalidUserAddress,

    #[msg("Invalid pool address")]
    InvalidPoolAddress,

    #[msg("Reward pool empty")]
    RewardPoolEmpty,

    #[msg("Invalid stake type")]
    InvalidStakeTypeValue,

    #[msg("Pool maintenance mode")]
    PoolMaintenanceMode,

    #[msg("User banned")]
    UserBanned,

    #[msg("Pool suspended")]
    PoolSuspended,

    #[msg("Invalid reward rate")]
    InvalidRewardRate,

    #[msg("Time calculation error")]
    TimeCalculationError,

    #[msg("State synchronization error")]
    StateSynchronizationError,

    #[msg("Staking instance has already been initialized.")]
    StakingInstanceAlreadyInitialized,
   


    #[msg("User has already been initialized.")]
    UserAlreadyInitialized,
    #[msg("User has not been staked.")]
    UserNotStaked,



#[msg("InvalidLpTokenOwner")]
    InvalidLpTokenOwner,

#[msg("InvalidLpTokenMint")]
    InvalidLpTokenMint,
    
#[msg("InvalidVaultOwner")]
InvalidVaultOwner,
#[msg("InvalidStakeAmount")]
InvalidStakeAmount,


}