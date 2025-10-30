use anchor_lang::prelude::*;

/// 释放周期类型
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VestingPeriod {
    /// 按天释放
    Daily,
    /// 按月释放
    Monthly,
    /// 按年释放
    Yearly,
    /// 线性释放（按秒）
    Linear,
}

impl VestingPeriod {
    /// 获取周期对应的秒数
    pub fn to_seconds(&self) -> i64 {
        match self {
            VestingPeriod::Daily => 24 * 60 * 60,      // 1天 = 86400秒
            VestingPeriod::Monthly => 30 * 24 * 60 * 60, // 1月 = 30天
            VestingPeriod::Yearly => 365 * 24 * 60 * 60, // 1年 = 365天
            VestingPeriod::Linear => 1,                 // 线性释放按秒计算
        }
    }
}

/// 线性释放计划账户
#[account]
pub struct VestingSchedule {
    /// 创建者
    pub creator: Pubkey,
    /// 受益人
    pub beneficiary: Pubkey,
    /// 代币mint
    pub mint: Pubkey,
    /// 总金额
    pub total_amount: u64,
    /// 已提取金额
    pub claimed_amount: u64,
    /// 开始时间（Unix时间戳）
    pub start_time: i64,
    /// 释放周期类型
    pub vesting_period: VestingPeriod,
    /// 释放周期数量（例如：12个月、365天、2年等）
    pub period_count: u32,
    /// 每个周期释放的金额
    pub amount_per_period: u64,
    /// 创建时间
    pub created_at: i64,
}

impl VestingSchedule {
    pub const LEN: usize = 8 + // discriminator
        32 + // creator
        32 + // beneficiary
        32 + // mint
        8 +  // total_amount
        8 +  // claimed_amount
        8 +  // start_time
        1 +  // vesting_period (enum)
        4 +  // period_count
        8 +  // amount_per_period
        8;   // created_at

    /// 获取总释放持续时间（秒）
    pub fn get_total_duration(&self) -> i64 {
        let period_seconds = self.vesting_period.to_seconds();
        period_seconds * self.period_count as i64
    }

    /// 获取已完成的周期数
    pub fn get_completed_periods(&self, current_time: i64) -> u32 {
        if current_time < self.start_time {
            return 0;
        }

        let elapsed_time = current_time - self.start_time;
        let period_seconds = self.vesting_period.to_seconds();

        let completed_periods = elapsed_time / period_seconds;
        completed_periods.min(self.period_count as i64) as u32
    }

    /// 获取可提取金额
    pub fn get_claimable_amount(&self, current_time: i64) -> Result<u64> {
        // 如果还没到开始时间，返回0
        if current_time < self.start_time {
            return Ok(0);
        }

        let vested_amount = match self.vesting_period {
            VestingPeriod::Linear => {
                // 线性释放：按时间比例计算
                let elapsed_time = current_time - self.start_time;
                let total_duration = self.get_total_duration();

                if elapsed_time >= total_duration {
                    self.total_amount
                } else {
                    let vested_amount = (self.total_amount as u128)
                        .checked_mul(elapsed_time as u128)
                        .ok_or(crate::errors::VestingError::MathOverflow)?
                        .checked_div(total_duration as u128)
                        .ok_or(crate::errors::VestingError::MathOverflow)? as u64;
                    vested_amount
                }
            },
            _ => {
                // 周期性释放：按完成的周期数计算
                let completed_periods = self.get_completed_periods(current_time);
                let vested_amount = (completed_periods as u64)
                    .checked_mul(self.amount_per_period)
                    .ok_or(crate::errors::VestingError::MathOverflow)?;
                vested_amount.min(self.total_amount)
            }
        };

        // 可提取金额 = 已释放金额 - 已提取金额
        vested_amount
            .checked_sub(self.claimed_amount)
            .ok_or(crate::errors::VestingError::MathOverflow.into())
    }

    /// 获取剩余锁定金额
    pub fn get_locked_amount(&self, current_time: i64) -> Result<u64> {
        let claimable = self.get_claimable_amount(current_time)?;
        Ok(self.total_amount - self.claimed_amount - claimable)
    }

    /// 检查是否已完全释放
    pub fn is_fully_vested(&self, current_time: i64) -> bool {
        match self.vesting_period {
            VestingPeriod::Linear => {
                current_time >= self.start_time + self.get_total_duration()
            },
            _ => {
                self.get_completed_periods(current_time) >= self.period_count
            }
        }
    }

    /// 获取释放进度百分比 (0-100)
    pub fn get_vesting_progress(&self, current_time: i64) -> u8 {
        if current_time < self.start_time {
            return 0;
        }

        if self.is_fully_vested(current_time) {
            return 100;
        }

        match self.vesting_period {
            VestingPeriod::Linear => {
                let elapsed_time = current_time - self.start_time;
                let total_duration = self.get_total_duration();
                let progress = (elapsed_time as u128 * 100) / total_duration as u128;
                progress.min(100) as u8
            },
            _ => {
                let completed_periods = self.get_completed_periods(current_time);
                let progress = (completed_periods as u128 * 100) / self.period_count as u128;
                progress.min(100) as u8
            }
        }
    }

    /// 获取下次释放时间
    pub fn get_next_release_time(&self, current_time: i64) -> Option<i64> {
        if self.is_fully_vested(current_time) {
            return None;
        }

        match self.vesting_period {
            VestingPeriod::Linear => {
                // 线性释放没有固定的下次释放时间
                None
            },
            _ => {
                let completed_periods = self.get_completed_periods(current_time);
                if completed_periods < self.period_count {
                    let next_period = completed_periods + 1;
                    let period_seconds = self.vesting_period.to_seconds();
                    Some(self.start_time + (next_period as i64 * period_seconds))
                } else {
                    None
                }
            }
        }
    }
}
