
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};

use crate::constants::*;
use super::errors::StakingError;

use super::*;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        mut,
        constraint = staking_instance.is_initialized @ StakingError::StakingInstanceAlreadyInitialized,
        seeds = [crate::constants::STAKING_SEED],
        bump
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    #[account(
        init,
        payer = authority,
        space = 8 + core::mem::size_of::<User>(),
        seeds = [b"user", authority.key().as_ref()],
        constraint = !user_instance.isinit @ StakingError::UserAlreadyStaked,
        bump
    )]
    pub user_instance: Account<'info, User>,


    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeUser<'info> {
    pub fn process(&mut self,user_superior_account: Pubkey) -> Result<()> {
        let user_instance = &mut self.user_instance;
        let staking_instance = &self.staking_instance;

        // 验证用户地址
        require!(
            self.authority.key() != Pubkey::default(),
            StakingError::InvalidUserAddress
        );


        // 设置用户基本信息
        user_instance.user_address = self.authority.key();
        user_instance.total_deposited_amount = 0;
        user_instance.user_superior_account= user_superior_account;
        user_instance.isinit = true;
        
    

        // 初始化 10 个质押槽位
        for staked in user_instance.staked_info.iter_mut() {
            staked.deposited_amount = 0;
            staked.reward_debt = 0;
            staked.accumulated_reward = 0;
            staked.is_staked = false;
            staked.stake_type = 0;
            staked.stake_start_time = 0;
            staked.stake_end_time = 0;
            staked.receivedReward = 0;
            staked.can_cancel_stake = false;
        }

        Ok(())
    }
}