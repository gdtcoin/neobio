<template>
  <a-config-provider
    :key="state.key"
    :locale="state.antdLocale"
    :theme="themeStore.antdThemeConfig"
  >
    <router-view />
  </a-config-provider>
</template>

<script setup lang="ts">
import { theme } from 'ant-design-vue';
import { reactive, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import { getAntdLocale } from '@/admin/locales';
import { useThemeStore } from '@/admin/store/theme';
import { reloadEventBus } from '@/admin/utils/eventBus';

const { useToken } = theme;

const { token } = useToken();
const themeStore = useThemeStore();
const { locale } = useI18n();

reloadEventBus.on(() => {
  state.key++;
});

const state = reactive({
  antdLocale: getAntdLocale(),
  key: 0,
});

watch(locale, () => {
  state.antdLocale = getAntdLocale();
});

watch(
  token,
  () => {
    themeStore.applyAntdCssVariablesViaStyle(token.value);
  },
  {
    immediate: true,
  },
);
</script>
