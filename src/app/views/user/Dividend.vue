<template>
  <div class="pb-[8px]">
    <TopBar :title="$t('totalDividendPool')" />

    <div
      v-if="state.dividend && state.tokenPool"
      class="bg-card mx-[8px] my-[8px] overflow-hidden rounded-[12px] px-[16px] py-[8px] text-black"
    >
      <!-- 表格头部 -->
      <div class="flex items-center font-bold">
        <div class="w-1/3 py-[12px] text-[14px]">{{ $t('category') }}</div>
        <div class="w-2/3 px-[8px] py-[12px] text-[14px]">
          {{ $t('currentMonth') }}
        </div>
        <!-- <div class="w-1/2 py-[12px] text-[14px]">{{ $t('cumulative') }}</div> -->
      </div>

      <!-- 表格数据行 -->
      <div class="flex items-center">
        <div class="w-1/3 py-[12px] text-[13px] font-bold">
          {{ $t('networkDividendPool') }}
        </div>
        <div class="w-2/3 px-[8px] py-[12px] text-[13px]">
          {{ totalDividendMonth }}
        </div>
        <!-- <div class="w-1/2 py-[12px] text-[13px]">{{ totalDividend }}</div> -->
      </div>

      <div class="flex items-center">
        <div class="w-1/3 py-[12px] text-[13px] font-bold">
          <div>{{ $t('fixedDividendPool') }}</div>
          <div class="mt-[4px] text-[12px] font-normal italic">
            {{ $t('fixedDividendPoolAmount') }}
          </div>
        </div>
        <div class="w-2/3 px-[8px] py-[12px] text-[13px]">
          <div>{{ TOKEN_AMOUNT_PER_MONTH.toLocaleString() }}</div>
          <div class="mt-[4px] text-[12px] font-normal italic">
            {{
              $t('fixedDividendPoolTips', {
                month: state.tokenPool.total_issued_periods,
                amount: state.tokenPool.remaining_amount.toLocaleString(),
              })
            }}
          </div>
        </div>
        <!-- <div class="w-1/2 py-[12px] text-[13px]">
          {{ state.tokenPool.total_issued?.toFixed(9) }}
        </div> -->
      </div>

      <div class="flex items-center">
        <div class="w-1/3 py-[12px] text-[13px] font-bold">
          {{ $t('rechargeDividend') }}
        </div>
        <div class="w-2/3 px-[8px] py-[12px] text-[13px]">
          {{ state.dividend.vip_buy_bio_month?.toFixed(9) }}
        </div>
        <!-- <div class="w-1/2 py-[12px] text-[13px]">
          {{ state.dividend.vip_buy_bio?.toFixed(9) }}
        </div> -->
      </div>

      <div class="flex items-center">
        <div class="w-1/3 py-[12px] text-[13px] font-bold">{{ $t('nodeDividend') }}</div>
        <div class="w-2/3 px-[8px] py-[12px] text-[13px]">
          {{ state.dividend.node_withdraw_bio_month?.toFixed(9) }}
        </div>
        <!-- <div class="w-1/2 py-[12px] text-[13px]">
          {{ state.dividend.node_withdraw_bio?.toFixed(9) }}
        </div> -->
      </div>

      <div class="flex items-center">
        <div class="w-1/3 py-[12px] text-[13px] font-bold">
          {{ $t('powerDividend') }}
        </div>
        <div class="w-2/3 px-[8px] py-[12px] text-[13px]">
          {{ state.dividend.vip_withdraw_bio_month?.toFixed(9) }}
        </div>
        <!-- <div class="w-1/2 py-[12px] text-[13px]">
          {{ state.dividend.vip_withdraw_bio?.toFixed(9) }}
        </div> -->
      </div>

      <div class="flex items-center">
        <div class="w-1/3 py-[12px] text-[13px] font-bold">{{ $t('lpDividend') }}</div>
        <div class="w-2/3 px-[8px] py-[12px] text-[13px]">
          {{ state.dividend.lp_withdraw_bio_month?.toFixed(9) }}
        </div>
        <!-- <div class="w-1/2  py-[12px] text-[13px]">
          {{ state.dividend.lp_withdraw_bio?.toFixed(9) }}
        </div> -->
      </div>
    </div>

    <div
      class="bg-card mx-[8px] my-[8px] overflow-hidden rounded-[12px] px-[16px] py-[16px] pb-[8px] text-black"
    >
      <div class="mb-[4px] font-bold">{{ $t('destroyAmount') }}</div>
      <div class="flex items-center">
        <div class="w-1/3 py-[12px] text-[13px] font-bold">
          {{ $t('gdtcAmount') }}
        </div>
        <div class="w-2/3 px-[8px] py-[12px] text-[13px]">
          {{ state.blackHoleGdtcBalance?.toFixed(9) }}
        </div>
      </div>

      <div class="flex items-center">
        <div class="w-1/3 py-[12px] text-[13px] font-bold">
          {{ $t('bioneoAmount') }}
        </div>
        <div class="w-2/3 px-[8px] py-[12px] text-[13px]">
          {{ state.blackHoleBioneoBalance?.toFixed(9) }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive } from 'vue';

import { fetchDividendApi, fetchTokenPool } from '@/app/api';
import { useStore } from '@/app/store';
import { getBlackHoleBioneoBalance, getBlackHoleGdtcBalance } from '@/web3';

const store = useStore();

const TOKEN_AMOUNT_PER_MONTH = 9625;

const state = reactive({
  dividend: null,
  tokenPool: null,
  blackHoleGdtcBalance: 0,
  blackHoleBioneoBalance: 0,
});

const loadBlackHoleBalance = async () => {
  const [bioneoBalance, gdtcBalance] = await Promise.all([
    getBlackHoleBioneoBalance(),
    getBlackHoleGdtcBalance(),
  ]);
  state.blackHoleBioneoBalance = bioneoBalance;
  state.blackHoleGdtcBalance = gdtcBalance;

  console.log('🚀 ~ loadBlackHoleBalance ~ balance:', bioneoBalance, gdtcBalance);
};

// eslint-disable-next-line
const totalDividend = computed(() => {
  if (!state.dividend || !state.tokenPool) {
    return 0;
  }

  return (
    state.dividend.lp_withdraw_bio +
    state.dividend.vip_buy_bio +
    state.dividend.node_withdraw_bio +
    state.dividend.vip_withdraw_bio +
    state.tokenPool.total_issued
  ).toFixed(9);
});

const totalDividendMonth = computed(() => {
  if (!state.dividend || !state.tokenPool) {
    return 0;
  }

  return (
    state.dividend.lp_withdraw_bio_month +
    state.dividend.vip_buy_bio_month +
    state.dividend.node_withdraw_bio_month +
    state.dividend.vip_withdraw_bio_month +
    TOKEN_AMOUNT_PER_MONTH
  ).toFixed(9);
});

const fetchData = async () => {
  loadBlackHoleBalance();
  const res = await Promise.all([
    fetchDividendApi({
      sol_address: store.walletAddress,
    }),
    fetchTokenPool({
      sol_address: store.walletAddress,
    }),
  ]);

  state.dividend = res[0].data;
  state.tokenPool = res[1].data;
};

onMounted(async () => {
  await store.connectWallet();
  fetchData();
});
</script>
