<template>
  <div>
    <TopBar :title="t('rechargeRecord')" />

    <div>
      <div
        v-for="item in state.orderInfo"
        :key="item.id"
        class="bg-card mx-[12px] mt-[12px] rounded-[12px] p-[16px] text-[13px] text-black"
      >
        <div class="flex items-center justify-between text-[14px] font-bold">
          <div>{{ t('rechargeRecord') }}</div>
          <div>+{{ item.investmentAmount }}USDT</div>
        </div>
        <div class="mt-[12px] flex items-center gap-[2px]">
          <div>{{ t('rechargeTime') }}</div>
          <div class="font-bold">{{ item.stakeStartTime }}</div>
        </div>

        <div class="mt-[12px] flex items-center gap-[2px]">
          <div>{{ t('currentHashrate') }}</div>
          <div class="font-bold">{{ (item.totalPower / item.investmentAmount) * 100 }}%</div>
        </div>

        <div class="mt-[12px] flex items-center gap-[2px]">
          <div>{{ t('claimableEarnings') }}</div>
          <div class="font-bold">{{ item.pendingReward }}</div>
        </div>
        <div class="mt-[12px] flex items-center gap-[2px]">
          <div>{{ t('receivedReward2') }}</div>
          <div class="font-bold">{{ item.receivedReward }}</div>
        </div>

        <div class="mt-[12px] flex items-center justify-between gap-[20px]">
          <van-button size="small" class="ghost-button w-1/2" @click="handleAddStake(item)">
            {{ t('rechargeHashrate') }}
          </van-button>
          <van-button size="small" class="black-button w-1/2" @click="handleClaimEarnings(item)">
            {{ t('claimEarnings') }}
          </van-button>
        </div>
      </div>
      <infinite-loading v-if="store.walletAddress" @load="handleLoad" />
    </div>
  </div>
</template>

<script setup lang="ts">
import dayjs from 'dayjs';
import { showToast } from 'vant';
import { onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';

import { addDividendApi, getGdtcPriceApi } from '@/app/api';
import { closeLoading, showLoading } from '@/app/components';
import { useStore } from '@/app/store';
import {
  addStake,
  claimRewards,
  config,
  fetchNftMiningSystem,
  fetchOrderInfo,
  movePointLeft,
  movePointRight,
} from '@/web3';

const { t } = useI18n();
const store = useStore();

const state = reactive({
  orderInfo: [],
  gdtcPrice: 0,
});

onBeforeMount(async () => {
  await store.connectWallet();
  loadGdtcPrice();
});

const loadGdtcPrice = async () => {
  const res = await getGdtcPriceApi();
  state.gdtcPrice = res.data.price;
};

const handleLoad = async (callback?) => {
  const nftMiningSystem = await fetchNftMiningSystem();
  const orderInfo = await fetchOrderInfo();
  console.log('🚀 ~ handleLoad ~ orderInfo:', orderInfo);
  const poolData = nftMiningSystem.pool;

  state.orderInfo = orderInfo.map((item) => {
    item.accumulatedReward = Number(item.accumulatedReward);
    item.rewardDebt = Number(item.rewardDebt);

    poolData.rewardTokenPerSec = Number(poolData.rewardTokenPerSec);
    poolData.lastRewardTimestamp = Number(poolData.lastRewardTimestamp);
    poolData.totalShares = Number(poolData.totalShares) || 1;
    const totalPower = movePointRight(item.totalPower, config.USDT_DECIMALS);

    let accShare =
      Number(
        movePointRight(
          poolData.rewardTokenPerSec * (dayjs().unix() - poolData.lastRewardTimestamp),
          12,
        ),
      ) / poolData.totalShares;

    accShare += Number(poolData.accumulatedRewardPerShare);

    item.pendingReward =
      Number(movePointLeft(totalPower * accShare, 12)) -
      Number(item.rewardDebt) +
      item.accumulatedReward;

    item.pendingReward = movePointLeft(item.pendingReward, 9);
    return {
      ...item,
      pendingReward: Number(item.pendingReward.toFixed(9)),
    };
  });
  callback?.({ list: orderInfo });
};

// 充值算力
const handleAddStake = async (item) => {
  try {
    showLoading();
    const usdtAmount = item.investmentAmount - item.totalPower;
    const gdtcAmount = usdtAmount / state.gdtcPrice;

    await addStake(item.orderInfoIndex, usdtAmount, gdtcAmount, state.gdtcPrice);
    await handleLoad();
    showToast(t('success'));
  } catch (error) {
    console.error('🚀 ~ handleAddStake ~ error:', error);
    showToast(error.message);
  } finally {
    closeLoading();
  }
};

// 领取收益
const handleClaimEarnings = async (item: any) => {
  try {
    // if (item.totalPower <= 0) {
    //   showToast(t('noClaimableEarnings'));
    //   return;
    // }
    showLoading();
    const preClaimedAmount = Number(item.receivedReward);

    await claimRewards(item.orderInfoIndex);
    await handleLoad();

    const { receivedReward: postClaimedAmount } = state.orderInfo.find(
      (i) => i.orderInfoIndex === item.orderInfoIndex,
    );
    addDividendApi({
      business_type: 3,
      bio_amount: (Number(postClaimedAmount) - preClaimedAmount) * 0.05,
      user_address: store.walletAddress,
      business_id: Number(item.orderInfoIndex),
    });
    closeLoading();
    showToast(t('success'));
    // 这里可以添加领取收益的逻辑
  } catch (error) {
    console.error('🚀 ~ handleClaimEarnings ~ error:', error);
    showToast(error.message);
  } finally {
    closeLoading();
  }
};
</script>
