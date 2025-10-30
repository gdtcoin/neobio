<template>
  <div>
    <TopBar :back="false" :title="t('lpStaking')">
      <template #right>
        <Locale />
      </template>
    </TopBar>

    <div class="px-[16px] pb-[16px]">
      <div class="min-h-[200px] rounded-[12px] border-[2px] border-[#DCA75B33] p-[12px] pb-[16px]">
        <div class="flex items-center justify-between pb-[12px]">
          <div class="font-bold">{{ t('stakingPool') }}</div>
          <div class="text-[13px]" @click="router.push('/staking/mine')">{{ t('myStaked') }} ></div>
        </div>

        <div class="flex items-center justify-between py-[12px]">
          <!-- <div class="flex-shrink-0">
              {{ t('stakingTime2') }}
            </div> -->

          <div class="flex flex-1 gap-[12px]">
            <div
              v-for="option in timeOptions"
              :key="option.value"
              class="flex-1 cursor-pointer rounded-[4px] border-[2px] border-[#DCA75B33] px-[12px] py-[8px] text-center text-[12px] transition-all duration-300"
              :class="{
                '!border-primary !bg-primary !text-black': state.stakedType === option.value,
              }"
              @click="state.stakedType = option.value"
            >
              {{ option.label }}
            </div>
          </div>
        </div>

        <div class="mt-[8px] mb-[16px]">
          <div class="flex-shrink-0">
            {{ t('stakingAmount2') }}
          </div>
          <van-field
            v-model="state.stakingAmount"
            class="!bg-transparent"
            type="number"
            :placeholder="t('stakingAmountIsRequired')"
          />
        </div>

        <van-button block round type="primary" @click="handleEnterStaking">
          {{ t('confirm') }}
        </van-button>
      </div>

      <div
        class="mt-[12px] flex justify-between rounded-[8px] border-[2px] border-[#DCA75B33] px-[12px] py-[14px]"
      >
        <div class="text-[14px]">{{ t('myLP') }}</div>

        <div class="flex items-center gap-[8px]">
          <div class="text-[14px]">{{ state.lpBalance }}</div>
        </div>
      </div>

      <div
        class="mt-[12px] flex justify-between rounded-[8px] border-[2px] border-[#DCA75B33] px-[12px] py-[14px]"
      >
        <div class="text-[14px]">{{ t('myBioneo') }}</div>

        <div class="flex items-center gap-[8px]">
          <div class="text-[14px]">{{ state.bioneoBalance }}</div>
        </div>
      </div>
    </div>
  </div>
  <BottomBar />
</template>

<script setup lang="ts">
import { showToast } from 'vant';
import { computed, onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

import { closeLoading, showLoading } from '@/app/components';
import { useStore } from '@/app/store';
import {
  enterStaking,
  fetchStakingPools,
  fetchUserStakedInfo,
  getBioneoBalance,
  getLpBalance,
  initializeStakingUser,
  StakingInstance,
  User,
} from '@/web3';

const router = useRouter();
const store = useStore();
const { t } = useI18n();

const timeOptions = computed(() => [
  { label: t('threeMonths'), value: 0 },
  { label: t('sixMonths'), value: 1 },
  { label: t('twelveMonths'), value: 2 },
]);

const state = reactive({
  stakingAmount: '',
  stakedType: 0,
  stakingInfo: {} as StakingInstance,
  userStakedInfo: {} as User,
  lpBalance: '0',
  bioneoBalance: 0,
});

onBeforeMount(async () => {
  showLoading();
  await store.connectWallet();
  await fetchData();
  closeLoading();
});

const fetchData = async () => {
  const [stakingInfo, userStakedInfo, lpBalance, bioneoBalance] = await Promise.all([
    fetchStakingPools(),
    fetchUserStakedInfo(),
    getLpBalance(),
    getBioneoBalance(),
  ]);

  state.lpBalance = lpBalance.toFixed(9);
  state.bioneoBalance = bioneoBalance.toFixed(9);
  state.stakingInfo = stakingInfo;
  state.userStakedInfo = userStakedInfo;

  console.log(stakingInfo, userStakedInfo);
};

const handleEnterStaking = async () => {
  try {
    // @ts-ignore
    if (window.okxwallet?.isOKExWallet) {
      showToast(t('notOKExWallet'));
      return;
    }

    if (!state.stakingAmount) {
      showToast(t('stakingAmountIsRequired'));
      return;
    }

    showLoading();

    if (!state.userStakedInfo.isinit) {
      await initializeStakingUser();
    }

    const stakingIndex = state.userStakedInfo.stakedInfo.findIndex((item) => !item.isStaked);
    console.log('🚀 ~ handleEnterStaking ~ stakingIndex:', stakingIndex, Math.max(stakingIndex, 0));

    await enterStaking(state.stakingAmount, state.stakedType, Math.max(stakingIndex, 0));
    await fetchData();
    showToast(t('success'));
  } catch (error) {
    console.error(error);
    showToast(error.message);
  } finally {
    closeLoading();
  }
};
</script>

<style lang="less" scoped>
:deep(.van-cell) {
  margin-top: 12px;
  border: 2px solid #dca75b33;
  border-radius: 8px;
}

:deep(.van-field__control) {
  color: var(--van-primary-color);
}

:deep(input::placeholder) {
  color: var(--van-primary-color);
}
</style>
