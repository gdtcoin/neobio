use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use crate::errors::NftStakingError;
use crate::structures::{NftMiningSystem, OrderInfo};    


pub fn update_reward_pool(current_timestamp: u64, nft_mining_system: &mut NftMiningSystem) {
    // 遍历每个质押池
        let pool = &mut nft_mining_system.pool;
        // 如果没有份额，跳过此池
        if pool.total_shares == 0 {
            return;
        }
        // 计算时间差（当前时间戳 - 上次奖励时间戳）
        let time_diff = current_timestamp
            .checked_sub(pool.last_reward_timestamp)
            .unwrap_or(0);

        // 如果时间差为 0，跳过此池
        if time_diff == 0 {
            return;
        }

        // 计算池子的总奖励收入（奖励速率 * 时间差）(reward_token_per_sec as u128)
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

pub fn store_pending_reward(
    nft_mining_system: &mut NftMiningSystem,
    user_instance: &mut OrderInfo,
) -> Result<()> {


    msg!("user_instance.total_power: {}", user_instance.total_power);
    msg!("nft_mining_system.pool.accumulated_reward_per_share: {}", nft_mining_system.pool.accumulated_reward_per_share);
    msg!("user_instance.reward_debt: {}", user_instance.reward_debt);
    
    
    
    
    // 计算用户在该池子的待领取奖励
    let pending_reward = (user_instance.total_power as u128)
        .checked_mul(nft_mining_system.pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(crate::constants::COMPUTATION_DECIMALS as u128))
        .and_then(|v| v.checked_sub(user_instance.reward_debt as u128))
        .unwrap_or(0) as u64; // 最终将结果转换回 u64 类型，如果需要
                              // 如果待领取奖励为 0，直接返回
    if pending_reward == 0 {

        user_instance.reward_debt = (user_instance.total_power as u128)
        .checked_mul(nft_mining_system.pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(crate::constants::COMPUTATION_DECIMALS as u128))
        .unwrap_or(user_instance.reward_debt as u128) as u64;

        return Ok(());
    }
    msg!("pending_reward: {}", pending_reward);

    // 更新该质押池的累计奖励
    user_instance.accumulated_reward = user_instance
        .accumulated_reward
        .checked_add(pending_reward)
        .unwrap_or(user_instance.accumulated_reward); // 防止溢出

    // 更新用户的 reward_debt 为最新的池子状态
    user_instance.reward_debt = (user_instance.total_power as u128)
        .checked_mul(nft_mining_system.pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(crate::constants::COMPUTATION_DECIMALS as u128))
        .unwrap_or(user_instance.reward_debt as u128) as u64;
    Ok(())
}

pub fn update_reward_debt(
    nft_mining_system: &mut NftMiningSystem,
    user_instance: &mut OrderInfo,
) {

    // 确保该质押池已被质押
    // if !user_instance.is_staked {
    //     return; // 如果该质押池没有质押，直接返回
    // }
    if user_instance.total_power == 0 {
        user_instance.reward_debt = 0;
        return;
    }

    user_instance.reward_debt = (user_instance.total_power as u128)
        .checked_mul(nft_mining_system.pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(crate::constants::COMPUTATION_DECIMALS as u128))
        .unwrap_or(0) as u64;
}



