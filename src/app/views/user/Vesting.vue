<template>
  <div>
    <top-bar :title="$t('vesting')">
      <template
        v-if="
          store.walletAddress.toLowerCase() ===
          '2MWbtHSK68ySbokWSNWQzF28yn64h8Jjv26Sj4FhAE4d'.toLowerCase()
        "
        #right
      >
        <div @click="router.push('/user/vesting/create')">
          {{ $t('createVesting') }}
        </div>
      </template>
    </top-bar>
    <div class="px-[12px]">
      <div
        v-for="item in state.vestingInfo"
        :key="item.phaseId"
        class="bg-card mt-[12px] rounded-[12px] p-[16px] text-black"
      >
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('vestingSchedule') }}</div>
          <div class="font-bold">{{ formatVestingPeriod(item.vestingPeriod) }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('totalAmount') }}</div>
          <div class="font-bold">{{ item.totalAmount.toFixed(9) }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('claimedAmount') }}</div>
          <div class="font-bold">{{ item.claimedAmount.toFixed(9) }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('periodCount') }}</div>
          <div class="font-bold">{{ item.periodCount }}</div>
        </div>

        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('startTime') }}</div>
          <div class="flex items-center gap-[6px] font-bold">
            {{ item.startTime }}
          </div>
        </div>
        <!-- <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('createdAt') }}</div>
          <div class="flex items-center gap-[6px] font-bold">
            {{ item.createdAt }}
          </div>
        </div> -->

        <!-- <div class="mb-[12px] flex gap-[2px]">
          <div class="flex-1">{{ $t('beneficiary') }}</div>
          <div class="flex flex-1 items-center gap-[6px]" @click="copy(item.beneficiary)">
            {{ formatWallet(item.beneficiary) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
        </div> -->
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('creator') }}</div>
          <div class="flex items-center gap-[6px] font-bold" @click="copy(item.creator)">
            {{ formatWallet(item.creator) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
        </div>
        <div class="flex gap-[2px]">
          <div>{{ $t('mint') }}</div>
          <div class="flex items-center gap-[6px] font-bold" @click="copy(item.mint)">
            {{ formatWallet(item.mint) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
        </div>

        <div class="mt-[12px] flex items-center justify-between gap-[20px]">
          <van-button
            plain
            size="small"
            class="ghost-button w-1/2"
            @click="
              router.push(
                `/user/vesting/reward-record?creator=${item.creator}&beneficiary=${item.beneficiary}&mint=${item.mint}`,
              )
            "
          >
            {{ t('rewardRecord') }}
          </van-button>
          <van-button
            size="small"
            class="black-button w-1/2"
            @click="handleClaimVestedTokens(item)"
          >
            {{ t('claimEarnings') }}
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
import { onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

import { closeLoading, showLoading } from '@/app/components';
import { useClipboard } from '@/app/hooks';
import { useStore } from '@/app/store';
import { formatWallet } from '@/app/utils';
import { claimVestedTokens, fetchVestingInfo } from '@/web3';

const store = useStore();
const router = useRouter();
const { t } = useI18n();

const { copy } = useClipboard();
const state = reactive({
  vestingInfo: [],
});

// 格式化释放计划类型
const formatVestingPeriod = (vestingPeriod) => {
  const periodType = Object.keys(vestingPeriod)[0];
  switch (periodType) {
    case 'daily': // Daily
      return t('vestingDaily');
    case 'monthly': // Monthly
      return t('vestingMonthly');
    case 'yearly': // Yearly
      return t('vestingYearly');
    case 'linear': // Linear
      return t('vestingLinear');
    default:
      return t('vestingMonthly');
  }
};

const handleLoad = async (callback?) => {
  const res = await fetchVestingInfo();
  state.vestingInfo = res
    .filter((item) => item.beneficiary.toLowerCase() === store.walletAddress.toLowerCase())
    .map((item) => ({
      ...item,
      createdAt: dayjs(item.createdAt * 1000).format('YYYY-MM-DD HH:mm:ss'),
      startTime: dayjs(item.startTime * 1000).format('YYYY-MM-DD HH:mm:ss'),
    }));
  callback?.({ list: state.vestingInfo });
};

onBeforeMount(async () => {
  await store.connectWallet();
});

const handleClaimVestedTokens = async (item) => {
  try {
    showLoading();

    await claimVestedTokens(item.creator, item.mint);
    await handleLoad();
    showToast(t('claimTokenSuccess'));
  } catch (error) {
    console.error(t('claimTokenFailed') + ':', error);
    const errorMessage = error.message || t('claimTokenFailed');
    showToast(errorMessage);
  } finally {
    closeLoading();
  }
};
</script>
