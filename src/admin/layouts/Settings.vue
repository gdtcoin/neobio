<template>
  <div class="flex items-center gap-2">
    <a-popover>
      <template #content>
        <div class="flex gap-2">
          <div
            v-for="color in themeStore.colorPrimaryList"
            :key="color"
            class="h-[28px] w-[28px] cursor-pointer rounded-xs"
            :style="{ background: color }"
            @click="themeStore.setColorPrimary(color)"
          />
        </div>
      </template>
      <icon icon="palette" :style="{ color: themeStore.colorPrimary }" />
    </a-popover>

    <icon :icon="themeStore.isDarkTheme ? 'brightness' : 'moon'" @click="themeStore.toggleTheme" />

    <a-dropdown>
      <template #overlay>
        <a-menu :selectedKeys="[locale]" @click="({ key }) => setLocale(key)">
          <a-menu-item v-for="item in SUPPORTED_LOCALES" :key="item.value">
            {{ item.name }}
          </a-menu-item>
        </a-menu>
      </template>

      <icon :icon="GlobalOutlined" />
    </a-dropdown>

    <icon
      v-if="isPC"
      :icon="isFullscreen ? FullscreenExitOutlined : FullscreenOutlined"
      @click="toggle"
    />

    <template v-if="userStore.userInfo">
      <a-dropdown>
        <template #overlay>
          <a-menu>
            <a-menu-item @click="$router.push('/profile')">
              <SettingOutlined />
              <span class="ml-[8px]">{{ $t('profile') }}</span>
            </a-menu-item>
            <a-menu-item @click="userStore.logout">
              <LogoutOutlined />
              <span class="ml-[8px]">{{ $t('logout') }}</span>
            </a-menu-item>
          </a-menu>
        </template>
        <a-avatar :src="userStore.userInfo.avatar" class="cursor-pointer" />
      </a-dropdown>
    </template>
  </div>
</template>

<script setup lang="ts">
import {
  FullscreenExitOutlined,
  FullscreenOutlined,
  GlobalOutlined,
  LogoutOutlined,
  SettingOutlined,
} from '@ant-design/icons-vue';
import { useFullscreen } from '@vueuse/core';
import { useI18n } from 'vue-i18n';

import { useDevice } from '@/admin/hooks';
import { setLocale, SUPPORTED_LOCALES } from '@/admin/locales';
import { useThemeStore } from '@/admin/store/theme';
import { useUserStore } from '@/admin/store/user';

const themeStore = useThemeStore();
const userStore = useUserStore();
const { locale } = useI18n();
const { isFullscreen, toggle } = useFullscreen();
const { isPC } = useDevice();
</script>
