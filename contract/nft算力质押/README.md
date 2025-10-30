# NFT算力质押合约文档

## 项目概述

NFT算力质押合约是一个创新的NFT挖矿系统，用户通过投资USDT获得算力，系统自动进行代币兑换和销毁，最终获得NFT奖励。该系统集成了多代币兑换、算力挖矿、NFT铸造等功能。

## 合约信息

- **程序ID**: `E58X9LwwYM9ZFhbfqMxU1zxUNhjYsS6HFd6wRvvS9XTo`
- **网络**: Solana Devnet
- **框架**: Anchor

## 核心功能

### 1. 系统初始化

#### `initialize_system`
初始化NFT算力挖矿系统，设置系统参数和代币地址。

**参数:**
- `total_supply`: u64 - 总供应量 (12,600,000 GDTC)
- `daily_output`: u64 - 每日产出 (1,726 GDTC)
- `start_timestamp`: u64 - 开始时间戳
- `pool_address`: Pubkey - 全网分红池地址
- `market_pool_address`: Pubkey - 市场分红池地址
- `gdtc_mint`: Pubkey - GDTC代币Mint地址
- `bio_mint`: Pubkey - BIONEO代币Mint地址
- `wsol_mint`: Pubkey - WSOL代币Mint地址
- `admin`: Pubkey - 管理员地址
- `black_hole_address`: Pubkey - 黑洞销毁地址

**权限:** 仅管理员可调用

### 2. 代币兑换流程

#### `usdt_wsol`
将USDT兑换为WSOL。

**参数:**
- `usdt_amount`: u64 - USDT数量

**功能:**
- 用户转入USDT
- 通过DEX兑换为WSOL
- 记录兑换数量

#### `wsol_gdtc`
使用WSOL购买GDTC并销毁一半。

**功能:**
- 使用WSOL购买GDTC
- 销毁50%的GDTC
- 剩余50%用于后续兑换

#### `gdtc_to_bio`
将GDTC兑换为BIONEO。

**功能:**
- 使用剩余GDTC购买BIONEO
- 销毁50%的BIONEO
- 剩余50%分配给用户

### 3. 算力质押

#### `enter_staking`
开始算力质押。

**功能:**
- 计算用户算力
- 开始挖矿奖励计算
- 记录质押开始时间

#### `add_stake`
增加质押算力。

**参数:**
- `reduce_amount`: u64 - 减少的算力数量
- `gdtc_amount`: u64 - 新增的GDTC数量

**功能:**
- 调整用户算力
- 更新奖励计算基准

#### `cancel_staking`
取消质押。

**参数:**
- `reduce_amount`: u64 - 减少的算力数量

**功能:**
- 减少用户算力
- 停止部分奖励计算

### 4. 奖励管理

#### `claim_rewards`
领取挖矿奖励。

**功能:**
- 计算可领取奖励
- 转移GDTC奖励给用户
- 更新奖励记录

### 5. NFT管理

#### `claim_nft`
领取NFT奖励。

**参数:**
- `nft_mint_address`: Pubkey - NFT Mint地址

**功能:**
- 铸造NFT给用户
- 记录NFT发放时间
- 更新用户NFT状态

## 数据结构

### NftMiningSystem (NFT挖矿系统)
```rust
pub struct NftMiningSystem {
    pub authority: Pubkey,                    // 管理员账户
    pub is_initialized: bool,                 // 是否初始化
    pub usdt_mint: Pubkey,                   // USDT代币Mint地址
    pub wsol_mint: Pubkey,                   // WSOL代币Mint地址
    pub gdtc_mint: Pubkey,                   // GDTC代币Mint地址
    pub bio_mint: Pubkey,                    // BIONEO代币Mint地址
    pub pool_address: Pubkey,                // 全网分红池地址
    pub market_pool_address: Pubkey,         // 市场分红池地址
    pub black_hole_address: Pubkey,          // 黑洞销毁地址
    pub admin: Pubkey,                        // 管理员
    pub total_supply: u64,                    // 总供应量
    pub daily_output: u64,                    // 每日产出
    pub start_timestamp: u64,                 // 开始时间戳
    pub pool: StakingPool,                    // 质押池信息
    pub order_info_index: u64,                // 订单信息索引
}
```

### OrderInfo (订单信息)
```rust
pub struct OrderInfo {
    pub user_address: Pubkey,                    // 用户地址
    pub order_info_index: u64,                    // 订单信息索引
    pub user_superior_account: Pubkey,           // 用户的上级Token账户
    pub total_power: u64,                        // 总算力
    pub accumulated_reward: u64,                 // 累计获得的奖励
    pub last_claim_timestamp: u64,               // 最后领取时间
    pub investment_amount: u64,                  // 投资金额(USDT)
    pub is_transfer_usdt: bool,                  // 是否已转入USDT
    pub transfer_wsol_amount: u64,               // 购买的WSOL数量
    pub is_init: bool,                           // 是否已质押
    pub stake_start_time: u64,                   // 质押开始时间
    pub reward_debt: u64,                        // 用户奖励债务
    pub is_staked: bool,                         // 用户是否已质押
    pub receivedReward: u64,                     // 已领取收益
    pub gdtc_amount: u64,                        // 购买到的GDTC数量
    pub burn_gdtc: bool,                         // 是否销毁GDTC
    pub remaining_gdtc: u64,                     // 剩余GDTC数量
    pub bio_amount: u64,                         // 购买到的BIONEO数量
    pub burn_bio: bool,                          // 是否销毁BIONEO
    pub is_nft_minted: bool,                    // 是否已发放NFT
    pub nft_minted_time: u64,                   // NFT发放时间
    pub nft_mint_address: Pubkey,               // NFT mint地址
}
```

### StakingPool (质押池)
```rust
pub struct StakingPool {
    pub reward_token_per_sec: u64,              // 每秒奖励代币数量
    pub accumulated_reward_per_share: u64,       // 累计奖励分摊
    pub last_reward_timestamp: u64,             // 上次更新奖励的时间戳
    pub total_shares: u64,                      // 该池中质押的总份额
}
```

## 常量配置

### 系统参数
- **TOTAL_SUPPLY**: 12,600,000 GDTC (总供应量)
- **DAILY_OUTPUT**: 1,726 GDTC (每日产出)
- **DAYS_IN_YEAR**: 365 (一年天数)
- **QUARTERS_IN_YEAR**: 4 (一年季度数)

### 分红比例 (基点制，10000=100%)
- **DIRECT_REFERRAL_RATE**: 5% (直接推荐奖励)
- **BIONEO_POOL_RATE**: 5% (BIONEO池奖励)
- **MARKETING_RATE**: 10% (市场推广奖励)
- **GDTC_BIONEO_RATE**: 80% (GDTC到BIONEO兑换)
- **MEMBER_DIVIDEND_RATE**: 11% (会员分红)

### 季度算力缩减
- **第1季度**: 0% (不缩减)
- **第2季度**: 25% (缩减25%)
- **第3季度**: 50% (缩减50%)
- **第4季度**: 75% (缩减75%)

### 会员等级配置
| 等级 | 投资金额 | NFT数量 | 算力比例 |
|------|----------|---------|----------|
| Tier 1 | 1 SOL | 1个 | 10% |
| Tier 2 | 3 SOL | 3个 | 30% |
| Tier 3 | 5 SOL | 5个 | 50% |
| Tier 4 | 10 SOL | 10个 | 100% |

## 代币兑换流程

1. **USDT → WSOL**: 用户转入USDT，系统兑换为WSOL
2. **WSOL → GDTC**: 使用WSOL购买GDTC，销毁50%
3. **GDTC → BIONEO**: 剩余GDTC兑换BIONEO，销毁50%
4. **NFT铸造**: 根据投资金额铸造对应数量的NFT

## 算力计算

- 算力基于投资金额计算
- 每季度算力会按比例缩减
- 算力决定挖矿奖励分配
- 支持动态调整算力

## 奖励机制

- **挖矿奖励**: 基于算力和时间计算GDTC奖励
- **推荐奖励**: 5%的直接推荐奖励
- **分红奖励**: 全网分红池和市场分红池奖励
- **NFT奖励**: 根据投资等级获得对应NFT

## 安全特性

1. **权限控制**: 关键操作需要管理员权限
2. **代币销毁**: 自动销毁部分代币控制通胀
3. **时间锁定**: NFT铸造有时间限制
4. **状态验证**: 多重状态检查确保操作安全

## 使用流程

1. **系统初始化**: 管理员调用 `initialize_system`
2. **投资USDT**: 用户调用 `usdt_wsol` 转入USDT
3. **代币兑换**: 系统自动执行 `wsol_gdtc` 和 `gdtc_to_bio`
4. **开始挖矿**: 用户调用 `enter_staking` 开始算力挖矿
5. **领取奖励**: 用户调用 `claim_rewards` 领取GDTC奖励
6. **领取NFT**: 用户调用 `claim_nft` 领取NFT奖励

## 注意事项

- 代币兑换过程中会自动销毁50%的代币
- NFT铸造需要满足投资等级要求
- 算力每季度会按比例缩减
- 所有代币数量需要考虑精度
- 推荐关系影响奖励分配
