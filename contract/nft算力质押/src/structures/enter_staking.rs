use anchor_lang::prelude::*;

use super::errors::NftStakingError;
use super::*;
use crate::constants::{NFT_MINING_SYSTEM_SEED, ORDER_INFO_SEED};
use super::utils::{update_reward_debt, update_reward_pool};

#[derive(Accounts)]
pub struct EnterStaking<'info> {
    /// 用户签名者
    #[account(mut)]
    pub user: Signer<'info>,

    /// 系统 PDA（已初始化）
    #[account(
        mut,
        seeds = [NFT_MINING_SYSTEM_SEED],
        bump,
        constraint = nft_mining_system.is_initialized @ NftStakingError::Unauthorized,
    )]
    pub nft_mining_system: Account<'info, NftMiningSystem>,

    /// 用户状态账户
    #[account(
        mut,
        seeds = [ORDER_INFO_SEED, &order_info.order_info_index.to_le_bytes()],
        bump,
        constraint = order_info.user_address == user.key() @ NftStakingError::Unauthorized,
    )]
    pub order_info: Account<'info, OrderInfo>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

impl<'info> EnterStaking<'info> {
    pub fn process(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;

        // 验证系统是否已经开始
        require!(
            current_timestamp >= self.nft_mining_system.start_timestamp, 
            NftStakingError::Unauthorized
        );

        // 验证用户状态
        require!(
            self.order_info.is_transfer_usdt, 
            NftStakingError::Unauthorized
        );

        require!(
            !self.order_info.is_staked, 
            NftStakingError::UserAlreadyStaked
        );

        require!(
            self.order_info.investment_amount > 0, 
            NftStakingError::InsufficientBalance
        );

        // 验证用户是否已经获得GDTC
        require!(
            self.order_info.gdtc_amount > 0, 
            NftStakingError::InsufficientBalance
        );

        // 计算质押权重（基于投资金额）
        let staking_weight = self.order_info.investment_amount;

        // 更新用户状态
        let user = &mut self.order_info;
        user.is_staked = true;
        user.stake_start_time = current_timestamp;
        user.total_power = staking_weight;



        {
            // 初始化奖励债务
            let staking_pool = &mut self.nft_mining_system.pool;
            // 更新系统质押池状态
            staking_pool.total_shares = staking_pool
            .total_shares
            .checked_add(staking_weight)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        }
        

        update_reward_pool(current_timestamp, &mut self.nft_mining_system);
        update_reward_debt(&mut self.nft_mining_system, user);
       
       
        // 获取最终状态用于日志记录
        let final_total_shares = self.nft_mining_system.pool.total_shares;
        let final_total_power = user.total_power;
        let final_reward_debt = user.reward_debt;

        // 记录操作日志
        msg!("用户: {}", self.user.key());
        msg!("投资金额: {} lamports", self.order_info.investment_amount);
        msg!("质押权重: {}", staking_weight);
        msg!("质押开始时间: {}", current_timestamp);
        msg!("质押池总份额: {}", final_total_shares);
        msg!("用户总算力: {}", final_total_power);
        msg!("奖励债务: {}", final_reward_debt);

        Ok(())
    }
}