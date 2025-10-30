<template>
  <a-layout-header
    :style="{
      background: token.colorBgContainer,
      borderBottom: `1px solid ${token.colorBorderSecondary}`,
    }"
  >
    <div class="flex items-center gap-2">
      <icon
        :icon="props.collapsed ? MenuUnfoldOutlined : MenuFoldOutlined"
        @click="props.handleCollapse"
      />

      <icon v-if="isPC" :icon="ReloadOutlined" @click="handleReload" />

      <a-breadcrumb v-if="isPC" style="margin-left: 4px">
        <a-breadcrumb-item
          v-for="(breadcrumb, index) in breadcrumbs"
          :key="index"
          :style="{ cursor: breadcrumb.clickable ? 'pointer' : null }"
          @click="breadcrumb.clickable && navigateTo(breadcrumb)"
        >
          <!-- <component :is="breadcrumb.icon" v-if="breadcrumb.icon" /> -->
          <span>{{ breadcrumb.name }}</span>
        </a-breadcrumb-item>
      </a-breadcrumb>
    </div>

    <Settings />
  </a-layout-header>
</template>

<script setup lang="ts">
import { MenuFoldOutlined, MenuUnfoldOutlined, ReloadOutlined } from '@ant-design/icons-vue';
import { theme } from 'ant-design-vue';
import { computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { useDevice } from '@/admin/hooks';
import { reloadEventBus } from '@/admin/utils/eventBus';

import Settings from './Settings.vue';

const { useToken } = theme;

const props = defineProps({
  handleCollapse: {
    type: Function,
    required: true,
  },
  collapsed: {
    type: Boolean,
    required: true,
  },
});

const route = useRoute();
const router = useRouter();
const { token } = useToken();
const { isPC } = useDevice();

const breadcrumbs = computed(() => {
  const pathArray = route.path.split('/').filter((p) => p);
  return pathArray.map((path, index) => {
    const fullPath = '/' + pathArray.slice(0, index + 1).join('/');
    const matchedRoute = router.getRoutes().find((r) => r.path === fullPath);

    const isLastItem = index === pathArray.length - 1;
    const isLayout = matchedRoute?.meta?.layout || false;
    const name = matchedRoute?.meta?.title;

    return {
      name,
      path: fullPath,
      icon: matchedRoute?.meta?.icon || null,
      clickable: !isLastItem && !isLayout,
      children: matchedRoute?.children || [],
    };
  });
});

const handleReload = () => {
  reloadEventBus.emit();
};

const navigateTo = (breadcrumb) => {
  const { children, path } = breadcrumb;

  if (children.length > 0) {
    router.push(children[0].path);
  } else {
    router.push(path);
  }
};
</script>

<style scoped>
.ant-layout-header {
  @apply flex items-center justify-between px-[16px];
}
</style>
