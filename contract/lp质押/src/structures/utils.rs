
use anchor_spl::token::{self, Token, TokenAccount, Mint};

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
// use anchor_spl::token;
use crate::constants::*;
use super::errors::StakingError;


use super::*;

// 工具函数
pub fn calculate_stake_duration(stake_type: u64) -> Result<u64> {
    match stake_type {
        0 => Ok(90 * 24 * 60 * 60), // 3个月
        1 => Ok(180 * 24 * 60 * 60), // 6个月
        2 => Ok(365 * 24 * 60 * 60), // 12个月
        _ => Err(StakingError::InvalidStakeType.into()),
    }
}

pub fn validate_stake_type(stake_type: u64) -> Result<()> {
    require!(stake_type < 3, StakingError::InvalidStakeType);
    Ok(())
}

pub fn validate_staked_info_index(index: u64) -> Result<()> {
    require!(index < 10, StakingError::InvalidStakedInfoIndex);
    Ok(())
}

pub fn calculate_reward_rate(base_rate: u64, time_factor: u64) -> Result<u64> {
    base_rate.checked_mul(time_factor)
        .ok_or(StakingError::InvalidRewardCalculation.into())
}

pub fn validate_time_parameters(start_time: u64, end_time: u64) -> Result<()> {
    require!(start_time > 0, StakingError::InvalidTimeParameters);
    require!(end_time > start_time, StakingError::InvalidTimeParameters);
    Ok(())
}

pub fn update_reward_pool(current_timestamp: u64, staking_instance: &mut StakingInstance) {
    // 遍历每个质押池
    for pool in staking_instance.pools.iter_mut() {
        // 如果没有份额，跳过此池
        if pool.total_shares == 0 {
            continue;
        }
        // 计算时间差（当前时间戳 - 上次奖励时间戳）
        let time_diff = current_timestamp
            .checked_sub(pool.last_reward_timestamp)
            .unwrap_or(0);

        // 如果时间差为 0，跳过此池
        if time_diff == 0 {
            continue;
        }

        // 计算池子的总奖励收入（奖励速率 * 时间差）
        let income = pool
            .reward_token_per_sec
            .checked_mul(time_diff)
            .unwrap_or(0);

        // 更新 `accumulated_reward_per_share`
        if pool.total_shares > 0 {
            // 每份奖励计算
            let reward_per_share = (income as u128)
                .checked_mul(crate::constants::COMPUTATION_DECIMALS as u128) // 精度调整
                .unwrap_or(0)
                .checked_div(pool.total_shares as u128) // 每份奖励
                .unwrap_or(0) as u64;

            // 累加每份奖励的累计值
            pool.accumulated_reward_per_share = pool
                .accumulated_reward_per_share
                .checked_add(reward_per_share)
                .unwrap_or(pool.accumulated_reward_per_share); // 防止溢出
        }

        // 更新最后奖励时间戳为当前时间戳
        pool.last_reward_timestamp = current_timestamp;
    }
}

pub fn store_pending_reward(
    staking_instance: &mut StakingInstance,
    user_instance: &mut User,
    staked_info_number: u64, // 修改为索引
) -> Result<()> {
    // 获取用户对应的质押信息
    let staked_info = &mut user_instance.staked_info[staked_info_number as usize];

    // 确保该质押池已被质押
    if !staked_info.is_staked {
        return Ok(()); // 如果该质押池没有质押，直接返回
    }

    // 获取质押类型对应的池子
    let stake_type = staked_info.stake_type as usize;

    // 检查 stake_type 是否为有效池子索引
    // if stake_type >= staking_instance.pools.len() {
    //     return Err(ErrorCode::InvalidStakeType.into()); // 自定义错误类型
    // }

    // 获取对应池子
    let pool = &staking_instance.pools[stake_type];

    // 计算用户在该池子的待领取奖励
    let pending_reward = (staked_info.deposited_amount as u128)
        .checked_mul(pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(crate::constants::COMPUTATION_DECIMALS as u128))
        .and_then(|v| v.checked_sub(staked_info.reward_debt as u128))
        .unwrap_or(0) as u64; // 最终将结果转换回 u64 类型，如果需要
                              // 如果待领取奖励为 0，直接返回
    if pending_reward == 0 {
        return Ok(());
    }

    // 更新该质押池的累计奖励
    staked_info.accumulated_reward = staked_info
        .accumulated_reward
        .checked_add(pending_reward)
        .unwrap_or(staked_info.accumulated_reward); // 防止溢出

    // 更新用户的 reward_debt 为最新的池子状态
    staked_info.reward_debt = (staked_info.deposited_amount as u128)
        .checked_mul(pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(crate::constants::COMPUTATION_DECIMALS as u128))
        .unwrap_or(staked_info.reward_debt as u128) as u64;
    Ok(())
}

pub fn update_reward_debt(
    staking_instance: &mut StakingInstance,
    user_instance: &mut User,
    staked_info_number: u64, // 用户质押池的索引
) {
    // 获取用户对应的质押信息
    let staked_info = &mut user_instance.staked_info[staked_info_number as usize];

    // 确保该质押池已被质押
    if !staked_info.is_staked {
        return; // 如果该质押池没有质押，直接返回
    }
    // 获取质押类型对应的池子
    let stake_type = staked_info.stake_type as usize;
    // 检查 stake_type 是否为有效池子索引
    if stake_type >= staking_instance.pools.len() {
        return; // 无效的池子索引，直接返回
    }

    // 获取对应池子
    let pool = &staking_instance.pools[stake_type];

    // msg!(
    //     "Hello world!!",
    //     pool.accumulated_reward_per_share,
    //     staked_info.deposited_amount
    // );
    // 更新该质押池的 reward_debt
    // msg!(
    //     "staked_info
    //     .deposited_amount",
    //     staked_info.deposited_amount
    // );
    // msg!("accumulated_reward_per_share", accumulated_reward_per_share);

    staked_info.reward_debt = (staked_info.deposited_amount as u128)
        .checked_mul(pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(crate::constants::COMPUTATION_DECIMALS as u128))
        .unwrap_or(0) as u64;
}

pub fn is_authorized(user: &Pubkey, authority: &Pubkey) -> bool {
    user == authority
}
pub fn can_unstake(staked: &Staked, current_timestamp: u64) -> bool {
    staked.is_staked && staked.stake_end_time <= current_timestamp
}

pub fn calculate_referral_reward(user: &User, amount: u64) -> u64 {
    // 计算推荐奖励，假设为10%
    let referral_reward = amount * 10 / 100;
    referral_reward
}
