use anchor_lang::prelude::*;

/// 众筹项目错误类型
#[error_code]
pub enum CrowdfundingError {

    /// 未授权操作
    #[msg("Unauthorized access")]
    Unauthorized,


    /// 派生地址不匹配
    #[msg("PDA account is not match")]
    PdaAccountIsNotMatch,

    /// usdt_mint_account 账户不存在
    #[msg("Usdt mint account is not match")]
    UsdtMintAccountIsNotMatch,


    #[msg("Usdt In account is not match")]
    UsdtInAccountIsNotMatch,

    #[msg("Invalid project signer")]
    InvalidProjectSigner,

    #[msg("Phase is sold out or not enough shares left")]
    PhaseSoldOut,

    #[msg("Nothing to claim at this time")]
    NothingToClaim,


    #[msg("Too many sale phases have been created. No more phases allowed.")]
    TooManyPhases,


    #[msg("Invalid share amount.")]
    InvalidShareAmount,

    #[msg("Token mint mismatch.")]
    TokenMintMismatch,
    #[msg("This sale phase has not started yet.")]
    PhaseNotStarted,

    #[msg("Vault token account is not owned by the program.")]
    InvalidVaultOwner,

     #[msg("Phase does not exist")]
    InvalidPhaseId,

    #[msg("Concurrent modification detected: sold_shares mismatch")]
    InvalidSoldShares,

    #[msg("Invalid start time")]
    InvalidStartTime,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("WSOL mint does not match the provided account")]
    WsolMintNotMatch,
    #[msg("GDTC mint does not match the provided account")]
    GdtcMintNotMatch,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    #[msg("Already burned")]
    AlreadyBurned,

    #[msg("NotBlackHole")]
    NotBlackHole,

    /// 余额不足
    #[msg("Insufficient balance for operation")]
    InsufficientBalance,
    #[msg("Too many rewards to claim")]
    MaxRewardsToClaim,

} 