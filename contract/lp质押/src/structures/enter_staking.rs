use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use super::errors::StakingError;
use super::*;
// use crate::generate_release_timestamps;
use crate::structures::tools::generate_release_timestamps;
use crate::structures::utils::update_reward_pool;
use crate::structures::utils::update_reward_debt;


#[derive(Accounts)]
pub struct EnterStaking<'info> {
    /// 全局质押状态，必须尚未初始化
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
        bump
    )]
    pub user_instance: Box<Account<'info, User>>,

    #[account(
        mut,
        constraint = user_lp_token_account.owner == authority.key() @ StakingError::InvalidLpTokenOwner,
        constraint = user_lp_token_account.mint == staking_instance.staking_token_mint @ StakingError::InvalidLpTokenMint,
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    /// 合约 LP Vault，接收用户 LP
    #[account(
        mut,
        constraint = gdtc_lp_in_account.owner == staking_instance.key() @ StakingError::InvalidVaultOwner,
        constraint = gdtc_lp_in_account.mint == staking_instance.staking_token_mint @ StakingError::InvalidLpTokenMint,
    )]
    pub gdtc_lp_in_account: Account<'info, TokenAccount>,

    /// 用户签名者
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> EnterStaking<'info> {
    pub fn process(
        &mut self,
        lp_staking_number: u64,
        stake_type: u64,
        staked_info_index: u64,
    ) -> Result<()> {
        let staking_instance = &mut self.staking_instance;
        let user_instance = &mut self.user_instance;
        let user_lp_token_account = &self.user_lp_token_account;
        let gdtc_lp_in_account = &self.gdtc_lp_in_account;

        let clock = Clock::get()?;

        // 检查余额、索引与质押池类型
        if user_lp_token_account.amount < lp_staking_number {
            return Err(StakingError::TokenAccountBalanceInsufficient.into());
        }
        //质押金额不能为0
        if lp_staking_number == 0 {
            return Err(StakingError::InvalidStakeAmount.into());
        }

        if staked_info_index > 9 {
            return Err(StakingError::InvalidStakedInfoIndex.into());
        }
        if stake_type > 2 {
            return Err(StakingError::InvalidStakeType.into());
        }

        // 判断当前质押周期是否结束
        let current_timestamp = clock.unix_timestamp as u64;
        let is_end = match stake_type {
            0 => current_timestamp > 2358810461,
            1 => current_timestamp > 2350861661,
            2 => current_timestamp > 2335136861,
            _ => true,
        };
        if is_end {
            return Err(StakingError::StakingEnded.into());
        }

        // 获取 staked_info
        let index = staked_info_index as usize;
        if user_instance.staked_info[index].is_staked {
            return Err(StakingError::UserAlreadyStaked.into());
        }
        if stake_type >= staking_instance.pools.len() as u64 {
            return Err(StakingError::InvalidStakeType.into());
        }

        // 计算质押结束时间
        let stake_end_time = generate_release_timestamps(current_timestamp, stake_type);

        // 更新用户状态
        user_instance.total_deposited_amount = user_instance
            .total_deposited_amount
            .checked_add(lp_staking_number)
            .ok_or(StakingError::Overflow)?;

        let staked_info = &mut user_instance.staked_info[index];
        staked_info.deposited_amount = staked_info
            .deposited_amount
            .checked_add(lp_staking_number)
            .ok_or(StakingError::Overflow)?;
        staked_info.stake_type = stake_type;
        staked_info.is_staked = true;
        staked_info.stake_start_time = current_timestamp;
        staked_info.stake_end_time = stake_end_time;




         // 更新奖励
        update_reward_pool(current_timestamp, staking_instance);
        update_reward_debt(staking_instance, user_instance, staked_info_index);

        // 更新质押池状态
        let pool = &mut staking_instance.pools[stake_type as usize];
        pool.total_shares = pool
            .total_shares
            .checked_add(lp_staking_number)
            .ok_or(StakingError::Overflow)?;

       
        // 转账 LP Token 到 Vault
        token::transfer(
            self.into_transfer_to_vault_context(),
            lp_staking_number,
        )?;

        Ok(())
    }

    fn into_transfer_to_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_lp_token_account.to_account_info(),
            to: self.gdtc_lp_in_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}