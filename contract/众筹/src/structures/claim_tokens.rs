use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer, Burn};
use crate::constants::*;
use super::{CrowdfundingInfo, SalePhase, UserPurchase};
use super::errors::CrowdfundingError;

#[derive(Accounts)]
#[instruction(id: u64, sold_share: u64)]
pub struct ClaimTokens<'info> {
    #[account(
        seeds = [CROWDFUNDING_SEED], 
        bump,
        constraint = crowdfunding_info.initialized @ CrowdfundingError::Unauthorized,
    )]
    pub crowdfunding_info: Account<'info, CrowdfundingInfo>,

    #[account(
        seeds = [SALE_PHASE_SEED, &id.to_le_bytes()], 
        bump,
    )]
    pub sale_phase: Account<'info, SalePhase>,

    #[account(
        mut,
        seeds = [
            USER_PURCHASE_SEED,
            user.key().as_ref(),
            &id.to_le_bytes(),
            &user_purchase.purchase_id.to_le_bytes(),
        ],
        bump,
        constraint = user_purchase.user == user.key() @ CrowdfundingError::Unauthorized,
    )]
    pub user_purchase: Account<'info, UserPurchase>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// 合约持有的 Token 账户 (Vault)
    #[account(
        mut,
        constraint = vault_token_account.mint == crowdfunding_info.bio_mint_account @ CrowdfundingError::TokenMintMismatch,
        constraint = vault_token_account.owner == crowdfunding_info.key() @ CrowdfundingError::InvalidVaultOwner,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// 用户的 Token 接收账户
    #[account(
        mut,
        constraint = user_token_account.mint == crowdfunding_info.bio_mint_account @ CrowdfundingError::TokenMintMismatch,
        constraint = user_token_account.owner == user.key() @ CrowdfundingError::Unauthorized,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// 用户上级 Token 接收地址
    #[account(mut,
        constraint = user_superior_token_account.mint == crowdfunding_info.bio_mint_account @ CrowdfundingError::TokenMintMismatch,
        constraint = user_superior_token_account.owner == user_purchase.superior_address @ CrowdfundingError::Unauthorized,
    )]
    pub user_superior_token_account: Account<'info, TokenAccount>,

    //全网分红池 token接收地址
    #[account(mut,
        constraint = user_global_pool_token_account.mint == crowdfunding_info.bio_mint_account @ CrowdfundingError::TokenMintMismatch,
        constraint = user_global_pool_token_account.owner == crowdfunding_info.gdtc_pool_address @ CrowdfundingError::Unauthorized,
    )]
    pub user_global_pool_token_account: Account<'info, TokenAccount>,

    //黑洞地址token account
    #[account(mut,
        constraint = gdtc_blackhole_token_account.mint == crowdfunding_info.bio_mint_account @ CrowdfundingError::TokenMintMismatch,
        constraint = gdtc_blackhole_token_account.owner == crowdfunding_info.gdtc_blackhole_address @ CrowdfundingError::Unauthorized,
    )]
    pub gdtc_blackhole_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ClaimTokens<'info> {
    pub fn process(&mut self,id:u64,sold_id:u64, pump_seed: u8) -> Result<()> {
        let user_purchase = &mut self.user_purchase;

        let current_time = Clock::get()?.unix_timestamp;
        
        // 计算已过天数
        let elapsed_seconds = current_time.saturating_sub(user_purchase.purchase_time);
        if elapsed_seconds <= 0 {
            return Err(CrowdfundingError::NothingToClaim.into());
        }

        // 计算已过天数
        msg!("elapsed_seconds: {}", elapsed_seconds);
       
        let elapsed_days = (elapsed_seconds / 86400) as u64; 
        msg!("elapsed_days: {}", elapsed_days);
        

        // 计算应释放总额 - 修复线性释放逻辑
        let total_claimable = if elapsed_days >= user_purchase.vesting_days {
            user_purchase.token_amount
        } else {
            // 线性释放：已过天数 / 总释放天数 * 总代币数量
            user_purchase.token_amount
                .checked_mul(elapsed_days)
                .ok_or(ProgramError::InvalidArgument)?
                .checked_div(user_purchase.vesting_days)
                .ok_or(ProgramError::InvalidArgument)?
        };

        //打印总可领数量
        msg!("total_claimable: {}", total_claimable);
        //打印已领数量
        msg!("user_purchase.claimed_amount: {}", user_purchase.claimed_amount);
        
        // 计算本次可领取数量
        let claimable_now = total_claimable
            .checked_sub(user_purchase.claimed_amount)
            .ok_or(ProgramError::InvalidArgument)?;

        if claimable_now == 0 {
            return Err(CrowdfundingError::NothingToClaim.into());
        }
        //打印本次可领数量
        msg!("claimable_now: {}", claimable_now);

        require!(
            user_purchase.burn_bio,
            CrowdfundingError::NothingToClaim
        );
        // 验证vault账户余额是否足够
        require!(
            self.vault_token_account.amount >= claimable_now,
            CrowdfundingError::InsufficientBalance
        );

       
        // let max_claim_number = 1000 * 10_u64.pow(6 as u32);
        // if claimable_now > max_claim_number{
        //     return Err(CrowdfundingError::MaxRewardsToClaim.into());
        // }

        // 计算分配比例 - 使用更精确的计算避免精度损失
        let superior_amount = claimable_now * 5 / 100; // 5% 给上级
        let global_pool_amount = claimable_now * 5 / 100; // 5% 给全网分红
        let burn_amount = claimable_now * 10 / 100; // 10% 销毁
        
        // 计算已分配的总量
        let allocated_amount = superior_amount + global_pool_amount + burn_amount;
        
        // 用户获得剩余部分，确保总和为100%
        let user_amount = claimable_now - allocated_amount;

        // 验证分配比例
        msg!("Allocation verification:");
        msg!("  Total claimable: {}", claimable_now);
        msg!("  Superior (5%): {}", superior_amount);
        msg!("  Global pool (5%): {}", global_pool_amount);
        msg!("  Burn (10%): {}", burn_amount);
        msg!("  User (remaining): {}", user_amount);
        msg!("  Total allocated: {}", allocated_amount + user_amount);
        
        // 确保分配正确
        require!(
            allocated_amount + user_amount == claimable_now,
            CrowdfundingError::InvalidShareAmount
        );
        
        let signer_seeds: &[&[&[u8]]] = &[&[CROWDFUNDING_SEED, &[pump_seed]]];

        // 1. 转给上级 (5%)
        if superior_amount > 0 {
            let cpi_accounts = Transfer {
                from: self.vault_token_account.to_account_info(),
                to: self.user_superior_token_account.to_account_info(),
                authority: self.crowdfunding_info.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
            token::transfer(cpi_ctx, superior_amount)?;
            msg!("Transferred {} tokens to superior", superior_amount);
        }

        // 2. 转给全网分红池 (5%)
        if global_pool_amount > 0 {
            let cpi_accounts = Transfer {
                from: self.vault_token_account.to_account_info(),
                to: self.user_global_pool_token_account.to_account_info(),
                authority: self.crowdfunding_info.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
            token::transfer(cpi_ctx, global_pool_amount)?;
            msg!("Transferred {} tokens to global pool", global_pool_amount);
        }

        // 3. 销毁代币 (10%)
        if burn_amount > 0 {
            let cpi_accounts = Transfer {
                from: self.vault_token_account.to_account_info(),
                to: self.gdtc_blackhole_token_account.to_account_info(),
                authority: self.crowdfunding_info.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
            token::transfer(cpi_ctx, burn_amount)?;
            msg!("Transferred {} tokens to gdtc blackhole", burn_amount);
        }

        // 4. 转给用户 (80%)
        if user_amount > 0 {
            let cpi_accounts = Transfer {
                from: self.vault_token_account.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: self.crowdfunding_info.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
            token::transfer(cpi_ctx, user_amount)?;
            msg!("Transferred {} tokens to user", user_amount);
        }

        // 更新已领取数量
        user_purchase.claimed_amount = user_purchase
            .claimed_amount
            .checked_add(claimable_now)
            .ok_or(ProgramError::InvalidArgument)?;

        Ok(())
    }
}