<template>
  <div class="text-primary">
    <TopBar :back="false" :title="t('powerPoolMiningTitle')">
      <template #left>
        <span @click="$router.push('/nft/recharge-record')">{{ t('rechargeRecord') }}</span>
      </template>
      <template #right>
        <Locale />
      </template>
    </TopBar>
    <div class="flex h-[100px] flex-col items-center justify-center gap-[6px]">
      <div>{{ t('myPower') }}</div>
      <div class="text-primary2 mt-[4px] text-[32px] font-bold">{{ totalPower }}</div>
    </div>

    <div class="my-[12px] flex flex-col gap-[12px] px-[12px]">
      <div
        v-for="item in nftList"
        :key="item.value"
        class="bg-card flex items-center justify-between rounded-[12px] p-[16px]"
      >
        <div class="flex flex-col items-start gap-[8px] text-black">
          <div class="text-[17px] font-bold">{{ item.name }}</div>
          <div
            class="flex items-center gap-[4px] rounded-[100px] bg-[#FFA025] px-[8px] py-[4px] text-[12px] font-bold text-black"
          >
            <div class="flex h-[18px] w-[18px] items-center justify-center rounded-[50%] bg-white">
              <img src="@/app/assets/images/gift.png" class="h-[14px] w-[14px]" />
            </div>
            {{ t('freeNFT', { count: item.nftCount }) }}
          </div>
        </div>

        <van-button
          ghost
          size="small"
          class="recharge-button"
          round
          @click="handleBuyNft(item.value)"
        >
          {{ t('recharge') }}
        </van-button>
      </div>
    </div>
  </div>

  <BottomBar />
</template>

<script setup lang="ts">
import { showToast } from 'vant';
import { computed, onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';

import { addDividendApi } from '@/app/api';
import { closeLoading, showLoading } from '@/app/components';
import { useStore } from '@/app/store';
import { clearAltAddress, fetchNftMiningSystem, fetchOrderInfo, purchaseNft } from '@/web3';

const { t } = useI18n();

const store = useStore();
const state = reactive({
  orderInfo: [],
});

const nftList = [
  {
    value: 100,
    name: '100USDT',
    nftCount: 1,
  },
  {
    value: 300,
    name: '300USDT',
    nftCount: 3,
  },

  {
    value: 500,
    name: '500USDT',
    nftCount: 5,
  },
  {
    value: 1000,
    name: '1000USDT',
    nftCount: 10,
  },
];

onBeforeMount(async () => {
  showLoading();
  await store.connectWallet();
  await loadOrderInfo();
  closeLoading();
});

const loadOrderInfo = async () => {
  const nftMiningSystem = await fetchNftMiningSystem();
  console.log(nftMiningSystem);
  const orderInfo = await fetchOrderInfo();
  state.orderInfo = orderInfo;
};

const totalPower = computed(() => {
  return state.orderInfo.reduce((acc, item) => acc + item.totalPower, 0).toFixed(2);
});

const handleBuyNft = async (value: number) => {
  try {
    // @ts-ignore
    if (window.okxwallet?.isOKExWallet) {
      showToast(t('notOKExWallet'));
      return;
    }
    showLoading();
    await purchaseNft(value);
    await loadOrderInfo();
    addDividendApi({
      business_type: 2,
      value_usdt: value * 0.05,
      user_address: store.walletAddress,
      business_id: state.orderInfo[0]?.orderInfoIndex,
    });
    showToast(t('success'));
  } catch (error) {
    let message = error?.message || '';

    if (message.includes('e.serialize')) {
      message = '';
    }

    if (message.includes('encoding overruns Uint8Array')) {
      clearAltAddress();
    }

    message && showToast(t(message));
  } finally {
    closeLoading();
  }
};
</script>

<style lang="less" scoped>
.recharge-button {
  min-width: 72px;
  font-weight: bold;
  color: black;
  background: transparent;
  border-color: black;
}
</style>
