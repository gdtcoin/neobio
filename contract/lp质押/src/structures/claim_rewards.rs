use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Token, TokenAccount};
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer, Burn};

use std::str::FromStr;

use super::*;
use crate::structures::utils::{update_reward_pool, store_pending_reward};
use super::errors::StakingError;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, //签名用户
      #[account(
        mut,
        seeds = [crate::constants::STAKING_SEED],
        bump,
        constraint = staking_instance.is_initialized @ StakingError::StakingInstanceAlreadyInitialized
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    /// 用户信息
    #[account(
        mut,
        seeds = [b"user", authority.key().as_ref()],
        constraint = user_instance.isinit @ StakingError::UserAlreadyStaked,
        constraint = user_instance.user_address == authority.key() @ StakingError::UserAccountIsNotMatch,
        bump
    )]
    pub user_instance: Box<Account<'info, User>>,


      /// 上级的 GDTC Token 账户（用于转发或发奖励）
    #[account(
        mut,
        constraint = user_super_gdtc_token_account.mint == staking_instance.reward_token_mint @ StakingError::MintAccountIsNotMatch,
        constraint = user_super_gdtc_token_account.owner == user_instance.user_superior_account.key() @ StakingError::InvalidVaultOwner
    )]
    pub user_super_gdtc_token_account: Account<'info, TokenAccount>,

    /// 用户的 GDTC Token 账户（用于接收奖励）
    #[account(
        mut,
        constraint = user_gdtc_token_account.mint == staking_instance.reward_token_mint @ StakingError::MintAccountIsNotMatch,
        constraint = user_gdtc_token_account.owner == authority.key() @ StakingError::InvalidVaultOwner
    )]
    pub user_gdtc_token_account: Account<'info, TokenAccount>,

    //全网分红池 token接收地址
    #[account(mut,
        constraint = user_global_pool_token_account.mint == staking_instance.reward_token_mint @ StakingError::MintAccountIsNotMatch,
        constraint = user_global_pool_token_account.owner == staking_instance.gdtc_pool_address @ StakingError::InvalidVaultOwner
    )]
    pub user_global_pool_token_account: Account<'info, TokenAccount>,



    //黑洞地址tokenaccount
    #[account(
        mut,
        constraint = black_hole_bio_account.mint == staking_instance.reward_token_mint @ StakingError::MintAccountIsNotMatch,
    )]
    pub black_hole_bio_account: Account<'info, TokenAccount>,



    //bio 代币mint地址
    #[account(
        mut,
        constraint = bio_mint_account.key() == staking_instance.reward_token_mint @ StakingError::MintAccountIsNotMatch,
    )]
    pub bio_mint_account: Account<'info, Mint>,

    /// 合约用于发放奖励的 GDTC 账户（Vault）
    #[account(
        mut,
        constraint = gdtc_reward_out_account.mint == staking_instance.reward_token_mint @ StakingError::MintAccountIsNotMatch,
        constraint = gdtc_reward_out_account.owner == staking_instance.key() @ StakingError::InvalidVaultOwner
    )]
    pub gdtc_reward_out_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>, //系统账户 programid
    pub token_program: Program<'info, Token>,   //token账户 可从sdk里导入
}

impl<'info> ClaimRewards<'info> {

    pub fn process(&mut self, staked_info_index: u64,bump_seed:u8) -> Result<()> {

        msg!("start claim rewards");
        msg!("staked_info_index: {:?}", staked_info_index);


        // 获取当前时间戳
        let staking_instance = &mut self.staking_instance;

        let user_instance = &mut self.user_instance;


        let gdtc_reward_out_account = &self.gdtc_reward_out_account;



        // 时间戳和用户校验
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;
        let index = staked_info_index as usize;


        msg!("user_instance.staked_info[index].is_staked: {:?}", user_instance.staked_info[index].is_staked);
        msg!("user_instance.staked_info[index].accumulated_reward: {:?}", user_instance.staked_info[index].accumulated_reward);
        msg!("user_instance.staked_info[index].receivedReward: {:?}", user_instance.staked_info[index].receivedReward);
        msg!("user_instance.staked_info[index].can_cancel_stake: {:?}", user_instance.staked_info[index].can_cancel_stake);
        msg!("user_instance.staked_info[index].deposited_amount: {:?}", user_instance.staked_info[index].deposited_amount);
        msg!("user_instance.staked_info[index].reward_debt: {:?}", user_instance.staked_info[index].reward_debt);
        msg!("user_instance.staked_info[index].stake_end_time: {:?}", user_instance.staked_info[index].stake_end_time);
        msg!("user_instance.staked_info[index].stake_start_time: {:?}", user_instance.staked_info[index].stake_start_time);
        msg!("user_instance.staked_info[index].stake_type: {:?}", user_instance.staked_info[index].stake_type);
        msg!("user_instance.staked_info[index].stake_start_time: {:?}", user_instance.staked_info[index].stake_start_time);
        
        
        if !user_instance.staked_info[index].is_staked {
            return Err(StakingError::NoStakingToClaimRewards.into());
        }

        // 更新奖励池并计算用户奖励
        update_reward_pool(current_timestamp, staking_instance);
        store_pending_reward(staking_instance, user_instance, staked_info_index)?;

        let mut accumulated_reward = user_instance.staked_info[index].accumulated_reward;
        
        // accumulated_reward = 10000;
        if accumulated_reward == 0 {
            return Err(StakingError::NoRewardsToClaim.into());
        }

        //大于10000个就报错
        let bio_decimals = self.bio_mint_account.decimals;
        let max_claim_number = 10000 * 10_u64.pow(bio_decimals as u32);

        if accumulated_reward > max_claim_number{
            return   Err(StakingError::MaxRewardsToClaim.into());
        }

        // 检查奖励账户余额
        if gdtc_reward_out_account.amount < accumulated_reward {
            if current_timestamp >= user_instance.staked_info[index].stake_end_time
                && user_instance.user_address == self.authority.key()
            {
                if !user_instance.staked_info[index].can_cancel_stake {
                    user_instance.staked_info[index].can_cancel_stake = true;
                    user_instance.total_deposited_amount = user_instance
                        .total_deposited_amount
                        .checked_sub(user_instance.staked_info[index].deposited_amount)
                        .ok_or(StakingError::Overflow)?;
                }
            }
            return Ok(());
        }

        {
        let staking_instance = &mut self.staking_instance;
        let signer_seeds: &[&[&[u8]]] = &[&[crate::STAKING_SEED.as_ref(), &[bump_seed]]];

        // 给上级分红（如果满足条件）
        // if super_instance.total_deposited_amount > 2_000_000_000 {
        let cpi_accounts = Transfer {
            from: self.gdtc_reward_out_account.to_account_info(),
            to: self.user_super_gdtc_token_account.to_account_info(),
            authority: staking_instance.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, accumulated_reward * 5 / 100)?;
        // }

        //给全网分红池分红（如果满足条件）
        if staking_instance.gdtc_pool_address.key() == self.user_global_pool_token_account.owner {
            let cpi_accounts = Transfer {
                from: self.gdtc_reward_out_account.to_account_info(),
                to: self.user_global_pool_token_account.to_account_info(),
                authority: staking_instance.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
            token::transfer(cpi_ctx, accumulated_reward *5/ 100)?;
        }



        // 给黑洞地址转账10%
      
        let black_hole = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        if  black_hole== self.black_hole_bio_account.owner {
            let cpi_accounts = Transfer {
                from: self.gdtc_reward_out_account.to_account_info(),
                to: self.black_hole_bio_account.to_account_info(),
                authority: staking_instance.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
            token::transfer(cpi_ctx, accumulated_reward *10/ 100)?;
        }


        //给上级再转10额外分红
        let cpi_accounts = Transfer {
            from: self.gdtc_reward_out_account.to_account_info(),
            to: self.user_super_gdtc_token_account.to_account_info(),
            authority: staking_instance.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, accumulated_reward * 10 / 100)?;


        
        //给用户发放80%
        let cpi_accounts = Transfer {
            from: self.gdtc_reward_out_account.to_account_info(),
            to: self.user_gdtc_token_account.to_account_info(),
            authority: staking_instance.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, accumulated_reward * 80 / 100 )?;
    }




        // 更新质押状态
        if current_timestamp >= user_instance.staked_info[index].stake_end_time {
            if user_instance.staked_info[index].can_cancel_stake {
                return Err(StakingError::NoRewardsToClaim.into());
            }
        
            user_instance.staked_info[index].can_cancel_stake = true;
            user_instance.total_deposited_amount = user_instance
                .total_deposited_amount
                .checked_sub(user_instance.staked_info[index].deposited_amount)
                .ok_or(StakingError::Overflow)?;
        }

        //打印用户信息
        msg!("用户信息: {:?}", user_instance.user_address);
        msg!("奖励信息: {:?}", accumulated_reward);
        msg!("奖励时间: {:?}", current_timestamp);
        msg!("奖励结束时间: {:?}", user_instance.staked_info[index].stake_end_time);
        msg!("奖励是否可取消: {:?}", user_instance.staked_info[index].can_cancel_stake);
        // 重置累计奖励并更新已领取记录
        user_instance.staked_info[index].accumulated_reward = 0;
        user_instance.staked_info[index].receivedReward = user_instance.staked_info[index]
            .receivedReward
            .checked_add(accumulated_reward)
            .ok_or(StakingError::Overflow)?;
        
        msg!("已领取奖励: {:?}", user_instance.staked_info[index].receivedReward);
        msg!("已质押信息: {:?}", user_instance.staked_info[index]);
        Ok(())
    }
}