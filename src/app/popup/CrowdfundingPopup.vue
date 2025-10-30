<template>
  <van-popup
    v-model:show="show"
    class="!w-[320px] rounded-[12px]"
    :close-on-click-overlay="false"
    closeable
    @close="close"
  >
    <div class="p-[24px] text-center">
      <div class="mb-[20px] text-[16px] font-bold">
        {{ $t('crowdfundingPhase', { phase: props.salePhase.phaseId }) }}
      </div>

      <div class="mb-[20px] flex cursor-pointer items-center justify-center gap-[8px]">
        <!-- <img src="@/app/assets/images/minus.png" class="w-[28px]" @click="handleMinus" /> -->
        <van-field
          v-model="amount"
          type="number"
          class="!w-[80px]"
          disabled
          :placeholder="$t('enterCrowdfundingAmount')"
          :border="false"
        />
        <!-- <img src="@/app/assets/images/plus.png" class="w-[28px]" @click="handlePlus" /> -->
      </div>

      <van-button type="primary" round block @click="handleConfirm">
        {{ $t('goCrowdfunding') }}
      </van-button>
    </div>
  </van-popup>
</template>

<script setup lang="ts">
import { useSessionStorage } from '@vueuse/core';
import { showToast } from 'vant';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import { closeLoading, showLoading } from '@/app/components';
import { purchaseShare } from '@/web3';

const crowdfundingLink = useSessionStorage('crowdfundingLink', '');

const props = defineProps(['salePhase', 'onSuccess']);

const { t } = useI18n();

const show = ref(false);
const amount = ref(1);

const open = () => {
  show.value = true;
};

const close = () => {
  show.value = false;
};

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const handleMinus = () => {
  if (amount.value <= 1) {
    return;
  }

  amount.value--;
};

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const handlePlus = () => {
  amount.value++;
};

const handleConfirm = async () => {
  const { phaseId, soldShares } = props.salePhase;

  try {
    showLoading();
    await purchaseShare(amount.value, phaseId, soldShares, crowdfundingLink.value);
    await props.onSuccess?.();
    closeLoading();
    showToast(t('success'));
    close();
  } catch (error) {
    console.error(error);
    showToast(error.message);
    closeLoading();
  }
};

defineExpose({
  open,
  close,
});
</script>

<style lang="less" scoped>
:deep(.van-field) {
  width: 80px;
  background-color: var(--van-gray-3);
  border-radius: 12px;

  .van-field__control {
    text-align: center;
  }
}
</style>
