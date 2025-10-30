<template>
  <div>
    <top-bar :title="$t('createVesting')"></top-bar>

    <div class="m-[12px] rounded-[12px] border-[2px] border-[#DCA75B33] px-[4px] py-[12px]">
      <van-form @submit="handleCreateVestingSchedule">
        <van-cell-group inset>
          <van-field v-model="state.beneficiary" name="beneficiary" :label="$t('beneficiary')" />
          <van-field v-model="state.mint" name="mint" :label="$t('mint')" />
          <van-field
            v-model="state.totalAmount"
            name="totalAmount"
            :label="$t('totalAmount')"
            type="number"
          />
          <van-field
            v-model="state.startTime"
            name="startTime"
            is-link
            readonly
            :label="$t('startTime')"
            @click="state.showCalendar = true"
          />
          <van-field
            :model-value="formatVestingPeriodForDisplay(state.vestingPeriod)"
            name="vestingPeriod"
            is-link
            readonly
            :label="$t('vestingSchedule')"
            @click="state.showVestingPicker = true"
          />
          <van-field
            v-model="state.periodCount"
            name="periodCount"
            :label="$t('periodCount')"
            type="number"
          />
        </van-cell-group>
        <div class="mt-[12px] px-[20px]">
          <van-button round block type="primary" native-type="submit">
            {{ $t('createVesting') }}
          </van-button>
        </div>
      </van-form>

      <!-- 日期选择器 -->
      <van-calendar
        v-model:show="state.showCalendar"
        type="single"
        :min-date="new Date()"
        @confirm="onCalendarConfirm"
      />

      <!-- 释放计划类型选择器 -->
      <van-action-sheet v-model:show="state.showVestingPicker" :title="$t('vestingSchedule')">
        <div class="px-[20px] py-[20px]">
          <van-cell
            v-for="period in vestingPeriodOptions"
            :key="period.value"
            :title="period.text"
            is-link
            @click="onVestingPeriodSelect(period.value)"
          />
        </div>
      </van-action-sheet>
    </div>

    <div class="px-[12px]">
      <div
        v-for="item in state.vestingInfo"
        :key="item.phaseId"
        class="bg-card mt-[12px] rounded-[12px] p-[16px] text-[13px] text-black"
      >
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('vestingSchedule') }}</div>
          <div class="font-bold">{{ formatVestingPeriod(item.vestingPeriod) }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('claimedAmount') }}</div>
          <div class="font-bold">{{ item.claimedAmount }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('periodCount') }}</div>
          <div class="font-bold">{{ item.periodCount }}</div>
        </div>
        <div class="mb-[12px] flex gap-[2px]">
          <div>{{ $t('totalAmount') }}</div>
          <div class="font-bold">{{ item.totalAmount }}</div>
        </div>

        <div class="mb-[12px] flex">
          <div>{{ $t('startTime') }}</div>
          <div class="flex items-center font-bold">
            {{ item.startTime }}
          </div>
        </div>
        <div class="mb-[12px] flex">
          <div>{{ $t('createdAt') }}</div>
          <div class="flex items-center gap-[6px] font-bold">
            {{ item.createdAt }}
          </div>
        </div>

        <div class="mb-[12px] flex">
          <div>{{ $t('beneficiary') }}</div>
          <div class="flex items-center gap-[6px] font-bold" @click="copy(item.beneficiary)">
            {{ formatWallet(item.beneficiary) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
        </div>
        <div class="mb-[12px] flex">
          <div>{{ $t('creator') }}</div>
          <div class="flex items-center gap-[6px] font-bold" @click="copy(item.creator)">
            {{ formatWallet(item.creator) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
        </div>
        <div class="flex">
          <div>{{ $t('mint') }}</div>
          <div class="flex items-center gap-[6px] font-bold" @click="copy(item.mint)">
            {{ formatWallet(item.mint) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
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

import { closeLoading, showLoading } from '@/app/components';
import { useClipboard } from '@/app/hooks';
import { useStore } from '@/app/store';
import { formatWallet } from '@/app/utils';
import { config, createVestingSchedule, fetchVestingInfo, VestingPeriod } from '@/web3';

const store = useStore();
const { t } = useI18n();

const { copy } = useClipboard();
const state = reactive({
  vestingInfo: [],
  showCalendar: false,
  startTime: '',
  beneficiary: '',
  mint: config.BIONEO_TOKEN_MINT, // 默认值
  totalAmount: '',
  vestingPeriod: VestingPeriod.Linear, // 默认值
  periodCount: '',
  showVestingPicker: false,
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

// 用于表单显示的释放计划类型格式化
const formatVestingPeriodForDisplay = (vestingPeriod) => {
  switch (vestingPeriod) {
    case VestingPeriod.Daily:
      return t('vestingDaily');
    case VestingPeriod.Monthly:
      return t('vestingMonthly');
    case VestingPeriod.Yearly:
      return t('vestingYearly');
    case VestingPeriod.Linear:
      return t('vestingLinear');
    default:
      return t('vestingLinear');
  }
};

// 释放计划选项
const vestingPeriodOptions = [
  { value: VestingPeriod.Daily, text: t('vestingDaily') },
  { value: VestingPeriod.Monthly, text: t('vestingMonthly') },
  { value: VestingPeriod.Yearly, text: t('vestingYearly') },
  { value: VestingPeriod.Linear, text: t('vestingLinear') },
];

// 日历确认回调
const onCalendarConfirm = (date) => {
  state.startTime = dayjs(date).format('YYYY-MM-DD HH:mm');
  state.showCalendar = false;
};

// 释放计划类型选择回调
const onVestingPeriodSelect = (period) => {
  state.vestingPeriod = period;
  state.showVestingPicker = false;
};

const handleLoad = async (callback?) => {
  const res = await fetchVestingInfo();
  console.log('🚀 ~ loadData ~ res:', res);
  state.vestingInfo = res
    .filter((item) => item.creator === store.walletAddress)
    .map((item) => ({
      ...item,
      createdAt: dayjs(item.createdAt * 1000).format('YYYY-MM-DD HH:mm:ss'),
      startTime: dayjs(item.startTime * 1000).format('YYYY-MM-DD HH:mm:ss'),
    }))
    .sort((a, b) => b.createdAt - a.createdAt);
  callback?.({ list: state.vestingInfo });
};

onBeforeMount(async () => {
  await store.connectWallet();
});

const handleCreateVestingSchedule = async () => {
  try {
    // 表单验证
    if (!state.beneficiary) {
      showToast('请输入受益人地址');
      return;
    }
    if (!state.mint) {
      showToast('请输入代币地址');
      return;
    }
    if (!state.totalAmount || Number(state.totalAmount) <= 0) {
      showToast('请输入有效的总金额');
      return;
    }
    if (!state.startTime) {
      showToast('请选择开始时间');
      return;
    }
    if (!state.periodCount || Number(state.periodCount) <= 0) {
      showToast('请输入有效的释放周期数');
      return;
    }

    showLoading();

    // 转换开始时间为Unix时间戳
    const startTimeUnix = dayjs(state.startTime).unix();

    await createVestingSchedule(
      state.beneficiary,
      state.mint,
      Number(state.totalAmount),
      startTimeUnix,
      state.vestingPeriod,
      Number(state.periodCount),
    );

    showToast(t('createVestingSuccess'));

    // 清空表单
    state.beneficiary = '';
    state.mint = config.BIONEO_TOKEN_MINT;
    state.totalAmount = '';
    state.startTime = '';
    state.vestingPeriod = VestingPeriod.Linear;
    state.periodCount = '';

    // 刷新列表
    await handleLoad();
  } catch (error) {
    console.error(t('createVestingFailed') + ':', error);
    const errorMessage = error.message || t('createVestingFailed');
    showToast(errorMessage);
  } finally {
    closeLoading();
  }
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
