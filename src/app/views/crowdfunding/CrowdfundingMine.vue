<template>
  <div>
    <top-bar :back="true" :title="$t('myNode')" />

    <van-tabs swipeable>
      <van-tab :title="$t('purchaseRecord')">
        <div class="px-[12px]">
          <div
            v-for="userPurchase in state.userPurchases"
            :key="userPurchase.phaseId"
            class="bg-card mt-[12px] rounded-[12px] p-[12px] text-[13px] text-black"
          >
            <div class="mb-[12px] flex items-center justify-between">
              <div class="text-[14px] font-bold">
                {{ $t('crowdfundingPhase', { phase: userPurchase.phaseId }) }}
              </div>

              <span
                v-if="Number(userPurchase.claimedAmount) <= Number(userPurchase.tokenAmount)"
                class="ml-[12px] text-[13px] font-bold"
                @click="handleClaimToken(userPurchase)"
              >
                {{ $t('claimToken') }} >
              </span>
            </div>

            <div class="mb-[12px] flex gap-[2px]">
              <div>{{ $t('purchaseShares') }}</div>
              <div class="font-bold">{{ userPurchase.shares }}</div>
            </div>
            <div class="mb-[12px] flex gap-[2px]">
              <div>{{ $t('vestingDays') }}</div>
              <div class="font-bold">{{ userPurchase.vestingDays }}</div>
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

            <div class="flex gap-[2px]">
              <div>{{ $t('purchaseTime') }}</div>
              <div class="font-bold">
                {{ userPurchase.purchaseTime }}
              </div>
            </div>
          </div>
        </div>
        <infinite-loading v-if="store.walletAddress" @load="handleLoad" />
      </van-tab>
      <van-tab :title="$t('applyRecord')">
        <div class="px-[12px]">
          <div
            v-for="item in state.applyRecord"
            :key="item.phaseId"
            class="bg-card mt-[12px] rounded-[12px] p-[12px] text-[13px] text-black"
          >
            <div class="mb-[12px] flex items-center justify-between">
              <div class="text-[14px] font-bold">
                {{ $t('crowdfundingPhase', { phase: item.phase_id }) }}
              </div>
              <div
                v-if="item.canPurchase && !getPurchaseInfo(item)"
                class="text-[13px] font-bold"
                @click="router.push(`/crowdfunding?purchaseLink=${item.purchase_link}`)"
              >
                {{ $t('goCrowdfunding') }} >
              </div>
            </div>
            <div class="flex gap-[2px]">
              <div>{{ $t('applyStatus') }}</div>
              <div class="font-bold">
                {{ getCrowdfundingStatus(item.status) }}
                <van-icon v-if="item.status === 1" name="checked" color="green" />
                <van-icon v-if="item.status === 2" name="clear" color="red" />
              </div>
            </div>
            <div class="mt-[12px] flex gap-[2px]">
              <div>{{ $t('applyTime') }}</div>
              <div class="font-bold">
                {{ item.created_time }}
              </div>
            </div>
            <div v-if="getPurchaseInfo(item)" class="mt-[12px] flex gap-[2px]">
              <div>{{ $t('purchaseTime') }}</div>
              <div class="font-bold">
                {{ getPurchaseInfo(item).purchaseTime }}
                <van-icon name="clock" color="green" />
              </div>
            </div>
          </div>
        </div>
        <infinite-loading v-if="store.walletAddress" @load="handleLoadApplyRecord" />
      </van-tab>
    </van-tabs>
  </div>
</template>

<script setup lang="ts">
import { showToast } from 'vant';
import { onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

import { addDividendApi, getApplicationDetailApi } from '@/app/api';
import { closeLoading, showLoading } from '@/app/components';
import { useStore } from '@/app/store';
import { claimTokens, fetchAllCrowdfundingInfo, fetchCrowdfundingInfo, UserPurchase } from '@/web3';

const { t } = useI18n();

const store = useStore();
const router = useRouter();

const getCrowdfundingStatus = (status) => {
  console.log('🚀 ~ getCrowdfundingStatus ~ status:', status);
  const statusMap = {
    0: t('pending'),
    1: t('approved'),
    2: t('rejected'),
  };

  return statusMap[status];
};

const state = reactive<any>({
  salePhases: [],
  userPurchases: [],
  crowdfundingInfo: null,
  applyRecord: [],
});

onBeforeMount(async () => {
  if (store.walletAddress) {
    return;
  }

  await store.connectWallet();
});

const getPurchaseInfo = (item) => {
  const purchaseInfo = state.userPurchases.find((i) => i.phaseId === item.phase_id);
  return purchaseInfo;
};

const handleLoadApplyRecord = async (callback?) => {
  const res = await getApplicationDetailApi({
    sol_address: store.walletAddress,
  });
  state.applyRecord = res.data;
  checkCrowdfunding();
  callback?.({ list: state.applyRecord });
};

const checkCrowdfunding = async () => {
  const crowdfundingInfo = await fetchCrowdfundingInfo();

  const item = state.applyRecord.find((item) => item.phase_id === crowdfundingInfo.phaseCount);
  if (item && item.purchase_link && item.status === 1) {
    item.canPurchase = true;
  }
};

const handleLoad = async (callback?) => {
  const { userPurchases, crowdfundingInfo, salePhases } = await fetchAllCrowdfundingInfo();

  state.userPurchases = userPurchases.filter(
    (item) => item.user.toLowerCase() === store.walletAddress.toLowerCase(),
  );
  state.crowdfundingInfo = crowdfundingInfo;
  state.salePhases = salePhases;

  callback?.({ list: state.userPurchases });
};

const handleClaimToken = async (userPurchase: UserPurchase) => {
  try {
    showLoading();
    console.log('🚀 ~ handleClaimToken ~ userPurchase:', userPurchase);
    const preClaimedAmount = Number(userPurchase.claimedAmount);
    await claimTokens(userPurchase.phaseId, userPurchase.purchaseId, userPurchase.superiorAddress);
    await handleLoad();
    const { claimedAmount: postClaimedAmount } = state.userPurchases.find(
      (item) => item.purchaseId === userPurchase.purchaseId,
    );
    const claimedAmount = Number(postClaimedAmount) - preClaimedAmount;
    addDividendApi({
      business_type: 1,
      bio_amount: claimedAmount * 0.05,
      user_address: store.walletAddress,
      business_id: Number(userPurchase.purchaseId),
      value_usdt: Number(userPurchase.phaseId),
    });
    showToast(t('success'));
  } catch (error) {
    console.error('🚀 ~ handleClaimToken ~ error:', error);
    showToast(
      error.message.includes('Nothing to claim at this time')
        ? t('noClaimableTokens')
        : error.message,
    );
  } finally {
    closeLoading();
  }
};
</script>
