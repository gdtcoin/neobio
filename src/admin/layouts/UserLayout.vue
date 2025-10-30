<template>
  <a-config-provider :theme="{ token: { controlHeight: 40 } }">
    <div
      class="h-screen bg-[var(--colorBgContainer)] bg-[url('@/admin/assets/images/bg.svg')] bg-no-repeat"
      style="background-size: 100% 100%"
    >
      <router-view />
    </div>
  </a-config-provider>
</template>
