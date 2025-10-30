use anchor_lang::prelude::*;

// 模块声明
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

// 重新导出
pub use errors::*;
pub use state::*;
pub use utils::*;

// 导入指令模块但不重新导出函数，避免命名冲突
use instructions::*;

declare_id!("9YLCGJaks5rLCpthMP8LoqW72yzkuxBGVxRzGK3ACrTc");

#[program]
pub mod vesting_project {
    use super::*;

    /// 创建释放计划
    pub fn create_vesting_schedule(
        ctx: Context<CreateVestingSchedule>,
        total_amount: u64,
        start_time: i64,
        vesting_period: VestingPeriod,
        period_count: u32,
    ) -> Result<()> {
        instructions::create_vesting_schedule::create_vesting_schedule(ctx, total_amount, start_time, vesting_period, period_count)
    }

    /// 提取已释放的代币
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim::claim(ctx)
    }

    /// 查询可提取金额
    pub fn get_claimable_amount(ctx: Context<GetClaimableAmount>) -> Result<u64> {
        instructions::get_claimable_amount::get_claimable_amount(ctx)
    }

    /// 查询释放计划详细信息
    pub fn get_vesting_info(ctx: Context<GetVestingInfo>) -> Result<VestingInfo> {
        instructions::get_vesting_info::get_vesting_info(ctx)
    }

    /// 取消释放计划（仅创建者可用）
    pub fn cancel_vesting(ctx: Context<CancelVesting>) -> Result<()> {
        instructions::cancel_vesting::cancel_vesting(ctx)
    }
}
