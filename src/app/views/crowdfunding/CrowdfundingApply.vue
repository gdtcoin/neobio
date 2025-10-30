<template>
  <div>
    <top-bar :back="true" :title="$t('nodeApply')" />

    <div class="px-[12px]">
      <div
        v-for="item in state.list"
        :key="item.phaseId"
        class="bg-card mt-[12px] rounded-[12px] p-[12px] text-[13px] text-black"
      >
        <div class="mb-[12px] flex items-center justify-between">
          <div class="text-[14px] font-bold">
            {{ $t('crowdfundingPhase', { phase: item.phase_id }) }}
          </div>
          <div
            v-if="item.canPurchase"
            class="text-[13px] font-bold"
            @click="router.push(`/crowdfunding`)"
          >
            {{ $t('goCrowdfunding') }} >
          </div>
        </div>
        <div class="flex gap-[2px]" @click="copy(item.sol_address)">
          <div>{{ $t('applyWallet') }}</div>
          <div class="font-bold">
            {{ formatWallet(item.sol_address) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
        </div>

        <div class="mt-[12px] flex gap-[2px]">
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
            {{ dayjs(item.created_time).format('YYYY-MM-DD HH:mm:ss') }}
          </div>
        </div>

        <div v-if="item.status === 1 && getPurchaseInfo(item)" class="mt-[12px] flex gap-[2px]">
          <div>{{ $t('purchaseTime') }}</div>
          <div class="font-bold">
            {{ getPurchaseInfo(item).purchaseTime }}
            <van-icon name="clock" color="green" />
          </div>
        </div>

        <div
          v-if="item.status === 0"
          class="mt-[12px] flex items-center justify-between gap-[20px]"
        >
          <van-button size="small" class="ghost-button w-1/2" @click="handleReview(item, false)">
            {{ t('reject') }}
          </van-button>
          <van-button size="small" class="black-button w-1/2" @click="handleReview(item, true)">
            {{ t('approve') }}
          </van-button>
        </div>
      </div>
    </div>
    <infinite-loading v-if="store.walletAddress" @load="handleLoadApplyRecord" />
  </div>
</template>

<script setup lang="ts">
import dayjs from 'dayjs';
import { showToast } from 'vant';
import { onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

import { fetchCrowdfundingApplyListApi, reviewCrowdfundingApplyApi } from '@/app/api';
import { closeLoading, showLoading } from '@/app/components';
import { useClipboard } from '@/app/hooks';
import { useStore } from '@/app/store';
import { formatWallet } from '@/app/utils';
import { fetchUserPurchases } from '@/web3';

const { t } = useI18n();

const store = useStore();
const router = useRouter();
const { copy } = useClipboard();

const getCrowdfundingStatus = (status) => {
  console.log('🚀 ~ getCrowdfundingStatus ~ status:', status);
  const statusMap = {
    0: t('pending'),
    1: t('approved'),
    2: t('rejected'),
  };

  return statusMap[status];
};
const getPurchaseInfo = (item) => {
  const purchaseInfo = state.userPurchases.find(
    (i) => i.phaseId === item.phase_id && i.user === item.sol_address,
  );
  return purchaseInfo;
};

const state = reactive<any>({
  list: [],
  phaseId: null,
  page: 1,
  pageSize: 10,
  userPurchases: [],
});

onBeforeMount(async () => {
  if (store.walletAddress) {
    return;
  }

  await store.connectWallet();
});

const handleLoadApplyRecord = async (callback?) => {
  const res = await fetchList();
  state.list = res.data.data;
  callback?.({ list: res.data.data, pageSize: state.pageSize, pageNum: state.page });
  state.page++;
};

const fetchList = async (page = state.page, pageSize = state.pageSize) => {
  const res = await fetchCrowdfundingApplyListApi({
    sol_address: store.walletAddress,
    page,
    page_size: pageSize,
  });
  await fetchUserPurchasesList();

  return res;
};

const fetchUserPurchasesList = async () => {
  const res = await fetchUserPurchases();
  console.log(res);
  state.userPurchases = res;
};

const handleRefresh = async () => {
  state.page = 1;
  const res = await fetchList();
  state.list = res.data.data;
};

const handleReview = async (item, approve) => {
  try {
    showLoading();
    await reviewCrowdfundingApplyApi({
      app_id: item.id,
      approve,
      sol_address: store.walletAddress,
    });
    await handleRefresh();
    showToast(t('success'));
  } catch (error) {
    console.log('🚀 ~ handleReview ~ error:', error);
  } finally {
    closeLoading();
  }
};
</script>
