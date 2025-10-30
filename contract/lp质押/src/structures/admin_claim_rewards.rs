use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke_signed;
use spl_token;

use crate::constants::*;
use crate::StakingError;

#[derive(Accounts)]
pub struct AdminClaimRewards<'info> {
    #[account(mut)]
    pub staking_instance: Account<'info, StakingInstance>,

    #[account(mut)]
    pub gdtc_reward_out_account: Account<'info, TokenAccount>, // GDTC 奖励资金池

    #[account(mut)]
    pub user_gdtc_token_account: Account<'info, TokenAccount>, // 接收账户（管理员提取目标）

    /// CHECK: PDA 仅用于签名，不做类型检查
    #[account(seeds = [crate::LPTOKEN_SEED.as_ref()], bump)]
    pub pda_account: UncheckedAccount<'info>,

    pub authority: Signer<'info>,  // 只有 StakingInstance.authority 可操作
    pub token_program: Program<'info, Token>,
}

impl<'info> AdminClaimRewards<'info> {
    pub fn process(&mut self, claim_number: u64) -> Result<()> {
        let staking_instance = &mut self.staking_instance;

        // 校验 StakingInstance 的 PDA
        let (expected_staking_address, _) =
            Pubkey::find_program_address(&[crate::STAKING_SEED.as_ref()], &crate::ID);
        if staking_instance.key() != expected_staking_address {
            return Err(StakingError::InvalidStakingInstance.into());
        }

        // 校验调用者是否为管理员
        if staking_instance.authority != self.authority.key() {
            return Err(StakingError::Unauthorized.into());
        }

        // 校验 GDTC 奖励资金池的 PDA 拥有者
        let (expected_pda_address, _) =
            Pubkey::find_program_address(&[crate::LPTOKEN_SEED.as_ref()], &crate::ID);
        if expected_pda_address != self.gdtc_reward_out_account.owner {
            return Err(StakingError::PdaAccountIsNotMatch.into());
        }

        // 校验资金池的 Mint
        if staking_instance.reward_token_mint != self.gdtc_reward_out_account.mint {
            return Err(StakingError::MintAccountIsNotMatch.into());
        }

        // PDA 签名种子
        let bump_seed = self.__bump;
        let signer_seeds: &[&[&[u8]]] = &[&[crate::LPTOKEN_SEED.as_ref(), &[bump_seed]]];

        // 从 GDTC 池转账到管理员指定的接收账户
        let transfer_instruction = spl_token::instruction::transfer(
            &self.token_program.key(),
            &self.gdtc_reward_out_account.key(),
            &self.user_gdtc_token_account.key(),
            &self.pda_account.key(),
            &[],
            claim_number,
        )?;

        invoke_signed(
            &transfer_instruction,
            &[
                self.token_program.to_account_info(),
                self.gdtc_reward_out_account.to_account_info(),
                self.user_gdtc_token_account.to_account_info(),
                self.pda_account.to_account_info(),
            ],
            signer_seeds,
        )?;

        Ok(())
    }
}