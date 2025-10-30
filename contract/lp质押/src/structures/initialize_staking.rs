use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

use crate::constants::*;

use super::*;
use super::errors::StakingError;


#[derive(Accounts)]
pub struct InitializeStaking<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + core::mem::size_of::<StakingInstance>() + 3 * (8 + core::mem::size_of::<StakingPool>()),
        constraint = !staking_instance.is_initialized @ StakingError::StakingInstanceAlreadyInitialized,
        seeds = [crate::constants::STAKING_SEED],
        bump
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    #[account(
        constraint = reward_token_mint.decimals > 0 @ StakingError::InvalidConfiguration,
    )]
    pub reward_token_mint: Account<'info, Mint>,

    #[account(
        constraint = staking_token_mint.decimals > 0 @ StakingError::InvalidConfiguration,
    )]
    pub staking_token_mint: Account<'info, Mint>,

    #[account(
        constraint = gdtc_token_mint.decimals > 0 @ StakingError::InvalidConfiguration,
    )]
    pub gdtc_token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>, // 管理员

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeStaking<'info> {
    pub fn process(
        &mut self,
        reward_per_sec_3_months: u64,
        reward_per_sec_6_months: u64,
        reward_per_sec_12_months: u64,
        start_reward_timestamp: u64,
        gdtc_pool_address: Pubkey,
    ) -> Result<()> {
        let staking_instance = &mut self.staking_instance;

        // 设置基础字段
        staking_instance.authority = self.authority.key();
        staking_instance.reward_token_mint = self.reward_token_mint.key();
        staking_instance.staking_token_mint = self.staking_token_mint.key();
        staking_instance.secend_reward_token_mint = self.gdtc_token_mint.key();
        staking_instance.is_initialized = true;
        staking_instance.gdtc_pool_address = gdtc_pool_address;

        // 初始化 3 个质押池
        staking_instance.pools = [
            StakingPool {
                stake_type: 0, // 3 个月
                reward_token_per_sec: reward_per_sec_3_months,
                accumulated_reward_per_share: 0,
                last_reward_timestamp: start_reward_timestamp,
                total_shares: 0,
            },
            StakingPool {
                stake_type: 1, // 6 个月
                reward_token_per_sec: reward_per_sec_6_months,
                accumulated_reward_per_share: 0,
                last_reward_timestamp: start_reward_timestamp,
                total_shares: 0,
            },
            StakingPool {
                stake_type: 2, // 12 个月
                reward_token_per_sec: reward_per_sec_12_months,
                accumulated_reward_per_share: 0,
                last_reward_timestamp: start_reward_timestamp,
                total_shares: 0,
            },
        ];
        Ok(())
    }

    /// 验证初始化参数
    fn validate_initialization_parameters(
        &self,
        reward_per_sec_3_months: u64,
        reward_per_sec_6_months: u64,
        reward_per_sec_12_months: u64,
        start_reward_timestamp: u64,
    ) -> Result<()> {
        // 验证奖励率
        require!(reward_per_sec_3_months > 0, StakingError::InvalidRewardRate);
        require!(reward_per_sec_6_months > 0, StakingError::InvalidRewardRate);
        require!(reward_per_sec_12_months > 0, StakingError::InvalidRewardRate);

        // 验证时间戳
        require!(start_reward_timestamp > 0, StakingError::InvalidTimeParameters);

        // 验证奖励率合理性（6个月池奖励率应该高于3个月池）
        require!(
            reward_per_sec_6_months >= reward_per_sec_3_months,
            StakingError::InvalidRewardRate
        );

        // 验证奖励率合理性（12个月池奖励率应该高于6个月池）
        require!(
            reward_per_sec_12_months >= reward_per_sec_6_months,
            StakingError::InvalidRewardRate
        );

        // 验证代币Mint
        require!(
            self.reward_token_mint.key() != self.staking_token_mint.key(),
            StakingError::InvalidConfiguration
        );
        require!(
            self.reward_token_mint.key() != self.gdtc_token_mint.key(),
            StakingError::InvalidConfiguration
        );
        require!(
            self.staking_token_mint.key() != self.gdtc_token_mint.key(),
            StakingError::InvalidConfiguration
        );

        Ok(())
    }
}