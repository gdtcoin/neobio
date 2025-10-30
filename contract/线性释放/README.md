# 线性释放合约文档

## 项目概述

线性释放合约是一个灵活的代币释放系统，支持多种释放模式（按天、按月、按年、线性释放）。用户可以为受益人创建释放计划，支持周期性释放和连续线性释放两种模式。

## 合约信息

- **程序ID**: `4F6YPEcRVbBUrjSFFAJgjHRyJUZwovLhti8jRC45WRgP`
- **网络**: Solana Devnet
- **框架**: Anchor

## 核心功能

### 1. 创建释放计划

#### `create_vesting_schedule`
创建新的代币释放计划。

**参数:**
- `total_amount`: u64 - 总释放金额
- `start_time`: i64 - 开始时间戳
- `vesting_period`: VestingPeriod - 释放周期类型
- `period_count`: u32 - 释放周期数量

**释放周期类型:**
- `Daily`: 按天释放
- `Monthly`: 按月释放  
- `Yearly`: 按年释放
- `Linear`: 线性释放（按秒）

**功能:**
- 创建释放计划账户
- 设置释放参数
- 转移代币到合约
- 计算每周期释放金额

### 2. 代币提取

#### `claim`
提取已释放的代币。

**功能:**
- 计算可提取金额
- 转移代币给受益人
- 更新已提取记录
- 支持部分提取

### 3. 查询功能

#### `get_claimable_amount`
查询当前可提取的代币数量。

**返回值:** u64 - 可提取金额

**功能:**
- 实时计算可提取金额
- 考虑已提取部分
- 支持不同释放模式

#### `get_vesting_info`
查询释放计划的详细信息。

**返回值:** VestingInfo - 释放计划信息

**功能:**
- 返回完整的释放计划信息
- 包含当前状态和进度
- 支持前端展示

### 4. 计划管理

#### `cancel_vesting`
取消释放计划（仅创建者可用）。

**功能:**
- 检查创建者权限
- 返还剩余代币给创建者
- 标记计划为已取消
- 支持紧急情况处理

## 数据结构

### VestingSchedule (释放计划)
```rust
pub struct VestingSchedule {
    pub creator: Pubkey,                       // 创建者
    pub beneficiary: Pubkey,                    // 受益人
    pub mint: Pubkey,                          // 代币mint
    pub total_amount: u64,                     // 总金额
    pub claimed_amount: u64,                   // 已提取金额
    pub start_time: i64,                       // 开始时间（Unix时间戳）
    pub vesting_period: VestingPeriod,         // 释放周期类型
    pub period_count: u32,                     // 释放周期数量
    pub amount_per_period: u64,                // 每个周期释放的金额
    pub created_at: i64,                       // 创建时间
}
```

### VestingPeriod (释放周期枚举)
```rust
pub enum VestingPeriod {
    Daily,     // 按天释放 (86400秒)
    Monthly,   // 按月释放 (30天)
    Yearly,    // 按年释放 (365天)
    Linear,    // 线性释放 (按秒)
}
```

## 释放模式详解

### 1. 周期性释放

#### 按天释放 (Daily)
- 周期长度：24小时 (86,400秒)
- 适用场景：短期释放计划
- 释放方式：每天固定金额

#### 按月释放 (Monthly)  
- 周期长度：30天 (2,592,000秒)
- 适用场景：中期释放计划
- 释放方式：每月固定金额

#### 按年释放 (Yearly)
- 周期长度：365天 (31,536,000秒)
- 适用场景：长期释放计划
- 释放方式：每年固定金额

### 2. 线性释放 (Linear)
- 周期长度：1秒
- 适用场景：连续释放
- 释放方式：按时间比例连续释放

## 计算方法

### 周期性释放计算
```
可提取金额 = min(已完成周期数 × 每周期金额, 总金额) - 已提取金额
```

### 线性释放计算
```
可提取金额 = (总金额 × 已过去时间) / 总释放时间 - 已提取金额
```

### 释放进度计算
```
进度百分比 = (已完成周期数 / 总周期数) × 100
```

## 核心方法

### 获取总释放持续时间
```rust
pub fn get_total_duration(&self) -> i64 {
    let period_seconds = self.vesting_period.to_seconds();
    period_seconds * self.period_count as i64
}
```

### 获取已完成周期数
```rust
pub fn get_completed_periods(&self, current_time: i64) -> u32 {
    if current_time < self.start_time {
        return 0;
    }
    let elapsed_time = current_time - self.start_time;
    let period_seconds = self.vesting_period.to_seconds();
    let completed_periods = elapsed_time / period_seconds;
    completed_periods.min(self.period_count as i64) as u32
}
```

### 获取可提取金额
```rust
pub fn get_claimable_amount(&self, current_time: i64) -> Result<u64> {
    // 线性释放：按时间比例计算
    // 周期性释放：按完成的周期数计算
    // 返回：已释放金额 - 已提取金额
}
```

### 检查是否完全释放
```rust
pub fn is_fully_vested(&self, current_time: i64) -> bool {
    // 线性释放：检查是否超过总释放时间
    // 周期性释放：检查是否完成所有周期
}
```

### 获取释放进度
```rust
pub fn get_vesting_progress(&self, current_time: i64) -> u8 {
    // 返回0-100的进度百分比
}
```

### 获取下次释放时间
```rust
pub fn get_next_release_time(&self, current_time: i64) -> Option<i64> {
    // 线性释放：返回None（连续释放）
    // 周期性释放：返回下次释放时间戳
}
```

## 使用场景

### 1. 员工股权激励
- 创建4年期线性释放计划
- 员工可随时提取已释放部分
- 支持离职后继续释放

### 2. 项目代币分发
- 创建按月释放计划
- 投资者定期获得代币
- 支持多期释放

### 3. 社区奖励
- 创建按天释放计划
- 用户每日领取奖励
- 支持动态调整

### 4. 长期投资
- 创建按年释放计划
- 投资者年度获得收益
- 支持提前取消

## 安全特性

1. **权限控制**: 只有创建者可以取消计划
2. **时间验证**: 严格的时间戳验证
3. **数学安全**: 防止溢出和下溢
4. **状态检查**: 多重状态验证
5. **代币安全**: 代币锁定在合约中

## 使用流程

1. **创建计划**: 调用 `create_vesting_schedule` 创建释放计划
2. **查询状态**: 调用 `get_vesting_info` 查看计划详情
3. **提取代币**: 调用 `claim` 提取已释放代币
4. **查询余额**: 调用 `get_claimable_amount` 查询可提取金额
5. **取消计划**: 调用 `cancel_vesting` 取消计划（可选）

## 错误处理

- **MathOverflow**: 数学计算溢出
- **VestingNotStarted**: 释放计划未开始
- **NothingToClaim**: 没有可提取的代币
- **Unauthorized**: 无权限操作
- **InvalidAmount**: 无效金额

## 注意事项

- 所有时间戳使用Unix时间戳格式
- 代币数量需要考虑精度
- 线性释放支持连续提取
- 周期性释放按周期计算
- 创建者可以随时取消计划
- 受益人只能提取，不能取消
- 支持部分提取，不强制一次性提取完
