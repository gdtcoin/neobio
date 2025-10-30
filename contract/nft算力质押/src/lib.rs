pub mod constants;
pub mod structures;



use anchor_lang::prelude::*;

use structures::*;


use structures::{
    initialize_system::*,
    usdt_wsol::*,
    wsol_gdtc::*,
    gdtc_bio::*,
    enter_staking::*,
    cancel_staking::*,
    claim_rewards::*,
    add_staking::*,
    claim_nft::*
};

declare_id!("Cyc7r9MqrmNECxDhs25cmjWdY6kXxWtUZmezBFCfaJkb");

#[program]
pub mod NftMiningProgram {
    use super::*;

    /// 初始化NFT算力挖矿系统
    pub fn initialize_system(
        ctx: Context<InitializeSystem>,
        total_supply: u64,
        daily_output: u64,
        start_timestamp: u64,
        pool_address: Pubkey,
        market_pool_address: Pubkey,
        gdtc_mint: Pubkey,
        bio_mint: Pubkey,
        wsol_mint: Pubkey,
        admin: Pubkey,
        black_hole_address: Pubkey,
    ) -> Result<()> {
        ctx.accounts.process(
            total_supply,
            daily_output,
            start_timestamp,
            pool_address,
            market_pool_address,
            gdtc_mint,
            bio_mint,
            wsol_mint,
            admin,
            black_hole_address
        )
    }

    pub fn usdt_wsol(
        ctx: Context<UsdtWsol>,
        usdt_amount: u64,
    ) -> Result<()> {
        ctx.accounts.process(usdt_amount)
    }

    /// 使用WSOL购买GDTC并销毁一半
    pub fn wsol_gdtc(
        ctx: Context<WsolGdtc>
    ) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn gdtc_to_bio(
        ctx: Context<GdtcToBio>,
    ) -> Result<()> {

        ctx.accounts.process()
    }

    pub fn enter_staking(
        ctx: Context<EnterStaking>,
    ) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn cancel_staking(
        ctx: Context<CancelStaking>,
        reduce_amount: u64,
    ) -> Result<()> {
        ctx.accounts.process(reduce_amount)
    }

    pub fn claim_rewards(
        ctx: Context<ClaimRewards>
    ) -> Result<()> {

        let bump_seed = ctx.bumps.nft_mining_system;
        ctx.accounts.process(bump_seed)
    }

    //add_stake
    pub fn add_stake(
        ctx: Context<AddStaking>,
        reduce_amount: u64,
        gdtc_amount: u64,
    ) -> Result<()> {
        ctx.accounts.process(reduce_amount,gdtc_amount)
    }

    pub fn claim_nft(
        ctx: Context<ClaimNft>,
        nft_mint_address: Pubkey,
    ) -> Result<()> {
        ctx.accounts.process(nft_mint_address)
    }

}
