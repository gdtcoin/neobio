use anchor_lang::prelude::*;

use super::errors::NftStakingError;
use super::*;
use crate::constants::{NFT_MINING_SYSTEM_SEED, ORDER_INFO_SEED};
use super::utils::{update_reward_pool, store_pending_reward};

use anchor_spl::{
    token::{self, Burn, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct ClaimNft<'info> {

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

impl<'info> ClaimNft<'info> {
    pub fn process(&mut self, nft_mint_address: Pubkey) -> Result<()> {

        //更新用户nft状态

        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;
        let user = &mut self.order_info;
        user.is_nft_minted = true;
        user.nft_minted_time = current_timestamp;
        user.nft_mint_address = nft_mint_address;
       
        msg!("用户: {}", user.user_address);
        msg!("nft mint地址: {}", nft_mint_address);
        msg!("nft发放时间: {}", user.nft_minted_time);
        msg!("nft状态: {}", user.is_nft_minted);

        Ok(())
    }
}