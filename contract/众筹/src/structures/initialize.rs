use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use crate::constants::*;
use super::*;
use super::errors::CrowdfundingError;
use std::str::FromStr;

/// 初始化众筹项目（自动创建第一期）
/// 注意：本交易仅写入与“结构中存在的字段”一致的内容。
#[derive(Accounts)]
#[instruction(admin: Pubkey, project_signer: Pubkey)]
pub struct InitializeCrowdfunding<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [CROWDFUNDING_SEED],
        bump,
        space = 8 + core::mem::size_of::<CrowdfundingInfo>()
    )]
    pub crowdfunding_info: Account<'info, CrowdfundingInfo>,

    // 第一 期固定 phase_id = 1；与 SalePhase.phase_id 的类型保持一致（u32）
    #[account(
        init,
        payer = authority,
        seeds = [SALE_PHASE_SEED, &1u64.to_le_bytes()],
        bump,
        space = 8 + core::mem::size_of::<SalePhase>()
    )]
    pub first_phase: Account<'info, SalePhase>,

    /// 部署者（唯一需要签名的人）
    #[account(mut)]
    pub authority: Signer<'info>,


    //usdt mint地址
    #[account(mut)]
    pub usdt_mint_account: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeCrowdfunding<'info> {
    pub fn process(
        &mut self,
        admin: Pubkey,
        project_signer: Pubkey,
        start_time: i64,
        wsol_mint_account: Pubkey,
        gdtc_mint_account: Pubkey,
        bio_mint_account: Pubkey,
        gdtc_pool_address: Pubkey,
        gdtc_blackhole_address: Pubkey,
    ) -> Result<()> {


        let black_hole = Pubkey::from_str("11111111111111111111111111111111").unwrap();

        require!(gdtc_blackhole_address == black_hole, CrowdfundingError::NotBlackHole);

        // 1) 参数校验
        require!(start_time > 0, CrowdfundingError::InvalidStartTime);

        // 2) 初始化全局众筹信息（严格遵循你的结构字段）
        let crowdfunding = &mut self.crowdfunding_info;
        crowdfunding.initialized       = true;
        crowdfunding.authority         = self.authority.key();
        crowdfunding.admin             = admin;
        crowdfunding.usdt_mint_account = self.usdt_mint_account.key();
        crowdfunding.wsol_mint_account = wsol_mint_account;
        crowdfunding.gdtc_mint_account = gdtc_mint_account;
        crowdfunding.bio_mint_account = bio_mint_account;
        crowdfunding.gdtc_blackhole_address = gdtc_blackhole_address;

        crowdfunding.total_shares    = TOTAL_SHARES;          // 800
        crowdfunding.sold_shares     = 0;
        crowdfunding.token_per_share = TOKEN_PER_SHARE;       // 例如 656_25 * 10^decimals，如有需要可与 Mint decimals 对齐
        crowdfunding.vesting_days    = VESTING_DAYS;          // 365
        crowdfunding.project_signer  = project_signer;
        crowdfunding.phase_count     = 1;
        crowdfunding.gdtc_pool_address = gdtc_pool_address;

        // 3) 计算第一期每份售价（单位：usdt），例如固定 2 usdt
        let usdt_decimals = self.usdt_mint_account.decimals as u32;
        let ten = 10u64;
        let base = ten.checked_pow(usdt_decimals)
            .ok_or(CrowdfundingError::ArithmeticOverflow)?;

        //第一期价格500usdt
        let price_per_share = 500u64
            .checked_mul(base)
            .ok_or(CrowdfundingError::ArithmeticOverflow)?;

        // 4) 初始化第一期信息
        let first_phase = &mut self.first_phase;
        first_phase.phase_id       = 1;
        first_phase.price_per_share= price_per_share;
        first_phase.max_shares     = 200;
        first_phase.sold_shares    = 0;
        first_phase.start_time     = start_time;
        first_phase.end_time       = 0;      // 0 表示暂未设置结束时间；后续由管理端更新
        first_phase.active         = true;

        Ok(())
    }
}