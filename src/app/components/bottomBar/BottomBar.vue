<template>
  <ul class="bottom-bar">
    <li
      v-for="item in list"
      :key="item.name"
      class="bar-item"
      :class="{ active: $route.path === item.path }"
      @click="handleClick(item)"
    >
      <img class="icon" :src="$route.path === item.path ? item.activeIcon : item.icon" />
      <p class="text">{{ item.name }}</p>
    </li>
  </ul>
  <div class="bottom-bar-placeholder"></div>
</template>

<script setup lang="ts">
import { useSessionStorage } from '@vueuse/core';
import { computed, onBeforeMount } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

import { getApplicationDetailApi } from '@/app/api';
import { useStore } from '@/app/store';
import { fetchCrowdfundingInfo } from '@/web3';

const router = useRouter();
const store = useStore();
const crowdfundingLink = useSessionStorage('crowdfundingLink', '');
const { t } = useI18n();

onBeforeMount(async () => {
  if (!store.walletAddress || store.walletAddress) {
    return;
  }

  checkCrowdfunding();
});

const list = computed(() => {
  const l = [
    {
      name: t('powerPoolMining'),
      path: '/nft',
      icon: require('./images/nft.png'),
      activeIcon: require('./images/nft_active.png'),
      visible: true,
    },
    {
      name: t('lpStaking'),
      path: '/staking',
      icon: require('./images/staking.png'),
      activeIcon: require('./images/staking_active.png'),
      visible: true,
    },
    // {
    //   name: t('crowdfunding'),
    //   path: '/crowdfunding',
    //   icon: require('./images/crowdfunding.png'),
    //   activeIcon: require('./images/crowdfunding_active.png'),
    // },
    {
      name: t('user'),
      path: '/user',
      icon: require('./images/user.png'),
      activeIcon: require('./images/user_active.png'),
      visible: true,
    },
  ];

  return l.filter((item) => item.visible);
});

const checkCrowdfunding = async () => {
  const res = await getApplicationDetailApi({
    sol_address: store.walletAddress,
  });
  const find = res.data.filter((item) => item.status === 1)[0];

  if (!find) {
    return;
  }

  const crowdfundingInfo = await fetchCrowdfundingInfo();

  if (find.phase_id === crowdfundingInfo.phaseCount) {
    crowdfundingLink.value = find.purchase_link;
  }
};

const handleClick = (item) => {
  router.push(item.path);
};
</script>

<style lang="less" scoped>
@keyframes scale {
  50% {
    transform: scale(0.8);
  }
}

.bottom-bar-placeholder {
  height: 70px;
}

.bottom-bar {
  position: fixed;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 10;
  display: flex;
  align-items: center;
  height: 70px;
  padding: 0 0 2px;
  background: var(--van-black);
  // box-shadow: 0 0 1px rgba(#c1c1c1, 0.5);

  .bar-item {
    flex: 1;
    text-align: center;

    &.active {
      animation: scale 0.3s ease-in;

      .text {
        position: relative;
        font-weight: bold;
        color: var(--primary-color2);
      }
    }

    .icon {
      width: 20px;
      height: 20px;
    }

    .text {
      margin-top: 2px;
      font-size: 12px;
      color: var(--primary-color);
    }
  }
}
</style>
