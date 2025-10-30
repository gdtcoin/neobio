<template>
  <a-card>
    <a-form
      :label-col="{ style: { width: '120px' } }"
      :wrapper-col="{ span: 12 }"
      labelAlign="left"
    >
      <a-form-item label="用户名">
        <span class="text-primary font-bold">{{ userInfo.username }}</span>
      </a-form-item>
      <a-form-item label="头像">
        <a-avatar :src="userInfo.avatar" />
      </a-form-item>
      <a-form-item label="说明">
        <a-textarea v-model:value="userInfo.description" />
      </a-form-item>
      <a-form-item label="密码">
        <a-button type="primary" size="small" @click="handleUpdatePassword">修改密码</a-button>
      </a-form-item>
    </a-form>

    <PasswordModal ref="passwordModalRef" />
  </a-card>
</template>
<script setup lang="ts">
import { toRefs, useTemplateRef } from 'vue';

import { useUserStore } from '@/admin/store/user';

import PasswordModal from './PasswordModal.vue';

const userStore = useUserStore();
const passwordModalRef = useTemplateRef('passwordModalRef');

const { userInfo } = toRefs(userStore);

const handleUpdatePassword = () => {
  passwordModalRef.value.openModal();
};
</script>
