use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount,Transfer};

use crate::constants::*;

use super::errors::StakingError;
use crate::structures::utils::{update_reward_pool, update_reward_debt, store_pending_reward};

use super::*;

#[derive(Accounts)]
pub struct CancelStaking<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, //签名用户
      #[account(
        mut,
        seeds = [crate::constants::STAKING_SEED],
        bump,
        constraint = staking_instance.is_initialized @ StakingError::StakingInstanceNotInitialized
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    /// 用户信息
    #[account(
        mut,
        seeds = [b"user", authority.key().as_ref()],
        constraint = user_instance.isinit @ StakingError::UserNotInitialized,
        bump
    )]
    pub user_instance: Box<Account<'info, User>>,

    /// 用户 LP Token 接收账户
    #[account(
        mut,
        constraint = user_lp_token_account.mint == staking_instance.staking_token_mint @ StakingError::MintAccountIsNotMatch,
        constraint = user_lp_token_account.owner == authority.key() @ StakingError::InvalidLpTokenOwner
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    /// 合约 LP Token Vault（接收用户质押的 LP）
    #[account(
        mut,
        constraint = gdtc_lp_in_account.mint == staking_instance.staking_token_mint @ StakingError::MintAccountIsNotMatch,
        constraint = gdtc_lp_in_account.owner == staking_instance.key() @ StakingError::InvalidLpTokenOwner
    )]
    pub gdtc_lp_in_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> CancelStaking<'info> {
    pub fn process(&mut self, staked_info_index: u64,bump_seed:u8) -> Result<()> {
       
        let user_instance = &mut self.user_instance;

        let index = staked_info_index as usize;
        let amount = user_instance.staked_info[index].deposited_amount;

        // 检查是否质押
        if !user_instance.staked_info[index].is_staked {
            return Err(StakingError::NoStakingToCancel.into());
        }

        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;

        // 检查质押是否到期
        if current_timestamp < user_instance.staked_info[index].stake_end_time {
            return Err(StakingError::StakingNotMatured.into());
        }
        if !user_instance.staked_info[index].can_cancel_stake {
            return Err(StakingError::NeedCliamRewards.into());
        }


         let staking_instance = &mut self.staking_instance;
        // 更新奖励池并计算待领取奖励
        update_reward_pool(current_timestamp, staking_instance);
        store_pending_reward(staking_instance, user_instance, staked_info_index)?;



        // 更新质押池份额
        let pool = &mut staking_instance.pools[user_instance.staked_info[index].stake_type as usize];
        pool.total_shares = pool.total_shares.checked_sub(amount).ok_or(StakingError::Underflow)?;

        // 更新奖励债务
        // update_reward_debt(staking_instance, user_instance, staked_info_index);

        // 重置用户质押状态
        let staked_info = &mut user_instance.staked_info[index];
        staked_info.deposited_amount = 0;
        staked_info.accumulated_reward = 0;
        staked_info.is_staked = false;
        staked_info.stake_type = 0;
        staked_info.reward_debt = 0;
        staked_info.stake_start_time = 0;
        staked_info.stake_end_time = 0;
        staked_info.receivedReward = 0;
        staked_info.can_cancel_stake = false;



        // PDA 签名转账 LP Token 回给用户
        // let bump_seed = self.bump; // Anchor 自动提供 bumps
        let signer_seeds: &[&[&[u8]]] = &[&[crate::STAKING_SEED.as_ref(), &[bump_seed]]];

        let cpi_accounts = Transfer {
            from: self.gdtc_lp_in_account.to_account_info(),
            to: self.user_lp_token_account.to_account_info(),
            authority: self.staking_instance.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, amount)?;


        

        Ok(())
    }
}