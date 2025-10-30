<template>
  <div>
    <a-row :gutter="[12, 12]">
      <a-col v-for="item in overviews" :key="item.title" :xs="24" :sm="12" :lg="6">
        <a-card hoverable>
          <div class="text-base font-bold">
            {{ item.title }}
          </div>

          <div class="flex items-center justify-between py-[16px]">
            <div class="text-base">{{ item.value.toLocaleString() }}</div>
            <svg-icon :name="item.icon" class="!size-[36px]" />
          </div>

          <div class="flex items-center justify-between">
            <div>{{ item.totalTitle }}</div>
            <div>
              {{ item.totalValue.toLocaleString() }}
            </div>
          </div>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const overviews = computed(() => [
  {
    icon: 'card',
    title: '用户',
    totalTitle: '总用户',
    totalValue: 120_000,
    value: 2000,
  },
  {
    icon: 'cake',
    title: '访问',
    totalTitle: '总访问',
    totalValue: 500_000,
    value: 20_000,
  },
  {
    icon: 'download',
    title: '下载',
    totalTitle: '总下载',
    totalValue: 120_000,
    value: 8000,
  },
  {
    icon: 'bell',
    title: '用户',
    totalTitle: '总用户',
    totalValue: 50_000,
    value: 5000,
  },
]);
</script>
