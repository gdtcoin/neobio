import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/nft',
    name: 'nft',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/nft/Nft.vue'),
  },
  {
    path: '/nft/recharge-record',
    name: 'RechargeRecord',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/nft/RechargeRecord.vue'),
  },
  {
    path: '/staking',
    name: 'staking',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/staking/Staking.vue'),
  },
  {
    path: '/staking/mine',
    name: 'StakingMine',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/staking/StakingMine.vue'),
  },
  {
    path: '/crowdfunding',
    name: 'crowdfunding',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/crowdfunding/Crowdfunding.vue'),
  },
  {
    path: '/crowdfunding/mine',
    name: 'CrowdfundingMine',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/crowdfunding/CrowdfundingMine.vue'),
  },
  {
    path: '/user/crowdfunding/create',
    name: 'CrowdfundingCreate',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/crowdfunding/CrowdfundingCreate.vue'),
  },
  {
    path: '/user/crowdfunding/apply',
    name: 'CrowdfundingApply',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/crowdfunding/CrowdfundingApply.vue'),
  },
  {
    path: '/user/crowdfunding/phase/:phaseId',
    name: 'CrowdfundingPhase',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/crowdfunding/CrowdfundingPhase.vue'),
  },
  {
    path: '/user',
    name: 'user',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/user/User.vue'),
  },
  {
    path: '/user/dividend',
    name: 'dividend',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/user/Dividend.vue'),
  },
  {
    path: '/user/vesting',
    name: 'Vesting',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/user/Vesting.vue'),
  },
  {
    path: '/user/vesting/reward-record',
    name: 'VestingRewards',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/user/VestingRewards.vue'),
  },
  {
    path: '/user/vesting/create',
    name: 'VestingCreate',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/user/VestingCreate.vue'),
  },
  {
    path: '/',
    name: 'login',
    meta: {
      auth: true,
    },
    component: () => import('@/app/views/login/Login.vue'),
  },
];

export default routes;
