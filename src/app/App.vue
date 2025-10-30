<template>
  <van-config-provider theme="light">
    <div class="app-root">
      <router-view v-slot="{ Component }">
        <keep-alive :include="state.keepAlive">
          <component :is="Component" />
        </keep-alive>
      </router-view>
    </div>
  </van-config-provider>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { useStore } from '@/app/store';

const route = useRoute();
const router = useRouter();

const state = reactive({
  keepAlive: ['User'] as string[],
});

const store = useStore();

watch([() => store.walletAddress, () => store.userInfo], () => {
  if (store.walletAddress && store.userInfo && !store.userInfo.parent_code && route.path !== '/') {
    router.push('/');
  }
});

watch(
  () => route.path,
  (newPath) => {
    if (newPath.startsWith('/user')) {
      state.keepAlive = ['User'];
    } else {
      state.keepAlive = [];
    }
  },
  { immediate: true },
);
</script>

<style lang="less">
.app-root {
  height: 100vh;
  -webkit-overflow-scrolling: touch;
}

.app-view {
  position: relative;
  min-height: 100%;
}
</style>
