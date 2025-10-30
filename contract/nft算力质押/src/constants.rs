// Token程序地址
pub static TOKEN_PROGRAM_BYTES: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub static NFT_TOKEN_PROGRAM_BYTES: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

// 算力精度
pub static COMPUTATION_DECIMALS: u64 = 10u64.pow(12);

// 种子常量
pub static STAKING_SEED: &[u8] = b"staking_instance";
pub static LPTOKEN_SEED: &[u8] = b"lp_token";
pub static NFT_MINING_SYSTEM_SEED: &[u8] = b"nft_mining_system";
pub static ORDER_INFO_SEED: &[u8] = b"order_info";
pub static MEMBER_TIER_SEED: &[u8] = b"member_tier";
pub static NFT_MACHINE_SEED: &[u8] = b"nft_machine";
pub static DIVIDEND_POOL_SEED: &[u8] = b"dividend_pool";
pub static GLOBAL_STATS_SEED: &[u8] = b"global_stats";

// 系统常量
pub static TOTAL_SUPPLY: u64 = 12_600_000_000_000; // 12,600,000 GDTC (考虑精度)
pub static DAILY_OUTPUT: u64 = 1_726_000_000_000;  // 1726 GDTC (考虑精度)
pub static DAYS_IN_YEAR: u64 = 365;
pub static QUARTERS_IN_YEAR: u8 = 4;
pub static DAYS_IN_QUARTER: u64 = 91; // 约91天

// 分红比例常量 (以基点为单位，10000 = 100%)
pub static DIRECT_REFERRAL_RATE: u16 = 500;        // 5%
pub static BIO_POOL_RATE: u16 = 500;            // 5%
pub static MARKETING_RATE: u16 = 1000;             // 10%
pub static GDTC_BIO_RATE: u16 = 8000;           // 80%
pub static MEMBER_DIVIDEND_RATE: u16 = 1100;       // 11%

// 季度算力缩减比例
pub static QUARTER_1_REDUCTION: u16 = 0;           // 第1季度不缩减
pub static QUARTER_2_REDUCTION: u16 = 2500;        // 第2季度缩减25%
pub static QUARTER_3_REDUCTION: u16 = 5000;        // 第3季度缩减50%
pub static QUARTER_4_REDUCTION: u16 = 7500;        // 第4季度缩减75%

// 会员等级配置
pub static TIER_1_INVESTMENT: u64 = 1_000_000_000; // 1 SOL (lamports)
pub static TIER_2_INVESTMENT: u64 = 3_000_000_000; // 3 SOL (lamports)
pub static TIER_3_INVESTMENT: u64 = 5_000_000_000; // 5 SOL (lamports)
pub static TIER_4_INVESTMENT: u64 = 10_000_000_000; // 10 SOL (lamports)

pub static TIER_1_NFT_COUNT: u8 = 1;
pub static TIER_2_NFT_COUNT: u8 = 3;
pub static TIER_3_NFT_COUNT: u8 = 5;
pub static TIER_4_NFT_COUNT: u8 = 10;

// 算力分配比例 (以基点为单位)
pub static TIER_1_POWER_RATIO: u16 = 1000;         // 10%
pub static TIER_2_POWER_RATIO: u16 = 3000;         // 30%
pub static TIER_3_POWER_RATIO: u16 = 5000;         // 50%
pub static TIER_4_POWER_RATIO: u16 = 10000;        // 100%

// 时间相关常量
pub static SECONDS_PER_DAY: u64 = 86400;
pub static SECONDS_PER_QUARTER: u64 = SECONDS_PER_DAY * DAYS_IN_QUARTER;
pub static SECONDS_PER_YEAR: u64 = SECONDS_PER_DAY * DAYS_IN_YEAR;

// 黑洞销毁地址 (示例地址，需要替换为实际地址)
pub static BLACK_HOLE_ADDRESS: &str = "11111111111111111111111111111111";

// 最小算力要求
pub static MIN_POWER_REQUIREMENT: u64 = 1_000_000_000; // 最小算力值

// 最大算力限制
pub static MAX_POWER_PER_MACHINE: u64 = 100_000_000_000_000; // 单机最大算力

// 奖励领取冷却时间 (秒)
pub static REWARD_CLAIM_COOLDOWN: u64 = 300; // 5分钟

// GDTC充值补算力价格比例 (25% U的价格)
pub static GDTC_RECHARGE_PRICE_RATIO: u16 = 2500; // 25%
