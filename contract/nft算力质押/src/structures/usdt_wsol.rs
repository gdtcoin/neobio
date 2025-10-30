use anchor_lang::prelude::*;

use anchor_spl::{
    token::{self, Mint, Token, TokenAccount, Transfer},
    token_interface::TokenInterface,
};

use crate::structures::*;
use crate::constants::*;
use crate::errors::NftStakingError;

use raydium_cp_swap::{
    cpi,
    program::RaydiumCpSwap,
    states::{AmmConfig, ObservationState, PoolState},
};

/// 用 wSOL 购买并印刷一个 Edition
#[derive(Accounts)]
pub struct UsdtWsol<'info> {
    /// 付款人与接收者（本例中一致；若要送给他人，可把 recipient 改成独立账户）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 系统 PDA（已初始化，持有 Master 那 1 枚）
    #[account(
        mut,
        seeds = [NFT_MINING_SYSTEM_SEED],
        bump,
        constraint = nft_mining_system.is_initialized @ NftStakingError::Unauthorized,
    )]
    pub nft_mining_system: Account<'info, NftMiningSystem>,

    /// 用户状态（按用户+固定种子派生，避免多用户冲突）
    #[account(
        init,
        payer = user,
        space = 8 + core::mem::size_of::<OrderInfo>(),
       
        seeds = [ORDER_INFO_SEED, &(nft_mining_system.order_info_index + 1).to_le_bytes()],
        bump,
    )]
    pub order_info: Account<'info, OrderInfo>,

    // ====== usdt 相关 ======
    #[account(
        mut,
        constraint = user_usdt_account.mint == nft_mining_system.usdt_mint @ NftStakingError::InvalidUsdtMint,
        constraint = user_usdt_account.owner == user.key() @ NftStakingError::InvalidUsdtOwner,
    )]
    pub user_usdt_account: Account<'info, TokenAccount>,

    //用户wsol账户
    #[account(
        mut,
        constraint = user_wsol_account.mint == nft_mining_system.wsol_mint @ NftStakingError::InvalidWsolMint,
        constraint = user_wsol_account.owner == user.key() @ NftStakingError::InvalidWsolOwner,
    )]
    pub user_wsol_account: Account<'info, TokenAccount>,

    //上级usdt账户
    #[account(
        mut,
        constraint = user_superior_usdt_account.mint == nft_mining_system.usdt_mint @ NftStakingError::InvalidUsdtMint,
    )]
    pub user_superior_usdt_account: Account<'info, TokenAccount>,

    //市场分红usdt
    #[account(
        mut,
        constraint = market_pool_address_usdt_account.mint == nft_mining_system.usdt_mint @ NftStakingError::InvalidUsdtMint,
        constraint = market_pool_address_usdt_account.owner == nft_mining_system.market_pool_address @ NftStakingError::InvalidMarketPoolAddress,
    )]
    pub market_pool_address_usdt_account: Account<'info, TokenAccount>,

    //usdt mint
    #[account(
        mut,
        constraint = usdt_mint_account.key() == nft_mining_system.usdt_mint @ NftStakingError::InvalidUsdtMint,
    )]
    pub usdt_mint_account: Account<'info, Mint>,


    //raydium cp swap program

    // -------- Raydium CP Swap 程序 --------
    pub cp_swap_program: Program<'info, RaydiumCpSwap>,

    /// CHECK: Raydium 权限账户
    #[account(
        seeds = [raydium_cp_swap::AUTH_SEED.as_bytes()],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

        // -------- usdt -> wsol 兑换池配置 --------
        /// usdt-wsol 兑换池的 AMM 配置
    #[account(mut)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// usdt-wsol 兑换池状态
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// usdt 输入金库
    #[account(mut)]
    pub input_vault: Box<Account<'info, TokenAccount>>,

    /// wsol 输出金库
    #[account(mut)]
    pub output_vault: Box<Account<'info, TokenAccount>>,

    /// usdt 输入代币程序
    pub input_token_program: Interface<'info, TokenInterface>,

    /// wsol 输出代币程序
    pub output_token_program: Interface<'info, TokenInterface>,

    /// usdt 输入代币 Mint
    pub input_token_mint: Box<Account<'info, Mint>>,

    /// wsol 输出代币 Mint
    pub output_token_mint: Box<Account<'info, Mint>>,

    /// usdt-wsol 兑换池的观察状态
    #[account(mut)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    // ====== 程序 ======
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}



impl<'info> UsdtWsol<'info> {
    pub fn process(
        &mut self,
        usdt_amount: u64,
    ) -> Result<()> {
        // 1) 校验 wSOL 金额（也即本次印刷数量）

        require!(
            matches!(usdt_amount, 100 | 300 | 500 | 1000),
            NftStakingError::InvalidUsdtAmount
        );

        //乘以wsol 的 decimals，需要获取
        let usdt_decimals = self.usdt_mint_account.decimals;
        let usdt_amount = usdt_amount * 10_u64.pow(usdt_decimals as u32);

        let transfer_amount = usdt_amount * 5 / 100; 
        let transfer_amount2 = usdt_amount * 10 / 100; 
        let transfer_amount3 = usdt_amount * 85 / 100; 


        // 给上级转5%
           
        let cpi_accounts = Transfer {
            from: self.user_usdt_account.to_account_info(),
            to: self.user_superior_usdt_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, transfer_amount)?;



        //给市场分红转10%
        
        let cpi_accounts = Transfer {
            from: self.user_usdt_account.to_account_info(),
            to: self.market_pool_address_usdt_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, transfer_amount2)?;



        // 剩余85% 购买wsol

              // 记录兑换前余额
              let wsol_before = self.user_wsol_account.amount;


                          
      {
              // CPI 2: Raydium swap（usdt -> wsol）

          
              
                  let cpi_accounts = cpi::accounts::Swap {
                      payer: self.user.to_account_info(),  
                      authority: self.authority.to_account_info(),
                      amm_config: self.amm_config.to_account_info(),
                      pool_state: self.pool_state.to_account_info(),
                      input_token_account: self.user_usdt_account.to_account_info(),
                      output_token_account: self.user_wsol_account.to_account_info(),
                      input_vault: self.input_vault.to_account_info(),
                      output_vault: self.output_vault.to_account_info(),
                      input_token_program: self.input_token_program.to_account_info(),
                      output_token_program: self.output_token_program.to_account_info(),
                      input_token_mint: self.input_token_mint.to_account_info(),
                      output_token_mint: self.output_token_mint.to_account_info(),
                      observation_state: self.observation_state.to_account_info(),
                  };
      
                      let cpi_ctx = CpiContext::new(
                      self.cp_swap_program.to_account_info(),
                      cpi_accounts
                  );
      
                  cpi::swap_base_input(cpi_ctx, transfer_amount3, 0)?;
       
              }
              
              // 重新加载账户以获取最新余额
              self.user_wsol_account.reload()?;
       
              
              // 计算兑换结果
              let wsol_after = self.user_wsol_account.amount;
              let wsol_received = wsol_after
                  .checked_sub(wsol_before)
                  .ok_or(NftStakingError::ArithmeticOverflow)?;
              

        // 3) 更新用户状态
        self.order_info.user_address = self.user.key();
        self.order_info.user_superior_account = self.user_superior_usdt_account.owner;
        
        self.order_info.total_power = usdt_amount;
        self.order_info.is_transfer_usdt = true;  
        self.order_info.investment_amount = usdt_amount;
       
        self.order_info.transfer_wsol_amount = wsol_received;
        self.order_info.is_init = true;


        self.order_info.order_info_index = self.nft_mining_system.order_info_index + 1;
        self.nft_mining_system.order_info_index += 1;

        Ok(())
    }

}