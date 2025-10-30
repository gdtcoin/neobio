
// 全局众筹状态（唯一）
pub static CROWDFUNDING_SEED: &[u8] = b"crowdfunding_instance";

// 每个销售期（第 N 期）
pub static SALE_PHASE_SEED: &[u8] = b"sale_phase";

// 每个用户在某一期的购买记录
pub static USER_PURCHASE_SEED: &[u8] = b"user_purchase";


/// Mint Authority 的 PDA 种子
pub static MINT_AUTHORITY_SEED: &[u8] = b"mint_authority";

// 固定参数
pub const TOTAL_SHARES: u64 = 800;
pub const TOKEN_PER_SHARE: u64 = 656_250_000_000;  // 656.25 * 10^9 (假设代币是 9 位小数)
pub const VESTING_DAYS: u64 = 365;

// 黑洞地址 - 用于销毁代币
pub static BLACKHOLE_ADDRESS: &str = "11111111111111111111111111111111";

// GDTC 的 Mint 地址（需要根据实际情况设置）
pub static Burn_SEED: &[u8] = b"burn";
// 项目方的 GDTC 账户种子
pub static GDTC_ACCOUNT_SEED: &[u8] = b"gdtc_account";

