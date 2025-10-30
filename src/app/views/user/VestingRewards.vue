<template>
  <div>
    <top-bar :title="t('rewardRecord')" />
    <div class="px-[12px]">
      <div
        v-for="item in state.list"
        :key="item.blockTime"
        class="bg-card mt-[12px] rounded-[12px] p-[16px] text-black"
      >
        <div class="mb-[12px] flex">
          <div class="flex-1">{{ $t('amount') }}</div>
          <div class="flex flex-1 items-center gap-[6px]">
            {{ item.amount.toFixed(9) }}
          </div>
        </div>

        <div class="mb-[12px] flex">
          <div class="flex-1">{{ $t('blockTime') }}</div>
          <div class="flex flex-1 items-center gap-[6px]">
            {{ dayjs(item.blockTime * 1000).format('YYYY-MM-DD HH:mm:ss') }}
          </div>
        </div>

        <div class="mb-[12px] flex">
          <div class="flex-1">{{ $t('mint') }}</div>
          <div class="flex flex-1 items-center gap-[6px]" @click="copy(item.mint)">
            {{ formatWallet(item.mint) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
        </div>

        <div class="mb-[12px] flex">
          <div class="flex-1">{{ $t('from') }}</div>
          <div class="flex flex-1 items-center gap-[6px]" @click="copy(item.from)">
            {{ formatWallet(item.from) }}
            <img src="@/app/assets/images/copy.png" class="w-[14px]" />
          </div>
        </div>
        <div class="flex">
          <div class="flex-1">{{ $t('to') }}</div>
          <div class="flex flex-1 items-center gap-[6px]" @click="copy(item.to)">
            {{ formatWallet(item.to) }}
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
import { onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';

import { useClipboard } from '@/app/hooks';
import { useStore } from '@/app/store';
import { formatWallet } from '@/app/utils';
import { getVestingTransferHistory } from '@/web3';

const { copy } = useClipboard();
const { t } = useI18n();

const route = useRoute();
const store = useStore();

const state = reactive({
  list: [],
});

onBeforeMount(async () => {
  await store.connectWallet();
});

const handleLoad = async (callback?) => {
  const creator = route.query.creator as string;
  const beneficiary = route.query.beneficiary as string;
  const mint = route.query.mint as string;
  const res = await getVestingTransferHistory(creator, beneficiary, mint);
  state.list = res;

  callback?.({ list: res });
};
</script>
