use anchor_lang::prelude::*;

// 模块定义
pub mod errors;          // 错误定义模块
pub mod initialize;      // 初始化逻辑模块

pub mod create_phase;    // 创建销售期逻辑模块
pub mod claim_tokens;    // 领取 Token 逻辑模块
pub mod tools;           // 工具方法模块

pub mod usdt_wsol;        // usdt 到 wsol 兑换逻辑模块
pub mod wsol_gdtc;  // 购买份额逻辑模块
pub mod gdtc_bio;        // GDTC 到 BIO 兑换逻辑模块

// 众筹项目全局信息
#[account]
pub struct CrowdfundingInfo {
    pub initialized: bool,            // 是否已初始化
    pub authority: Pubkey,            // 项目部署者
    pub admin: Pubkey,                 // 管理员
    pub usdt_mint_account: Pubkey,     // USDT Mint 地址
    pub wsol_mint_account: Pubkey,     // WSOL Mint 地址
    pub gdtc_mint_account: Pubkey,     // GDTC Token Mint 地址
    pub bio_mint_account: Pubkey,   // BIO Token Mint 地址
    pub total_shares: u64,             // 总份数（例如 800 份）
    pub sold_shares: u64,              // 已售出的份数
    pub token_per_share: u64,          // 每份对应的 Token 数量（例如 656.25）
    pub vesting_days: u64,             // 默认线性释放天数（例如 365 天）
    pub project_signer: Pubkey,        // 项目方签名公钥
    pub phase_count: u32,              // 已创建的销售期数量
    //全网分红池
    pub gdtc_pool_address: Pubkey,
    //黑洞地址
    pub gdtc_blackhole_address: Pubkey,
}

// 每一期的销售信息
#[account]
pub struct SalePhase {
    pub phase_id: u32,                 // 第几期（期数 ID）
    pub price_per_share: u64,          // 每份售价（单位 usdt）
    pub max_shares: u64,               // 本期可售份数
    pub sold_shares: u64,              // 本期已售份数
    pub start_time: i64,               // 开始时间（Unix 时间戳）
    pub end_time: i64,                 // 结束时间（Unix 时间戳）
    pub active: bool,                  // 是否处于可售状态
}

// 用户购买记录
#[account]
pub struct UserPurchase {
    pub user: Pubkey,           // 用户钱包地址
    //用户上级地址
    pub superior_address: Pubkey,
    pub phase_id: u32,          // 所属期数
    pub purchase_id: u64,       // 用户在该期的第 N 笔购买（递增编号）
    pub shares: u64,            // 购买的份额数量
    pub token_amount: u64,      // 对应的 Token 总量
    pub claimed_amount: u64,    // 已领取的 Token 数量
    pub purchase_time: i64,     // 购买时间（Unix 时间戳）
    pub vesting_days: u64,      // 该笔购买的线性释放天数
    
    pub wsol_amount :u64,       //购买到的wsol数量

    pub gdtc_amount :u64,       //购买到的gdtc数量
    pub burn_gdtc :bool,        //是否销毁gdtc
    pub remaining_gdtc :u64,    //剩余gdtc 数量
    pub bio_amount :u64,        //购买到的bio数量
    pub burn_bio :bool,         //是否销毁bio
}