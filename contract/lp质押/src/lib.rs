pub mod constants;
pub mod structures;

use anchor_spl::token::{self, Token, TokenAccount, Mint};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;

use constants::*;
use structures::{
    cancel_staking::*, claim_rewards::*, enter_staking::*, initialize_staking::*,
    initialize_user::*
};

declare_id!("FfTLXfiSaB72MRCJH2xtmuV4rXFQi24ubC85kq4LPi1h");

#[program]
pub mod Stake_Program {
    use super::*;

    pub fn initialize_staking(
        ctx: Context<InitializeStaking>,
        reward_per_sec_3_months: u64,
        reward_per_sec_6_months: u64,
        reward_per_sec_12_months: u64,
        start_reward_timestamp: u64,
        gdtc_pool_address: Pubkey,
    ) -> Result<()> {
        ctx.accounts.process(
            reward_per_sec_3_months,
            reward_per_sec_6_months,
            reward_per_sec_12_months,
            start_reward_timestamp,
            gdtc_pool_address
        )
    }

 pub fn initialize_user(ctx: Context<InitializeUser>,_user_superior_account:Pubkey) -> Result<()> {
        ctx.accounts.process(_user_superior_account)
    }


     pub fn enter_staking(
        ctx: Context<EnterStaking>,
        lp_staking_number: u64,
        stake_type: u64,
        staked_info_index: u64,
    ) -> Result<()> {
        ctx.accounts.process(lp_staking_number, stake_type, staked_info_index)
    }

    pub fn cancel_staking(ctx: Context<CancelStaking>, staked_info_index: u64) -> Result<()> {

        let bump_seed = ctx.bumps.staking_instance;
    ctx.accounts.process(staked_info_index,bump_seed)
}


    pub fn claim_rewards(ctx: Context<ClaimRewards>, staked_info_index: u64) -> Result<()> {

        let bump_seed = ctx.bumps.staking_instance;
    ctx.accounts.process(staked_info_index,bump_seed)
}

    // pub fn admin_claim_rewards(ctx: Context<AdminClaimRewards>, claim_number: u64) -> Result<()> {
    // ctx.accounts.process(claim_number)
    // }
}
