<template>
  <a-card>
    <div class="flex flex-wrap gap-2">
      <div
        v-for="item in svgIconsList"
        :key="item"
        class="flex h-[80px] w-[80px] cursor-pointer flex-col items-center justify-center"
        @click="handleCopy(item)"
      >
        <svg-icon :name="item" class="!size-[24px]" />
        <span class="pt-2">{{ item }}</span>
      </div>
    </div>
  </a-card>
</template>

<script setup lang="ts">
import svgIcons from '@/admin/assets/svgIcons';
import { useClipboard } from '@/admin/hooks';

const svgIconsList = Object.keys(svgIcons);

const { copy } = useClipboard();

const handleCopy = (item) => {
  copy(`<svg-icon name="${item}" />`);
};
</script>
