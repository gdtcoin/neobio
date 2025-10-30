<template>
  <a-menu
    v-model:selectedKeys="state.selectedKeys"
    v-model:openKeys="state.openKeys"
    mode="inline"
    :theme="themeStore.theme"
    :items="state.menuData"
    style="border: none"
    @select="handleSelect"
  />
</template>

<script setup lang="ts">
import { h, reactive, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';

import routes from '@/admin/router/routes';
import { useThemeStore } from '@/admin/store/theme';
import { useUserStore } from '@/admin/store/user';

const SELECTED_KEYS_MAP = {};
const OPEN_KEYS_MAP = {};

const props = defineProps({
  collapsed: {
    type: Boolean,
  },
});

const { locale } = useI18n();
const themeStore = useThemeStore();
const userStore = useUserStore();
const route = useRoute();
const router = useRouter();

const getMenuData = (items = [], parentKeys = [], selectedKey?) => {
  const menuData = [];

  for (const item of items) {
    if (item.meta && item.meta.roles && !userStore.hasRole(item.meta.roles)) {
      continue;
    }

    if (item.name && !item.hideInMenu) {
      OPEN_KEYS_MAP[item.path] = parentKeys;
      SELECTED_KEYS_MAP[item.path] = [selectedKey || item.path];

      const newItem = {
        key: item.path,
        icon: item.meta.icon ? () => h(item.meta.icon) : null,
        label: item.meta.title,
      };

      if (item.children && !item.hideChildrenInMenu) {
        // @ts-ignore
        newItem.children = getMenuData(item.children, [...parentKeys, item.path]);
      } else {
        getMenuData(
          item.children,
          selectedKey ? parentKeys : [...parentKeys, item.path],
          selectedKey || item.path,
        );
      }
      menuData.push(newItem);
    } else if (!item.hideInMenu && !item.hideChildrenInMenu && item.children) {
      menuData.push(...getMenuData(item.children, [...parentKeys, item.path]));
    }
  }

  return menuData;
};

const state = reactive({
  menuData: getMenuData(routes),
  selectedKeys: [],
  openKeys: [],
});

watch(locale, () => {
  state.menuData = getMenuData(routes);
});

watch(
  route,
  (value, oldValue) => {
    const selectedKeys = SELECTED_KEYS_MAP[route.path];
    const openKeys = OPEN_KEYS_MAP[route.path];

    if (selectedKeys && openKeys) {
      state.selectedKeys = selectedKeys;
      // 解决首次刷新，菜单折叠的时候会自动弹出的问题
      state.openKeys = props.collapsed && !oldValue ? [] : [...state.openKeys, ...openKeys];
    } else {
      state.selectedKeys = [];
    }
  },
  { immediate: true },
);

const handleSelect = ({ key }) => {
  router.push(key);
};
</script>

<style lang="less" scoped>
.ant-menu {
  &.ant-menu-dark {
    background: var(--colorBgContainer);

    :deep(.ant-menu-inline) {
      background: var(--colorBgContainer);
    }
  }
}
</style>
