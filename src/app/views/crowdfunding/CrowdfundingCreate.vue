<template>
  <div>
    <top-bar :title="$t('nodeManagement')"></top-bar>

    <div
      class="m-[12px] rounded-[12px] border-[2px] border-[#DCA75B33] px-[4px] py-[12px] pb-[24px]"
    >
      <van-form @submit="handleSubmit">
        <van-cell-group inset>
          <van-field
            v-model="state.pricePerShare"
            name="pricePerShare"
            :label="$t('crowdfundingPrice')"
            type="number"
          />
          <van-field
            v-model="state.startTime"
            name="startTime"
            is-link
            readonly
            :label="$t('crowdfundingStartTime')"
            @click="state.showCalendar = true"
          />
        </van-cell-group>
        <div class="mt-[12px] px-[12px]">
          <van-button round block type="primary" native-type="submit">
            {{ $t('createCrowdfunding') }}
          </van-button>
        </div>
      </van-form>
    </div>
    <div class="px-[16px]">
      <div
        v-for="salePhase in state.salePhases"
        :key="salePhase.phaseId"
        class="bg-card mt-[12px] rounded-[12px] p-[16px] text-[13px] text-black"
      >
        <div class="mb-[12px] flex items-center justify-between">
          <span class="text-[14px] font-bold text-black" @click="handlePhaseClick(salePhase)">
            {{ $t('crowdfundingPhase', { phase: salePhase.phaseId }) }}
            <span v-if="state.crowdfundingInfo.phaseCount === salePhase.phaseId" class="pl-[4px]">
              >
            </span>
          </span>
          <span
            class="text-[13px] font-bold"
            @click="$router.push(`/user/crowdfunding/phase/${salePhase.phaseId}`)"
          >
            {{ $t('participateRecords') }} >
          </span>
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
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('crowdfundingPricePerShare') }}</div>
          <div class="font-bold">{{ salePhase.pricePerShare }} USDT</div>
        </div>

        <div class="flex gap-[2px]">
          <div>{{ $t('crowdfundingStartTime') }}</div>
          <div class="font-bold">
            {{ formatTime(salePhase.startTime) }}
          </div>
        </div>
      </div>
    </div>
    <infinite-loading v-if="store.walletAddress" @load="handleLoad" />

    <van-calendar v-model:show="state.showCalendar" @confirm="handleSelectTime" />
  </div>
</template>

<script setup lang="ts">
import dayjs from 'dayjs';
import { showToast } from 'vant';
import { onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

import { closeLoading, showLoading } from '@/app/components';
import { useStore } from '@/app/store';
import { formatTime } from '@/app/utils/format';
import { createPhase, fetchAllCrowdfundingInfo } from '@/web3';

const store = useStore();
const router = useRouter();
const { t } = useI18n();
const state = reactive({
  salePhases: [],
  userPurchases: [],
  crowdfundingInfo: null,
  pricePerShare: '',
  startTime: '',
  showCalendar: false,
});

onBeforeMount(async () => {
  if (store.walletAddress) {
    return;
  }

  await store.connectWallet();
});

const handlePhaseClick = (salePhase) => {
  if (salePhase.phaseId !== state.crowdfundingInfo.phaseCount) {
    return;
  }

  router.push(`/crowdfunding`);
};

const handleLoad = async (callback?) => {
  const { salePhases, userPurchases, crowdfundingInfo } = await fetchAllCrowdfundingInfo();

  state.salePhases = salePhases;
  state.userPurchases = userPurchases;
  state.crowdfundingInfo = crowdfundingInfo;

  console.log('🚀 ~ salePhases:', state.salePhases);
  console.log('🚀 ~ userPurchases:', state.userPurchases);
  console.log('🚀 ~ crowdfundingInfo:', state.crowdfundingInfo);

  callback?.({ list: state.salePhases });
};

const handleSubmit = async () => {
  try {
    const startTime = dayjs(state.startTime).unix();
    showLoading();
    await createPhase(state.pricePerShare, startTime);
    await handleLoad();
    state.startTime = '';
    showToast(t('success'));
  } catch (error) {
    console.error(error);
    showToast(error.message);
  } finally {
    closeLoading();
  }
};

const handleSelectTime = (value) => {
  state.startTime = dayjs(value).format('YYYY-MM-DD 00:00:00');
  state.showCalendar = false;
};
</script>

<style lang="less" scoped>
:deep(.van-cell-group--inset) {
  margin: 0;
}

:deep(.van-cell-group) {
  background-color: transparent;
}

:deep(.van-cell) {
  background-color: transparent;

  &::after {
    border-color: #dca75b33;
  }

  .van-field__label,
  .van-field__control {
    color: var(--van-primary-color);
  }

  .van-cell__right-icon {
    color: var(--van-primary-color);
  }
}

:deep(input::placeholder) {
  color: var(--van-primary-color);
}
</style>
