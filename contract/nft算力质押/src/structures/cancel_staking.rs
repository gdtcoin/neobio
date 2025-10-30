use anchor_lang::prelude::*;

use super::errors::NftStakingError;
use super::*;
use crate::constants::{NFT_MINING_SYSTEM_SEED, ORDER_INFO_SEED};
use super::utils::{update_reward_debt, update_reward_pool, store_pending_reward};

#[derive(Accounts)]
pub struct CancelStaking<'info> {
    /// 管理员签名者
    #[account(mut,
        constraint = admin.key() == nft_mining_system.admin @ NftStakingError::Unauthorized,
    )]
    pub admin: Signer<'info>,
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
        constraint = order_info.is_staked == true @ NftStakingError::UserNotInitialized,
        constraint = order_info.user_address == user_address.key() @ NftStakingError::Unauthorized,
    )]
    pub order_info: Account<'info, OrderInfo>,

    /// 用户地址（用于验证）
    /// CHECK: 这是用户的公钥地址
    pub user_address: UncheckedAccount<'info>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

impl<'info> CancelStaking<'info> {
    pub fn process(&mut self, reduce_amount: u64) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;

        // 验证用户是否已经质押
        require!(
            self.order_info.is_staked, 
            NftStakingError::UserNotInitialized
        );

        // 验证减少的质押数量是否合理
        require!(
            reduce_amount > 0, 
            NftStakingError::InvalidAmount
        );

        require!(
            reduce_amount <= self.order_info.total_power, 
            NftStakingError::InsufficientBalance
        );

        // 验证用户地址是否匹配
        require!(
            self.order_info.user_address == self.user_address.key(), 
            NftStakingError::UserAccountIsNotMatch
        );
        
        update_reward_pool(current_timestamp, &mut self.nft_mining_system);
        store_pending_reward(&mut self.nft_mining_system, &mut self.order_info)?;

        // 计算新的质押权重
        let new_staking_weight = self.order_info.total_power
            .checked_sub(reduce_amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // 更新用户状态
        let user = &mut self.order_info;
        user.total_power = new_staking_weight;

        // 如果质押权重为0，则完全取消质押
        // if new_staking_weight == 0 {
        //     // user.is_staked = false;
        //     // user.stake_start_time = 0;
        //     // user.reward_debt = 0;
        // }

        user.stake_start_time = current_timestamp;


        {
            // 更新系统质押池状态
            let staking_pool = &mut self.nft_mining_system.pool;
            
            // 安全检查：确保质押池总份额不会变成负数
            require!(
                staking_pool.total_shares >= reduce_amount, 
                NftStakingError::InsufficientBalance
            );
            
            staking_pool.total_shares = staking_pool
                .total_shares
                .checked_sub(reduce_amount)
                .ok_or(ProgramError::ArithmeticOverflow)?;
        }


        update_reward_debt(&mut self.nft_mining_system, user);
    
        // 记录操作日志
        msg!("管理员: {}", self.admin.key());
        msg!("用户: {}", self.user_address.key());
        msg!("减少质押数量: {}", reduce_amount);
        msg!("用户原质押权重: {}", user.total_power + reduce_amount);
        msg!("用户新质押权重: {}", new_staking_weight);
        msg!("质押池原总份额: {}", self.nft_mining_system.pool.total_shares + reduce_amount);
        msg!("操作时间: {}", current_timestamp);
        if new_staking_weight == 0 {
            msg!("用户质押已完全取消");
            msg!("用户质押状态: 已取消");
        } else {
            msg!("用户质押状态: 部分减少");
            msg!("用户新奖励债务: {}", user.reward_debt);
        }

        Ok(())
    }
}