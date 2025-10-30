<template>
  <div>
    <top-bar :back="true" :title="$t('myStaked')" />

    <div class="px-[12px]">
      <div
        v-for="item in stakedInfo"
        :key="item.stakedIndex"
        class="bg-card mt-[12px] rounded-[12px] p-[16px] text-[13px] text-black"
      >
        <div class="mb-[12px] flex items-center justify-between text-[14px] font-bold">
          <div>
            {{ $t('stakingLPAmount', { amount: item.depositedAmount }) }}({{
              timeOptions[item.stakeType].label
            }})
          </div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('redeemedAmount') }}</div>
          <div class="font-bold">{{ item.receivedReward }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('pendingAmount') }}</div>
          <div class="font-bold">{{ item.pendingReward }}</div>
        </div>

        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('stakingTime') }}</div>
          <div class="font-bold">
            {{ item.stakeStartTime }}
          </div>
        </div>

        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('redeemTime') }}</div>
          <div class="font-bold">
            {{ item.stakeEndTime }}
          </div>
        </div>

        <div class="flex justify-between gap-[20px]">
          <van-button
            class="ghost-button w-1/2"
            size="small"
            plain
            @click="handleCancelStaking(item)"
          >
            {{ $t('redeem') }}
          </van-button>
          <van-button
            class="black-button w-1/2"
            size="small"
            :disabled="!item.pendingReward || item.canCancelStake"
            @click="handleClaimToken(item)"
          >
            {{ $t('claim') }}
          </van-button>
        </div>
      </div>
    </div>
    <infinite-loading v-if="store.walletAddress" @load="handleLoad" />
  </div>
</template>

<script setup lang="ts">
import dayjs from 'dayjs';
import { showToast } from 'vant';
import { computed, onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';

import { addDividendApi } from '@/app/api';
import { closeLoading, showLoading } from '@/app/components';
import { useStore } from '@/app/store';
import {
  cancelStaking,
  claimStakingRewards,
  fetchStakingPools,
  fetchUserStakedInfo,
  movePointLeft,
  movePointRight,
  StakingInstance,
  User,
} from '@/web3';

const { t } = useI18n();

const timeOptions = computed(() => [
  { label: t('threeMonths'), value: 0 },
  { label: t('sixMonths'), value: 1 },
  { label: t('twelveMonths'), value: 2 },
]);

const store = useStore();

const state = reactive({
  stakingInfo: {} as StakingInstance,
  userStakedInfo: {
    stakedInfo: [],
  } as User,
});

onBeforeMount(async () => {
  if (store.walletAddress) {
    return;
  }

  await store.connectWallet();
});

const stakedInfo = computed(() => {
  return state.userStakedInfo.stakedInfo;
});

const handleLoad = async (callback?) => {
  let [stakingInfo, userStakedInfo] = await Promise.all([
    fetchStakingPools(),
    fetchUserStakedInfo(),
  ]);
  console.log('🚀 ~ handleLoad ~ userStakedInfo:', userStakedInfo);
  userStakedInfo.stakedInfo = userStakedInfo.stakedInfo
    .filter((item) => item.isStaked)
    .map((item, index) => ({
      ...item,
      stakedIndex: index,
    }))
    .reverse()
    .map((item) => {
      item.stakeEndTime = dayjs(Number(item.stakeEndTime) * 1000).format('YYYY-MM-DD HH:mm:ss');
      item.stakeStartTime = dayjs(Number(item.stakeStartTime) * 1000).format('YYYY-MM-DD HH:mm:ss');
      item.depositedAmount = movePointLeft(item.depositedAmount, 9);
      item.receivedReward = movePointLeft(item.receivedReward, 9);
      item.accumulatedReward = movePointLeft(item.accumulatedReward, 9);
      item.rewardDebt = movePointLeft(item.rewardDebt, 9);

      const poolData = stakingInfo.pools.find((pool) => pool.stakeType === item.stakeType);
      poolData.rewardTokenPerSec = Number(poolData.rewardTokenPerSec);
      poolData.lastRewardTimestamp = Number(poolData.lastRewardTimestamp);
      poolData.totalShares = Number(poolData.totalShares);
      console.log('🚀 ~ handleLoad ~ poolData:', poolData);

      let accShare =
        Number(
          movePointRight(
            poolData.rewardTokenPerSec * (dayjs().unix() - poolData.lastRewardTimestamp),
            12,
          ),
        ) / poolData.totalShares;
      console.log('🚀 ~ handleLoad ~ accShare:', accShare);

      accShare += Number(poolData.accumulatedRewardPerShare);
      console.log('🚀 ~ handleLoad ~ accShare:', accShare);
      item.pendingReward = item.canCancelStake
        ? 0
        : Number(movePointLeft(item.depositedAmount * accShare, 12)) - Number(item.rewardDebt);
      console.log(movePointLeft(item.depositedAmount * accShare, 12), Number(item.rewardDebt));
      return {
        ...item,
        pendingReward: Number(item.pendingReward.toFixed(9)),
      };
    });

  state.stakingInfo = stakingInfo;
  state.userStakedInfo = userStakedInfo;

  callback?.({ list: userStakedInfo.stakedInfo });
};

const handleClaimToken = async (item) => {
  console.log('🚀 ~ handleClaimToken ~ item:', item.stakedIndex);
  try {
    showLoading();
    const preClaimedAmount = Number(item.receivedReward);
    await claimStakingRewards(item.stakedIndex, state.userStakedInfo.userSuperiorAccount);
    await handleLoad();
    const { receivedReward: postClaimedAmount } = state.userStakedInfo.stakedInfo.find(
      (i) => i.stakedIndex === item.stakedIndex,
    );
    const claimedAmount = Number(postClaimedAmount) - preClaimedAmount;
    addDividendApi({
      business_type: 4,
      bio_amount: claimedAmount * 0.05,
      user_address: store.walletAddress,
      business_id: item.stakedIndex,
    });
    showToast(t('success'));
  } catch (error) {
    console.error('🚀 ~ handleClaimToken ~ error:', error);
    showToast(t('noClaimableTokens'));
  } finally {
    closeLoading();
  }
};

const handleCancelStaking = async (item) => {
  try {
    if (!item.canCancelStake) {
      showToast(t('stake_not_expired'));
      return;
    }

    showLoading();
    await cancelStaking(item.stakedIndex);
    await handleLoad();
    showToast(t('success'));
  } catch (error) {
    console.error('🚀 ~ handleClaimToken ~ error:', error);
    showToast(error.message);
  } finally {
    closeLoading();
  }
};
</script>
