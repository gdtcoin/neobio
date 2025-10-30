use anchor_lang::prelude::*;

// 导出所有模块
pub mod initialize_system;

pub mod errors;

pub mod wsol_gdtc;
pub mod gdtc_bio;
pub mod utils;
pub mod enter_staking;
pub mod cancel_staking;
pub mod claim_rewards;
pub mod add_staking;
pub mod usdt_wsol;
pub mod claim_nft;
// pub mod tools;

// NFT算力挖矿系统主结构体
#[account]
pub struct NftMiningSystem {
    pub authority: Pubkey,                    // 管理员账户
    pub is_initialized: bool,                 // 是否初始化
    pub usdt_mint: Pubkey,                   // usdt代币Mint地址
    pub wsol_mint: Pubkey,                   // wSOL代币Mint地址
    pub gdtc_mint: Pubkey,                   // GDTC代币Mint地址
    pub bio_mint: Pubkey,                    // BIO代币Mint地址

    //全网分红池地址
    pub pool_address: Pubkey,
    //市场分红地址
    pub market_pool_address: Pubkey,
    //黑洞地址
    pub black_hole_address: Pubkey,
    // admin
    pub admin: Pubkey,
    pub total_supply: u64,                    // 总供应量 12,600,000
    pub daily_output: u64,                    // 每日产出 1726
    pub start_timestamp: u64,                 // 开始时间戳
    pub pool: StakingPool,  
    pub order_info_index: u64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StakingPool {
    pub reward_token_per_sec: u64, // 每秒奖励代币数量
    pub accumulated_reward_per_share: u64, // 累计奖励分摊
    pub last_reward_timestamp: u64, // 上次更新奖励的时间戳
    pub total_shares: u64, // 该池中质押的总份额
}

// 用户结构体 - 简化版
#[account]
pub struct OrderInfo {
    pub user_address: Pubkey,                    // 用户地址
    pub order_info_index: u64,                    // 订单信息索引

    pub user_superior_account: Pubkey,           // 用户的上级 Token 账户

    pub total_power: u64,                        // 总算力
    pub accumulated_reward: u64,                 // 累计获得的奖励
    pub last_claim_timestamp: u64,               // 最后领取时间

    pub investment_amount: u64,                  // 投资金额 (Usdt)
    pub is_transfer_usdt: bool,                  //是否已转入usdt

    pub transfer_wsol_amount: u64,                //购买的wsol数量
 
    pub is_init: bool,                           // 是否已质押
    pub stake_start_time: u64,                   // 质押开始时间
    pub reward_debt: u64,                        // 用户奖励债务
    pub is_staked: bool,                         // 用户是否已质押
    pub receivedReward: u64,                     //已领取收益

    pub gdtc_amount :u64,                        //购买到的gdtc数量
    pub burn_gdtc :bool,                         //是否销毁gdtc
    pub remaining_gdtc :u64,                     //剩余gdtc 数量
    pub bio_amount :u64,                         //购买到的bio数量
    pub burn_bio :bool,                          //是否销毁bio

    //nft 是否发放
    pub is_nft_minted: bool,                    //是否已发放nft
    pub nft_minted_time: u64,                   //nft发放时间
    //nft mint地址
    pub nft_mint_address: Pubkey,               //nft mint地址
}







