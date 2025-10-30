<template>
  <a-modal
    v-model:open="isOpen"
    title="修改密码"
    :maskClosable="false"
    :confirmLoading="isConfirmLoading"
    :width="600"
    @ok="handleOk"
  >
    <a-form
      ref="formRef"
      :key="isOpen"
      :model="formData"
      :label-col="{ style: { width: '100px' } }"
      class="!pt-[20px]"
      labelAlign="left"
    >
      <a-form-item label="旧密码" name="oldPassword" :rules="[{ required: true, message: '' }]">
        <a-input-password v-model:value="formData.oldPassword" />
      </a-form-item>
      <a-form-item label="新密码" name="newPassword" :rules="[{ required: true, message: '' }]">
        <a-input-password v-model:value="formData.newPassword" />
      </a-form-item>
      <a-form-item
        label="确认密码"
        name="confirmPassword"
        :rules="[{ required: true, message: '' }]"
      >
        <a-input-password v-model:value="formData.confirmPassword" />
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import type { FormInstance } from 'ant-design-vue';
import { message } from 'ant-design-vue';
import { reactive, ref, useTemplateRef } from 'vue';

import { updatePasswordApi } from '@/admin/api';
import { useUserStore } from '@/admin/store/user';
import { showSuccessMessage } from '@/admin/utils';

const userStore = useUserStore();

const INITIAL_FORM_DATA = {
  oldPassword: '',
  newPassword: '',
  confirmPassword: '',
};

const formRef = useTemplateRef<FormInstance>('formRef');
const formData = reactive({ ...INITIAL_FORM_DATA });
const isOpen = ref(false);
const isConfirmLoading = ref(false);

const openModal = () => {
  Object.assign(formData, INITIAL_FORM_DATA);
  isOpen.value = true;
};

const handleOk = async () => {
  await formRef.value.validate();

  try {
    isConfirmLoading.value = true;

    if (formData.newPassword !== formData.confirmPassword) {
      message.error('新密码与确认密码不一致');
      return;
    }

    await updatePasswordApi({
      old_password: formData.oldPassword,
      new_password: formData.newPassword,
      username: userStore.userInfo.username,
    });

    isOpen.value = false;
    showSuccessMessage();
  } finally {
    isConfirmLoading.value = false;
  }
};

defineExpose({ openModal });
</script>
