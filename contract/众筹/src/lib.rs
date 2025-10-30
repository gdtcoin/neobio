use anchor_lang::prelude::*;



pub mod constants;
pub mod structures;

pub use structures::{
    initialize::*, 
    create_phase::*,
    errors::*,
    usdt_wsol::*,
    wsol_gdtc::*,
    gdtc_bio::*,
    claim_tokens::*
};


declare_id!("AqXKuogwtfi45d4vKdUdXymr2yQhBXsfV8hADmL8NYy6");

#[program]
pub mod crowdfunding {
    use super::*;

   pub fn initialize_crowdfunding(
    ctx: Context<InitializeCrowdfunding>,
    admin: Pubkey,
    project_signer: Pubkey,
    start_time:i64,
    wsol_mint_account:Pubkey,
    gdtc_mint_account:Pubkey,
    bio_mint_account:Pubkey,
    gdtc_pool_address:Pubkey,
    gdtc_blackhole_address:Pubkey,
    ) -> Result<()> {

    ctx.accounts.process(
        admin, 
        project_signer,
        start_time,
        wsol_mint_account,
        gdtc_mint_account,
        bio_mint_account,
        gdtc_pool_address,
        gdtc_blackhole_address
    )
    }

     /// 创建后续销售期（phase_id 自动递增，每期固定 100 份）
    pub fn create_phase(
        ctx: Context<CreatePhase>,
        price_per_share: u64,
        start_time: i64,
        id:u64
    ) -> Result<()> {

    let crowdfunding_info = &ctx.accounts.crowdfunding_info;
    

    // ✅ 3. 可选：校验当前轮次是否允许创建新轮（例如是否存在未结束轮次）
    if crowdfunding_info.phase_count >= 7 {
        return Err(CrowdfundingError::TooManyPhases.into());
    }
    
        ctx.accounts.process(price_per_share, start_time,id)
    }

    //usdt 到 wsol 兑换
    pub fn usdt_to_wsol(
        ctx: Context<UsdtToWsol>,
        shares_to_buy: u64,
        phase_id:u64,
        user_superior_address:Pubkey,
    ) -> Result<()> {


        require!(shares_to_buy > 0, CrowdfundingError::InvalidShareAmount);

        // ✅ 校验当前期是否已开始
        let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time >= ctx.accounts.sale_phase.start_time,
        CrowdfundingError::PhaseNotStarted
    );
        ctx.accounts.process(shares_to_buy,phase_id,user_superior_address)
    }

 

     /// 用户购买众筹份额（USDT 支付 + 生成 UserPurchase 记录）
    pub fn wsol_gdtc(
        ctx: Context<WsolGdtc>,
        shares_to_buy: u64,
        phase_id:u64,
        
    ) -> Result<()> {
        ctx.accounts.process(shares_to_buy,phase_id)
    }


        /// 项目方将 GDTC 兑换为 BIO（通过 PDA 账户）
        pub fn gdtc_to_bio(
            ctx: Context<GdtcToBio>,
            shares_to_buy: u64,
            phase_id:u64,
        ) -> Result<()> {

            
            ctx.accounts.process(shares_to_buy,phase_id)
        }
    


    //用户领取
    pub fn claim_tokens(
    ctx: Context<ClaimTokens>,
    id: u64,
        sold_share:u64,
) -> Result<()> {

    let bump = ctx.bumps.crowdfunding_info;
        ctx.accounts.process(id,sold_share,bump)
}


 
}
