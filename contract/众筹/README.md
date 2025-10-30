# 众筹合约文档

## 项目概述

众筹合约是一个多期销售系统，支持用户使用USDT购买项目份额，系统自动进行代币兑换和销毁，最终获得BIONEO代币。该系统支持最多8期销售，每期100份，集成了多代币兑换、线性释放等功能。

## 合约信息

- **程序ID**: `EobpvEmfpk94NLXdoYr2JZLqc6r9RY7ijs5qqCEXnDYP`
- **网络**: Solana Devnet
- **框架**: Anchor

## 核心功能

### 1. 系统初始化

#### `initialize_crowdfunding`
初始化众筹系统，设置系统参数和代币地址。

**参数:**
- `admin`: Pubkey - 管理员地址
- `project_signer`: Pubkey - 项目方签名公钥
- `start_time`: i64 - 开始时间戳
- `wsol_mint_account`: Pubkey - WSOL代币Mint地址
- `gdtc_mint_account`: Pubkey - GDTC代币Mint地址
- `bioneo_mint_account`: Pubkey - BIONEO代币Mint地址
- `gdtc_pool_address`: Pubkey - GDTC全网分红池地址
- `gdtc_blackhole_address`: Pubkey - GDTC黑洞销毁地址

**权限:** 仅管理员可调用

### 2. 销售期管理

#### `create_phase`
创建新的销售期。

**参数:**
- `price_per_share`: u64 - 每份售价（USDT）
- `start_time`: i64 - 开始时间戳
- `id`: u64 - 期数ID

**限制:**
- 最多支持8期销售
- 每期固定100份
- 期数ID自动递增

**功能:**
- 创建新的销售期
- 设置售价和开始时间
- 初始化销售状态

### 3. 购买流程

#### `usdt_to_wsol`
使用USDT购买份额（第一步）。

**参数:**
- `shares_to_buy`: u64 - 购买的份额数量
- `phase_id`: u64 - 销售期ID
- `user_superior_address`: Pubkey - 用户上级地址

**功能:**
- 用户转入USDT
- 通过DEX兑换为WSOL
- 记录购买信息
- 建立推荐关系

#### `wsol_gdtc`
使用WSOL购买GDTC（第二步）。

**参数:**
- `shares_to_buy`: u64 - 购买的份额数量
- `phase_id`: u64 - 销售期ID

**功能:**
- 使用WSOL购买GDTC
- 销毁50%的GDTC
- 剩余50%用于后续兑换

#### `gdtc_to_bio`
将GDTC兑换为BIONEO（第三步）。

**参数:**
- `shares_to_buy`: u64 - 购买的份额数量
- `phase_id`: u64 - 销售期ID

**功能:**
- 使用剩余GDTC购买BIONEO
- 销毁50%的BIONEO
- 剩余50%分配给用户

### 4. 代币领取

#### `claim_tokens`
用户领取购买的代币。

**参数:**
- `id`: u64 - 购买记录ID
- `sold_share`: u64 - 已售份额数量

**功能:**
- 计算可领取的代币数量
- 转移BIONEO代币给用户
- 更新已领取记录
- 支持线性释放

## 数据结构

### CrowdfundingInfo (众筹信息)
```rust
pub struct CrowdfundingInfo {
    pub initialized: bool,                    // 是否已初始化
    pub authority: Pubkey,                     // 项目部署者
    pub admin: Pubkey,                         // 管理员
    pub usdt_mint_account: Pubkey,             // USDT Mint地址
    pub wsol_mint_account: Pubkey,             // WSOL Mint地址
    pub gdtc_mint_account: Pubkey,             // GDTC Token Mint地址
    pub bioneo_mint_account: Pubkey,           // BIONEO Token Mint地址
    pub total_shares: u64,                     // 总份数（800份）
    pub sold_shares: u64,                      // 已售出的份数
    pub token_per_share: u64,                  // 每份对应的Token数量（656.25）
    pub vesting_days: u64,                     // 默认线性释放天数（365天）
    pub project_signer: Pubkey,                // 项目方签名公钥
    pub phase_count: u32,                      // 已创建的销售期数量
    pub gdtc_pool_address: Pubkey,             // 全网分红池
    pub gdtc_blackhole_address: Pubkey,         // 黑洞销毁地址
}
```

### SalePhase (销售期)
```rust
pub struct SalePhase {
    pub phase_id: u32,                         // 第几期（期数ID）
    pub price_per_share: u64,                  // 每份售价（单位USDT）
    pub max_shares: u64,                       // 本期可售份数
    pub sold_shares: u64,                      // 本期已售份数
    pub start_time: i64,                       // 开始时间（Unix时间戳）
    pub end_time: i64,                         // 结束时间（Unix时间戳）
    pub active: bool,                          // 是否处于可售状态
}
```

### UserPurchase (用户购买记录)
```rust
pub struct UserPurchase {
    pub user: Pubkey,                          // 用户钱包地址
    pub superior_address: Pubkey,               // 用户上级地址
    pub phase_id: u32,                         // 所属期数
    pub purchase_id: u64,                      // 用户在该期的第N笔购买（递增编号）
    pub shares: u64,                           // 购买的份额数量
    pub token_amount: u64,                     // 对应的Token总量
    pub claimed_amount: u64,                   // 已领取的Token数量
    pub purchase_time: i64,                    // 购买时间（Unix时间戳）
    pub vesting_days: u64,                     // 该笔购买的线性释放天数
    pub wsol_amount: u64,                      // 购买到的WSOL数量
    pub gdtc_amount: u64,                      // 购买到的GDTC数量
    pub burn_gdtc: bool,                       // 是否销毁GDTC
    pub remaining_gdtc: u64,                   // 剩余GDTC数量
    pub bio_amount: u64,                       // 购买到的BIONEO数量
    pub burn_bio: bool,                        // 是否销毁BIONEO
}
```

## 常量配置

### 固定参数
- **TOTAL_SHARES**: 800 (总份数)
- **TOKEN_PER_SHARE**: 656.25 * 10^9 (每份对应的Token数量)
- **VESTING_DAYS**: 365 (默认线性释放天数)

### 种子常量
- **CROWDFUNDING_SEED**: "crowdfunding_instance" (众筹实例种子)
- **SALE_PHASE_SEED**: "sale_phase" (销售期种子)
- **USER_PURCHASE_SEED**: "user_purchase" (用户购买记录种子)
- **MINT_AUTHORITY_SEED**: "mint_authority" (Mint权限种子)
- **GDTC_ACCOUNT_SEED**: "gdtc_account" (GDTC账户种子)

### 销毁地址
- **BLACKHOLE_ADDRESS**: "11111111111111111111111111111111" (黑洞销毁地址)

## 销售期配置

| 期数 | 份数 | 说明 |
|------|------|------|
| 1-8期 | 每期100份 | 最多支持8期销售 |
| 总份数 | 800份 | 所有期数总计 |
| 每份Token | 656.25 BIONEO | 每份对应的Token数量 |

## 代币兑换流程

1. **USDT → WSOL**: 用户转入USDT，系统兑换为WSOL
2. **WSOL → GDTC**: 使用WSOL购买GDTC，销毁50%
3. **GDTC → BIONEO**: 剩余GDTC兑换BIONEO，销毁50%
4. **线性释放**: BIONEO按365天线性释放给用户

## 推荐机制

- 支持推荐关系建立
- 推荐奖励通过分红池分配
- 上级地址记录在购买记录中

## 线性释放机制

- 默认释放周期：365天
- 支持自定义释放天数
- 用户可随时领取已释放部分
- 释放进度实时计算

## 安全特性

1. **权限控制**: 关键操作需要管理员权限
2. **期数限制**: 最多8期销售，防止无限扩展
3. **代币销毁**: 自动销毁部分代币控制通胀
4. **时间验证**: 销售期时间验证
5. **状态检查**: 多重状态验证确保操作安全

## 使用流程

1. **系统初始化**: 管理员调用 `initialize_crowdfunding`
2. **创建销售期**: 管理员调用 `create_phase` 创建销售期
3. **购买份额**: 用户调用 `usdt_to_wsol` 开始购买流程
4. **代币兑换**: 系统自动执行 `wsol_gdtc` 和 `gdtc_to_bio`
5. **领取代币**: 用户调用 `claim_tokens` 领取BIONEO代币

## 错误处理

- **InvalidShareAmount**: 无效份额数量
- **PhaseNotStarted**: 销售期未开始
- **TooManyPhases**: 超过最大期数限制
- **InsufficientFunds**: 资金不足
- **AlreadyClaimed**: 已领取过

## 注意事项

- 代币兑换过程中会自动销毁50%的代币
- 每期销售固定100份，不能修改
- 最多支持8期销售
- 线性释放支持365天周期
- 所有代币数量需要考虑精度
- 推荐关系影响奖励分配
- 销售期时间不能重叠
