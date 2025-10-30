# LP质押合约文档

## 项目概述

LP质押合约是一个基于Solana区块链的流动性代币质押系统，支持用户质押LP代币获得奖励。合约支持多种质押期限（3个月、6个月、12个月），并提供相应的奖励机制。

## 合约信息

- **程序ID**: `4omXPE38UJfWj7rSJjUdbUHH4U9bLB5UR77pY1BjR9pn`
- **网络**: Solana Devnet
- **框架**: Anchor

## 核心功能

### 1. 系统初始化

#### `initialize_staking`
初始化质押系统，设置奖励参数和质押池配置。

**参数:**
- `reward_per_sec_3_months`: u64 - 3个月质押池每秒奖励数量
- `reward_per_sec_6_months`: u64 - 6个月质押池每秒奖励数量  
- `reward_per_sec_12_months`: u64 - 12个月质押池每秒奖励数量
- `start_reward_timestamp`: u64 - 奖励开始时间戳
- `gdtc_pool_address`: Pubkey - GDTC全网分红池地址

**权限:** 仅管理员可调用

### 2. 用户管理

#### `initialize_user`
初始化用户账户，设置用户上级推荐关系。

**参数:**
- `_user_superior_account`: Pubkey - 用户上级账户地址

**功能:**
- 创建用户质押账户
- 建立推荐关系
- 初始化用户质押信息数组

### 3. 质押操作

#### `enter_staking`
用户进行LP代币质押。

**参数:**
- `lp_staking_number`: u64 - 质押的LP代币数量
- `stake_type`: u64 - 质押类型 (0=3个月, 1=6个月, 2=12个月)
- `staked_info_index`: u64 - 质押信息索引位置

**功能:**
- 转移LP代币到合约
- 记录质押开始时间
- 计算质押结束时间
- 更新用户质押状态

#### `cancel_staking`
取消质押（提前解质押）。

**参数:**
- `staked_info_index`: u64 - 要取消的质押信息索引

**功能:**
- 检查是否允许取消质押
- 计算已获得奖励
- 返还LP代币给用户
- 更新质押状态

### 4. 奖励领取

#### `claim_rewards`
领取质押奖励。

**参数:**
- `staked_info_index`: u64 - 质押信息索引

**功能:**
- 计算可领取奖励数量
- 转移奖励代币给用户
- 更新已领取奖励记录
- 重置奖励债务

## 数据结构

### StakingInstance (质押实例)
```rust
pub struct StakingInstance {
    pub authority: Pubkey,          // 管理员账户
    pub is_initialized: bool,       // 是否初始化
    pub reward_token_mint: Pubkey,  // 奖励代币Mint地址
    pub staking_token_mint: Pubkey, // 质押代币Mint地址
    pub secend_reward_token_mint: Pubkey, // 第二个奖励代币（如GDTC）
    pub pools: [StakingPool; 3],    // 固定3个质押池
    pub gdtc_pool_address: Pubkey,  // 全网分红池地址
}
```

### StakingPool (质押池)
```rust
pub struct StakingPool {
    pub stake_type: u64,                    // 质押类型 (0,1,2)
    pub reward_token_per_sec: u64,          // 每秒奖励代币数量
    pub accumulated_reward_per_share: u64,   // 累计奖励分摊
    pub last_reward_timestamp: u64,         // 上次更新奖励的时间戳
    pub total_shares: u64,                  // 该池中质押的总份额
}
```

### User (用户信息)
```rust
pub struct User {
    pub total_deposited_amount: u64,    // 用户总存入的质押金额
    pub user_superior_account: Pubkey,  // 用户的上级Token账户
    pub staked_info: [Staked; 10],      // 固定10个质押池
    pub isinit: bool,                   // 是否已初始化
    pub user_address: Pubkey,           // 用户地址
}
```

### Staked (质押信息)
```rust
pub struct Staked {
    pub deposited_amount: u64,      // 用户总存入的质押金额
    pub reward_debt: u64,           // 用户奖励债务
    pub accumulated_reward: u64,    // 用户累计获得的奖励
    pub is_staked: bool,            // 用户是否已质押
    pub stake_type: u64,            // 质押类型
    pub stake_start_time: u64,      // 质押开始时间
    pub stake_end_time: u64,        // 质押结束时间
    pub receivedReward: u64,       // 已领取收益
    pub can_cancel_stake: bool,    // 是否可以解除质押
}
```

## 常量配置

- **TOKEN_PROGRAM_BYTES**: Token程序地址
- **NFT_TOKEN_PROGRAM_BYTES**: NFT Token程序地址
- **COMPUTATION_DECIMALS**: 算力精度 (10^12)
- **STAKING_SEED**: 质押实例种子
- **LPTOKEN_SEED**: LP代币种子
- **REWARD_CLAIM_COOLDOWN**: 奖励领取冷却时间 (3600秒)

## 质押类型说明

| 类型 | 期限 | 说明 |
|------|------|------|
| 0 | 3个月 | 短期质押，奖励相对较低 |
| 1 | 6个月 | 中期质押，奖励中等 |
| 2 | 12个月 | 长期质押，奖励最高 |

## 安全特性

1. **权限控制**: 关键操作需要管理员权限
2. **时间锁定**: 质押期间不能随意取消
3. **奖励计算**: 基于时间加权平均的奖励分配
4. **状态检查**: 多重状态验证确保操作安全

## 使用流程

1. **系统初始化**: 管理员调用 `initialize_staking` 设置系统参数
2. **用户注册**: 用户调用 `initialize_user` 创建账户
3. **开始质押**: 用户调用 `enter_staking` 进行质押
4. **领取奖励**: 用户调用 `claim_rewards` 领取奖励
5. **取消质押**: 用户调用 `cancel_staking` 提前解质押

## 注意事项

- 质押期间不能随意取消，需要满足特定条件
- 奖励计算基于时间加权，提前解质押可能影响奖励
- 用户最多可以同时进行10个质押操作
- 所有时间戳使用Unix时间戳格式
- 代币数量需要考虑精度（通常为10^9或10^12）
