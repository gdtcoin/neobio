<template>
  <div>
    <top-bar :back="true" :title="$t('crowdfundingPhase', { phase: route.params.phaseId })" />

    <div class="px-[12px]">
      <div
        v-for="userPurchase in state.userPurchases"
        :key="userPurchase.phaseId"
        class="bg-card mt-[12px] rounded-[12px] p-[12px] text-[13px] text-black"
      >
        <div class="mb-[12px] flex">
          <div>{{ $t('participatingWallet') }}</div>
          <div class="flex items-center gap-[6px] font-bold" @click="copy(userPurchase.user)">
            {{ formatWallet(userPurchase.user) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('purchaseShares') }}</div>
          <div class="font-bold">{{ userPurchase.shares }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('tokenAmount') }}</div>
          <div class="font-bold">{{ userPurchase.tokenAmount }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('claimableAmount') }}</div>
          <div class="font-bold">{{ userPurchase.claimableAmount }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('claimedAmount') }}</div>
          <div class="font-bold">{{ userPurchase.claimedAmount }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('vestingDays') }}</div>
          <div class="font-bold">{{ userPurchase.vestingDays }}</div>
        </div>

        <div class="flex gap-[2px]">
          <div>{{ $t('purchaseTime') }}</div>
          <div class="font-bold">
            {{ userPurchase.purchaseTime }}
          </div>
        </div>
      </div>
    </div>
    <infinite-loading v-if="store.walletAddress" @load="handleLoad" />
  </div>
</template>

<script setup lang="ts">
import { onBeforeMount, reactive } from 'vue';
import { useRoute } from 'vue-router';

import { useClipboard } from '@/app/hooks';
import { useStore } from '@/app/store';
import { formatWallet } from '@/app/utils';
import { AllCrowdfundingInfo, fetchAllCrowdfundingInfo } from '@/web3';

const route = useRoute();
const store = useStore();
const { copy } = useClipboard();

const state = reactive<AllCrowdfundingInfo>({
  salePhases: [],
  userPurchases: [],
  crowdfundingInfo: null,
});

onBeforeMount(async () => {
  if (store.walletAddress) {
    return;
  }

  await store.connectWallet();
});

const handleLoad = async (callback?) => {
  const { userPurchases, crowdfundingInfo, salePhases } = await fetchAllCrowdfundingInfo();

  state.userPurchases = userPurchases.filter(
    (item) => item.phaseId === Number(route.params.phaseId),
  );
  state.crowdfundingInfo = crowdfundingInfo;
  state.salePhases = salePhases;

  callback?.({ list: state.userPurchases });
};
</script>
