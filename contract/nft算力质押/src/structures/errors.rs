use anchor_lang::prelude::*;

/// 质押项目错误类型
#[error_code]
pub enum NftStakingError {
    
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("UserAccountIsNotMatch")]
    UserAccountIsNotMatch,

    #[msg("UserNotInitialized")]
    UserNotInitialized,

    #[msg("UserAlreadyStaked")]
    UserAlreadyStaked,

    #[msg("InvalidWsolMint")]
    InvalidWsolMint,

    #[msg("InvalidWsolOwner")]
    InvalidWsolOwner,

    #[msg("InvalidNftMint")]
    InvalidNftMint,

    #[msg("InvalidNftOwner")]
    InvalidNftOwner,

    #[msg("InvalidTierLevel")]
    InvalidTierLevel,

    #[msg("InsufficientWsolBalance")]
    InsufficientWsolBalance,

    #[msg("InvalidWsolAmount")]
    InvalidWsolAmount,

    #[msg("InvalidAccountArrayLength")]
    InvalidAccountArrayLength,

    #[msg("NotEnoughRemainingAccounts")]
    NotEnoughRemainingAccounts,

    #[msg("InvalidGdtcMint")]
    InvalidGdtcMint,

    #[msg("InvalidGdtcOwner")]
    InvalidGdtcOwner,

    #[msg("InvalidBioMint")]
    InvalidBioMint,

    #[msg("InvalidPoolAddress")]
    InvalidPoolAddress,

    #[msg("InvalidMarketPoolAddress")]
    InvalidMarketPoolAddress,

    #[msg("InvalidAmount")]
    InvalidAmount,

    #[msg("InsufficientBalance")]
    InsufficientBalance,

    #[msg("InvalidTokenMint")]
    InvalidTokenMint,

    #[msg("SystemAlreadyInitialized")]
    SystemAlreadyInitialized,

    #[msg("InvalidUsdtMint")]
    InvalidUsdtMint,

    #[msg("InvalidUsdtOwner")]
    InvalidUsdtOwner,

    #[msg("InvalidUsdtAmount")]
    InvalidUsdtAmount,

    #[msg("InsufficientUsdtBalance")]
    InsufficientUsdtBalance,
    #[msg("ArithmeticOverflow")]
    ArithmeticOverflow,
    #[msg("NoRewardsToClaim")]
    NoRewardsToClaim,
    #[msg("MaxRewardsToClaim")]
    MaxRewardsToClaim,
    #[msg("TokenMintMismatch")]
    TokenMintMismatch,
    #[msg("NotBlackHole")]
    NotBlackHole,

}