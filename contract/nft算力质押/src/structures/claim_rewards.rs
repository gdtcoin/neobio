use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

use super::errors::NftStakingError;
use super::*;
use crate::constants::{NFT_MINING_SYSTEM_SEED, ORDER_INFO_SEED, REWARD_CLAIM_COOLDOWN};
use super::utils::{update_reward_pool, store_pending_reward,update_reward_debt};

use std::str::FromStr;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    /// 用户签名者
    #[account(mut)]
    pub user: Signer<'info>,

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
        constraint = order_info.user_address == user.key() @ NftStakingError::Unauthorized,
    )]
    pub order_info: Account<'info, OrderInfo>,



    /// 用户的 BIO Token 账户（用于接收奖励）
    #[account(
        mut,
        constraint = user_bio_account.mint == nft_mining_system.bio_mint @ NftStakingError::InvalidTokenMint,
        constraint = user_bio_account.owner == user.key() @ NftStakingError::Unauthorized,
    )]
    pub user_bio_account: Box<Account<'info, TokenAccount>>,

    /// 全网分红池的 BIO Token 账户
    #[account(
        mut,
        constraint = pool_bio_account.mint == nft_mining_system.bio_mint @ NftStakingError::InvalidTokenMint,
        constraint = pool_bio_account.owner == nft_mining_system.pool_address @ NftStakingError::Unauthorized,
    )]
    pub pool_bio_account: Box<Account<'info, TokenAccount>>,

    /// 系统 BIO 奖励账户（Vault）
    #[account(
        mut,
        constraint = system_bio_account.mint == nft_mining_system.bio_mint @ NftStakingError::InvalidTokenMint,
        constraint = system_bio_account.owner == nft_mining_system.key() @ NftStakingError::Unauthorized,
    )]
    pub system_bio_account: Box<Account<'info, TokenAccount>>,


    #[account(mut,
        constraint = user_superior_token_account.mint == nft_mining_system.bio_mint @ NftStakingError::TokenMintMismatch,
        constraint = user_superior_token_account.owner == order_info.user_superior_account @ NftStakingError::Unauthorized,
    )]
    pub user_superior_token_account: Account<'info, TokenAccount>,

    /// 黑洞地址 BIO Token 账户
    #[account(
        mut,
        constraint = black_hole_bio_account.mint == nft_mining_system.bio_mint @ NftStakingError::InvalidTokenMint,
        constraint = black_hole_bio_account.owner == nft_mining_system.black_hole_address @ NftStakingError::Unauthorized,
    )]
    pub black_hole_bio_account: Box<Account<'info, TokenAccount>>,

    /// BIO Mint 账户
    #[account(
        constraint = bio_mint.key() == nft_mining_system.bio_mint @ NftStakingError::InvalidTokenMint,
    )]
    pub bio_mint: Box<Account<'info, Mint>>,

    /// 系统程序
    pub system_program: Program<'info, System>,
    
    /// Token 程序
    pub token_program: Program<'info, Token>,
}

impl<'info> ClaimRewards<'info> {
    pub fn process(&mut self, bump_seed: u8) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;

        msg!("开始领取 BIO 质押奖励");
        msg!("用户: {}", self.user.key());

        // 验证用户是否有质押权重
        // require!(
        //     self.order_info.total_power > 0, 
        //     NftStakingError::InsufficientBalance
        // );

        // 验证用户是否已经完成GDTC购买流程
        require!(
            self.order_info.burn_gdtc, 
            NftStakingError::Unauthorized
        );

        // 验证用户是否已经完成BIO购买流程
        require!(
            self.order_info.burn_bio, 
            NftStakingError::Unauthorized
        );

        // 验证用户是否已经质押
        require!(
            self.order_info.is_staked, 
            NftStakingError::Unauthorized
        );
        // 检查奖励领取冷却时间
        let time_since_last_claim = current_timestamp
            .checked_sub(self.order_info.last_claim_timestamp)
            .unwrap_or(REWARD_CLAIM_COOLDOWN + 1);

        require!(
            time_since_last_claim >= REWARD_CLAIM_COOLDOWN, 
            NftStakingError::Unauthorized
        );

        update_reward_pool(current_timestamp, &mut self.nft_mining_system);
        store_pending_reward(&mut self.nft_mining_system, &mut self.order_info)?;


         // 计算用户应得 BIO 奖励
         let mut reward_amount = self.order_info.accumulated_reward;
       

       
        let bio_decimals = self.bio_mint.decimals;
        let max_claim_number = 1000 * 10_u64.pow(bio_decimals as u32);

        if reward_amount > max_claim_number{
            return   Err(NftStakingError::MaxRewardsToClaim.into());
        }

        //  if reward_amount == 0 {
        //     //  return Err(NftStakingError::NoRewardsToClaim.into());
        //     reward_amount = 1000000
        //     //  
        //  }

         require!(
            reward_amount > 0, 
             NftStakingError::InsufficientBalance
         );
 
         msg!("reward_amount: {}", reward_amount);
         msg!("self.system_bio_account.amount: {}", self.system_bio_account.amount);
         // 检查系统奖励账户余额
         require!(
             self.system_bio_account.amount >= reward_amount,    
             NftStakingError::InsufficientBalance
         );

        // 获取系统 PDA 的签名种子
        let signer_seeds: &[&[&[u8]]] = &[&[
            NFT_MINING_SYSTEM_SEED,
            &[bump_seed],
        ]];

        // 计算推荐奖励（5%）
        let referral_reward = reward_amount * 5 / 100;
        let user_reward = reward_amount - (referral_reward * 4);


        // 给全网分红池发放推荐奖励
        let cpi_accounts = Transfer {
            from: self.system_bio_account.to_account_info(),
            to: self.pool_bio_account.to_account_info(),
            authority: self.nft_mining_system.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, referral_reward)?;




        // 给用户上级发放推荐奖励
        let cpi_accounts = Transfer {
            from: self.system_bio_account.to_account_info(),
            to: self.user_superior_token_account.to_account_info(),
            authority: self.nft_mining_system.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, referral_reward)?;

        
        let black_hole = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        // 给黑洞地址转账10%
        if black_hole == self.black_hole_bio_account.owner {
        let cpi_accounts = Transfer {
            from: self.system_bio_account.to_account_info(),
            to: self.black_hole_bio_account.to_account_info(),
            authority: self.nft_mining_system.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, referral_reward * 2)?;
        }else{
            return Err(NftStakingError::NotBlackHole.into());
        }


        // 给用户发放主要奖励
        let cpi_accounts = Transfer {
            from: self.system_bio_account.to_account_info(),
            to: self.user_bio_account.to_account_info(),
            authority: self.nft_mining_system.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, user_reward)?;

        // 更新用户状态
        let user = &mut self.order_info;
        
        user.receivedReward = user.receivedReward
            .checked_add(reward_amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        user.last_claim_timestamp = current_timestamp;

        user.accumulated_reward = 0;

        update_reward_debt(&mut self.nft_mining_system, user);
       
        // 记录操作日志
        msg!("用户质押权重: {}", user.total_power);
        msg!("总奖励金额: {} BIO", reward_amount);
        msg!("用户获得奖励: {} BIO", user_reward);
        msg!("推荐奖励: {} BIO", referral_reward);
        msg!("累计奖励: {}", user.accumulated_reward);
        msg!("已领取奖励: {}", user.receivedReward);
        msg!("领取时间: {}", current_timestamp);
        msg!("下次可领取时间: {}", current_timestamp + REWARD_CLAIM_COOLDOWN);
        msg!("奖励债务: {}", user.reward_debt);

        Ok(())
    }

    
   


}