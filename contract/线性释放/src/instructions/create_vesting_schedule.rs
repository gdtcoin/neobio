use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{VestingSchedule, VestingPeriod};
use crate::errors::VestingError;
use crate::utils::{validate_vesting_params_v2, check_sufficient_balance};



pub fn validate_vesting_period(vesting_period: VestingPeriod) -> Result<()> {
    match vesting_period {
        VestingPeriod::Daily 
        | VestingPeriod::Monthly 
        | VestingPeriod::Yearly 
        | VestingPeriod::Linear => Ok(()),
        // 如果未来扩展了未知枚举值
        _ => Err(VestingError::InvalidVestingPeriod.into()),
    }
}

/// 创建释放计划
pub fn create_vesting_schedule(
    ctx: Context<CreateVestingSchedule>,
    total_amount: u64,
    start_time: i64,
    vesting_period: VestingPeriod,
    period_count: u32,
) -> Result<()> {

    validate_vesting_period(vesting_period)?;
    
    let vesting_schedule = &mut ctx.accounts.vesting_schedule;
    let clock = Clock::get()?;

    // 验证参数
    validate_vesting_params_v2(total_amount, start_time, period_count, clock.unix_timestamp)?;

    // 检查创建者代币账户余额
    check_sufficient_balance(&ctx.accounts.creator_token_account, total_amount)?;

    // 计算每个周期释放的金额
    let amount_per_period = match vesting_period {
        VestingPeriod::Linear => {
            // 线性释放不需要每周期金额，设为0
            0
        },
        _ => {
            // 周期性释放：总金额除以周期数
            total_amount / period_count as u64
        }
    };

    // 初始化释放计划
    vesting_schedule.creator = ctx.accounts.creator.key();
    vesting_schedule.beneficiary = ctx.accounts.beneficiary.key();
    vesting_schedule.mint = ctx.accounts.mint.key();
    vesting_schedule.total_amount = total_amount;
    vesting_schedule.claimed_amount = 0;
    vesting_schedule.start_time = start_time;
    vesting_schedule.vesting_period = vesting_period;
    vesting_schedule.period_count = period_count;
    vesting_schedule.amount_per_period = amount_per_period;
    vesting_schedule.created_at = clock.unix_timestamp;

    // 转移代币到托管账户
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.creator_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.creator.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, total_amount)?;

    let period_name = match vesting_period {
        VestingPeriod::Daily => "days",
        VestingPeriod::Monthly => "months",
        VestingPeriod::Yearly => "years",
        VestingPeriod::Linear => "linear",
    };

    msg!(
        "Vesting schedule created: {} tokens over {} {} starting at {}",
        total_amount,
        period_count,
        period_name,
        start_time
    );
    Ok(())
}

/// 创建释放计划的账户验证
#[derive(Accounts)]
pub struct CreateVestingSchedule<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    /// CHECK: 受益人账户，不需要签名
    pub beneficiary: UncheckedAccount<'info>,
    
    /// 代币mint
    pub mint: Account<'info, anchor_spl::token::Mint>,
    
    /// 创建者的代币账户
    #[account(
        mut,
        constraint = creator_token_account.mint == mint.key() @ VestingError::InvalidTokenMint,
        constraint = creator_token_account.owner == creator.key() @ VestingError::Unauthorized
    )]
    pub creator_token_account: Account<'info, TokenAccount>,
    
    /// 托管代币账户（PDA）
    #[account(
        mut,
        constraint = vault_token_account.mint == mint.key() @ VestingError::InvalidTokenMint,
        seeds = [b"vault", vesting_schedule.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    /// 释放计划账户（PDA）
    #[account(
        init,
        payer = creator,
        space = VestingSchedule::LEN,
        seeds = [b"vesting", creator.key().as_ref(), beneficiary.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
