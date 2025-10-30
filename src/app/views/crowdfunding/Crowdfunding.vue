<template>
  <div>
    <top-bar back :title="$t('crowdfunding')">
      <!-- <template #left>
        <div @click="$router.push('/crowdfunding/create')">
          {{ $t('createCrowdfunding') }}
        </div>
      </template> -->

      <!-- <template #left>
        <div @click="$router.push('/crowdfunding/mine')">
          {{ $t('myNode') }}
        </div>
      </template> -->
    </top-bar>

    <!-- <div class="m-[12px] flex justify-between rounded-[8px] bg-white p-[12px]">
      <div class="text-[13px]">{{ t('myBioneo') }}</div>

      <div class="flex items-center gap-[8px]">
        <div class="text-[13px]">{{ state.bioneoBalance }}</div>
      </div>
    </div> -->

    <div class="px-[12px]">
      <div
        v-for="salePhase in state.salePhases"
        :key="salePhase.phaseId"
        class="bg-card mt-[12px] rounded-[12px] p-[20px] text-black"
      >
        <div class="mb-[12px] text-center text-[15px] font-bold">
          {{ $t('crowdfundingPhase', { phase: salePhase.phaseId }) }}
        </div>

        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('crowdfundingShares') }}</div>
          <div class="font-bold">{{ salePhase.maxShares }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('crowdfundingSoldShares') }}</div>
          <div class="font-bold">
            {{ Number(salePhase.soldShares) }}
          </div>
        </div>
        <div class="mb-[16px] flex gap-[2px]">
          <div>{{ $t('crowdfundingPricePerShare') }}</div>
          <div class="font-bold">{{ salePhase.pricePerShare }} USDT</div>
        </div>
        <div class="mb-[16px] flex gap-[2px]">
          <div>{{ $t('crowdfundingStartTime') }}</div>
          <div class="font-bold">
            {{ formatTime(salePhase.startTime, 'YYYY-MM-DD 00:00:00') }}
          </div>
        </div>
        <div class="mb-[24px] flex gap-[2px]">
          <div>{{ $t('crowdfundingParticipatedShares') }}</div>
          <div class="font-bold">{{ 1 }}</div>
        </div>

        <van-button
          class="go-crowdfunding"
          round
          block
          :disabled="!salePhase.active"
          @click="handleConfirm"
        >
          {{ $t('goCrowdfunding') }}
        </van-button>
      </div>
    </div>
    <infinite-loading v-if="store.walletAddress" noMoreText=" " @load="handleLoad" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from 'vant';
import { onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';

import { getApplicationDetailApi } from '@/app/api';
import { closeLoading, showLoading } from '@/app/components';
import { useStore } from '@/app/store';
import { formatTime } from '@/app/utils/format';
import { fetchAllCrowdfundingInfo, getBioneoBalance, purchaseShare } from '@/web3';

const route = useRoute();

const purchaseLink = route.query.purchaseLink as string;

const store = useStore();
const router = useRouter();
const { t } = useI18n();
const state = reactive({
  salePhases: [],
  userPurchases: [],
  crowdfundingInfo: null,
  bioneoBalance: 0,
  usdtBalance: 0,
  amount: 1,
  purchaseLink: '',
});

onBeforeMount(async () => {
  await store.connectWallet();
  fetchData();
});

const fetchData = async () => {
  const [bioneoBalance] = await Promise.all([getBioneoBalance()]);
  state.bioneoBalance = bioneoBalance;
};

// const getMyShares = (salePhase: SalePhase) => {
//   return state.userPurchases
//     .filter((item) => item.phaseId === salePhase.phaseId && item.user === store.walletAddress)
//     .reduce((acc, item) => acc + Number(item.shares), 0);
// };

const handleLoad = async (callback?) => {
  const { salePhases, userPurchases, crowdfundingInfo } = await fetchAllCrowdfundingInfo();
  await checkCrowdfunding(crowdfundingInfo);

  state.salePhases = salePhases?.slice(0, 1) || [];
  state.userPurchases = userPurchases;
  state.crowdfundingInfo = crowdfundingInfo;

  console.log('🚀 ~ salePhases:', state.salePhases);
  console.log('🚀 ~ userPurchases:', state.userPurchases);
  console.log('🚀 ~ crowdfundingInfo:', state.crowdfundingInfo);

  fetchData();
  callback?.({ list: state.salePhases });
};

const handleConfirm = async () => {
  const { phaseId, soldShares } = state.salePhases[0];

  if (!state.purchaseLink) {
    showToast(t('noPermission'));
    return;
  }

  try {
    showLoading();
    await purchaseShare(state.amount, phaseId, soldShares, state.purchaseLink);
    await handleLoad();
    closeLoading();
    showToast(t('success'));
    router.go(-1);
  } catch (error) {
    console.error(error);
    showToast(error.message);
    closeLoading();
  }
};

const checkCrowdfunding = async (crowdfundingInfo) => {
  try {
    const res = await getApplicationDetailApi({
      sol_address: store.walletAddress,
    });
    const find = res.data.find((item) => item.phase_id === crowdfundingInfo.phaseCount);
    if (find?.purchase_link === purchaseLink) {
      state.purchaseLink = purchaseLink;
    }
  } catch (error) {
    console.log('🚀 ~ checkCrowdfunding ~ error:', error);
  }
};
</script>

<style lang="less" scoped>
.go-crowdfunding {
  font-weight: bold;
  color: var(--van-white-color);
  background-color: transparent;
  border-color: black;
}
</style>
