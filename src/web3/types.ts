export type CrowdfundingInfo = {
  initialized: boolean;
  authority: string;
  admin: string;
  usdtInAccount: string;
  usdtMintAccount: string;
  tokenMintAccount: string;
  totalShares: number;
  soldShares: number;
  tokenPerShare: number;
  vestingDays: number;
  projectSigner: string;
  phaseCount: number;
  wsolMintAccount: string;
  gdtcBlackholeAddress: string;
  gdtcPoolAddress: string;
};

export type SalePhase = {
  phaseId: number;
  pricePerShare: number;
  maxShares: number;
  soldShares: number;
  startTime: number;
  endTime: number;
  active: boolean;
};

export type UserPurchase = {
  user: string;
  phaseId: number;
  purchaseId: number;
  shares: number;
  tokenAmount: number;
  claimedAmount: number;
  purchaseTime: number;
  vestingDays: number;
  superiorAddress: string;
  claimableAmount: number | string;
};

export type AllCrowdfundingInfo = {
  crowdfundingInfo: CrowdfundingInfo;
  salePhases: SalePhase[];
  userPurchases: UserPurchase[];
};

export type StakedInfo = {
  depositedAmount: number;
  rewardDebt: number;
  accumulatedReward: number;
  isStaked: boolean;
  stakeType: number;
  stakeStartTime: string;
  stakeEndTime: string;
  receivedReward: number;
  canCancelStake: boolean;
  pendingReward?: number;
  stakedIndex?: number;
};

export type StakingPool = {
  stakeType: number;
  rewardTokenPerSec: number;
  accumulatedRewardPerShare: number;
  lastRewardTimestamp: number;
  totalShares: number;
};

export type StakingInstance = {
  authority: string;
  isInitialized: boolean;
  rewardTokenMint: string;
  stakingTokenMint: string;
  secendRewardTokenMint: string;
  pools: StakingPool[];
  gdtcPoolAddress: string;
};

export type User = {
  totalDepositedAmount: number;
  userSuperiorAccount: string;
  stakedInfo: StakedInfo[];
  isinit: boolean;
  userAddress: string;
};

// Vesting相关类型定义
export type VestingScheduleInfo = {
  creator: string;
  beneficiary: string;
  mint: string;
  totalAmount: number;
  claimedAmount: number;
  startTime: number;
  vestingPeriod: number;
  periodCount: number;
  amountPerPeriod: number;
  createdAt: number;
};

export type VestingInfo = {
  creator: string;
  beneficiary: string;
  mint: string;
  totalAmount: number;
  claimedAmount: number;
  claimableAmount: number;
  lockedAmount: number;
  startTime: number;
  vestingPeriod: number;
  periodCount: number;
  amountPerPeriod: number;
  completedPeriods: number;
  createdAt: number;
  progress: number;
  isFullyVested: boolean;
  nextReleaseTime: number | null;
};

export enum VestingPeriod {
  Daily = 'daily',
  Monthly = 'monthly',
  Yearly = 'yearly',
  Linear = 'linear',
}
