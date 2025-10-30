use anchor_lang::prelude::*;
use anchor_spl::token::{Mint};
use crate::structures::*;
use crate::constants::*;
use crate::errors::NftStakingError;

use std::str::FromStr;

#[derive(Accounts)]
pub struct InitializeSystem<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + core::mem::size_of::<NftMiningSystem>()+8+core::mem::size_of::<StakingPool>(),
        seeds = [NFT_MINING_SYSTEM_SEED],
        bump
    )]
    pub nft_mining_system: Account<'info, NftMiningSystem>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub usdt_mint_account: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeSystem<'info> {
    pub fn process(
        &mut self,
        total_supply: u64,
        daily_output: u64,
        start_timestamp: u64,
        pool_address: Pubkey,
        market_pool_address: Pubkey,
        gdtc_mint: Pubkey,
        bio_mint: Pubkey,
        wsol_mint: Pubkey,
        admin: Pubkey,
        black_hole_address: Pubkey,

    ) -> Result<()> {

        let black_hole = Pubkey::from_str("11111111111111111111111111111111").unwrap();

        require!(black_hole_address == black_hole, NftStakingError::NotBlackHole);
        require!(self.nft_mining_system.is_initialized == false, NftStakingError::SystemAlreadyInitialized);
        // 设置系统参数
        self.nft_mining_system.authority = self.authority.key();
        self.nft_mining_system.is_initialized = true;
        self.nft_mining_system.usdt_mint = self.usdt_mint_account.key();
        self.nft_mining_system.gdtc_mint = gdtc_mint;
        self.nft_mining_system.bio_mint = bio_mint;
        self.nft_mining_system.wsol_mint = wsol_mint;
        self.nft_mining_system.admin = admin;
        self.nft_mining_system.pool_address = pool_address;
        self.nft_mining_system.market_pool_address = market_pool_address;
        self.nft_mining_system.black_hole_address = black_hole_address;
        self.nft_mining_system.total_supply = total_supply;
        self.nft_mining_system.daily_output = daily_output;
        self.nft_mining_system.start_timestamp = start_timestamp;
        self.nft_mining_system.order_info_index = 0;
        
        // 初始化质押池
        self.nft_mining_system.pool = StakingPool {
            reward_token_per_sec: daily_output / SECONDS_PER_DAY, // 每秒奖励 = 每日产出 / 86400
            accumulated_reward_per_share: 0,
            last_reward_timestamp: start_timestamp,
            total_shares: 0,
        };
        
    
       //打印所有数据
        msg!("NFT算力挖矿系统初始化数据:");
        msg!("管理员: {}", admin);
        msg!("全网分红池地址: {}", pool_address);
        msg!("市场分红地址: {}", market_pool_address);
        msg!("USDT代币地址: {}", self.usdt_mint_account.key());
        msg!("GDTC代币地址: {}", gdtc_mint);
        msg!("BIO代币地址: {}", bio_mint);
        msg!("WSOL代币地址: {}", wsol_mint);
        msg!("总供应量: {}", total_supply);
        msg!("每日产出: {}", daily_output);
        msg!("开始时间戳: {}", start_timestamp);
        msg!("NFT算力挖矿系统初始化成功!");
        
        Ok(())
    }
}
