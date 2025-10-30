<template>
  <div class="login-page">
    <Locale />

    <div class="login-logo">
      <img
        :src="
          locale === 'zh-CN'
            ? require('@/app/assets/images/logo_zh.png')
            : require('@/app/assets/images/logo_en.png')
        "
        class="logo"
      />
      <!-- <img src="@/app/assets/images/logo_title.png" class="logo-title" />
      <div class="logo-description">
        {{ $t('loginDescription') }}
      </div> -->
    </div>

    <div class="login-wrapper">
      <template v-if="store.userInfo && !store.userInfo?.parent_code">
        <van-field v-model="state.code" :border="false" :placeholder="$t('pleaseFillCode')" />

        <van-button block round type="primary" class="login-button" @click="handleBind">
          {{ $t('login') }}
        </van-button>

        <!-- <p class="skip" @click="handleSkip">
          {{ $t('skip') }}
        </p> -->
      </template>

      <template v-else>
        <van-button block round type="primary" class="login-button" @click="handleConnectWallet">
          {{ $t('connectWallet') }}
        </van-button>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { showToast } from 'vant';
import { reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

import { bindApi } from '@/app/api';
import Locale from '@/app/components/Locale.vue';
import { useStore } from '@/app/store';

const store = useStore();
const router = useRouter();
const { t, locale } = useI18n();
const state = reactive({
  code: '',
});

const handleConnectWallet = async () => {
  await store.connectWallet();

  if (store.userInfo?.parent_code) {
    router.push('/nft');
  }
};

const handleBind = async () => {
  if (!state.code) {
    showToast(t('pleaseFillCode'));
    return;
  }

  await bindApi({ parent_code: state.code, sol_address: store.walletAddress });
  router.push('/nft');
};

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const handleSkip = () => {
  router.push('/nft');
};
</script>

<style lang="less" scoped>
.login-page {
  padding: 24px 16px;

  .login-logo {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 100px 0 80px;

    .logo {
      width: 340px;
    }

    .logo-title {
      width: 160px;
      margin: 16px 0 12px;
    }

    .logo-description {
      color: var(--van-primary-color);
    }
  }

  :deep(.locale-wrapper) {
    position: fixed;
    top: 16px;
    right: 16px;
  }

  :deep(.van-cell) {
    margin-bottom: 16px;
    text-align: center;
    background: transparent;
    border: 1px solid var(--van-primary-color);
    border-radius: 100px;

    .van-field__control {
      color: var(--van-primary-color);
      text-align: center;
    }

    input::placeholder {
      color: var(--van-primary-color);
    }
  }

  .skip {
    margin-top: 16px;
    font-size: 13px;
    color: var(--van-text-color-2);
    text-align: center;
    cursor: pointer;
  }

  .login-button {
    height: 48px;
    color: black;
    background: linear-gradient(0deg, #a34600 -1.67%, #e9a02d 100%);
    border: none;
    box-shadow: 0 2px 0 0 #f4c26f inset;
  }
}
</style>
