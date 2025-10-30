use anchor_lang::prelude::*;
use crate::constants::{CROWDFUNDING_SEED, SALE_PHASE_SEED};
use super::{CrowdfundingInfo, SalePhase};
use super::errors::CrowdfundingError;

#[derive(Accounts)]
#[instruction(price_per_share: u64, start_time: i64,id:u64)]
pub struct CreatePhase<'info> {
    #[account(
        mut,
        seeds = [CROWDFUNDING_SEED],
        bump,
        constraint = crowdfunding_info.initialized @ CrowdfundingError::Unauthorized,
        constraint = crowdfunding_info.phase_count < 8 @ CrowdfundingError::TooManyPhases,
    )]
    pub crowdfunding_info: Account<'info, CrowdfundingInfo>,

    #[account(
        init,
        payer = admin,
        seeds = [
            SALE_PHASE_SEED, 
            &id.to_le_bytes()
        ],
        bump,
        space = 8 + core::mem::size_of::<SalePhase>()
    )]
    pub sale_phase: Account<'info, SalePhase>,

    /// 管理员，必须等于 CrowdfundingInfo.admin
    #[account(
        mut,
        constraint = admin.key() == crowdfunding_info.admin @ CrowdfundingError::Unauthorized,
    )]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreatePhase<'info> {
    pub fn process(
        &mut self,
        price_per_share: u64,
        start_time: i64,
        id:u64
    ) -> Result<()> {
        let crowdfunding = &mut self.crowdfunding_info;

        // 验证价格参数
        require!(price_per_share > 0, CrowdfundingError::InvalidShareAmount);
        
        // 验证时间参数
        require!(start_time > 0, CrowdfundingError::InvalidShareAmount);

        // 自动生成期数
        let phase_id = crowdfunding.phase_count + 1;

        // 每期固定 100 份
        let fixed_shares: u64 = 100;

        // 初始化新一期
        let phase = &mut self.sale_phase;
        phase.phase_id = phase_id;
        phase.price_per_share = price_per_share;
        phase.max_shares = fixed_shares;
        phase.sold_shares = 0;
        phase.start_time = start_time;
        phase.end_time = 0;        // 默认未定义，管理员后续设置
        phase.active = true;
      
        // 更新全局期数
        crowdfunding.phase_count = phase_id;

        Ok(())
    }
}