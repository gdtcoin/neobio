<template>
  <a-modal
    v-model:open="isOpen"
    :title="isEdit ? '编辑用户' : '新增用户'"
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
      <a-form-item label="用户名" name="name" :rules="[{ required: true, message: '' }]">
        <a-input v-model:value="formData.name" />
      </a-form-item>
      <a-form-item label="邮箱" name="email" :rules="[{ required: true, message: '' }]">
        <a-input v-model:value="formData.email" />
      </a-form-item>
      <a-form-item label="状态" name="status" :rules="[{ required: true, message: '' }]">
        <a-select
          v-model:value="formData.status"
          placeholder="请选择状态"
          :options="UserStatusOptions"
        />
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import type { FormInstance } from 'ant-design-vue';
import { computed, reactive, ref, useTemplateRef } from 'vue';

import { UserStatus, UserStatusOptions } from '@/admin/enums';
import { showSuccessMessage, sleep } from '@/admin/utils';

const INITIAL_FORM_DATA = {
  name: '',
  email: '',
  id: '',
  status: UserStatus.Enabled,
};

const formRef = useTemplateRef<FormInstance>('formRef');
const formData = reactive({ ...INITIAL_FORM_DATA });
const isOpen = ref(false);
const isConfirmLoading = ref(false);

const isEdit = computed(() => formData.id);

const openModal = (record = INITIAL_FORM_DATA) => {
  Object.assign(formData, record);
  isOpen.value = true;
};

const handleOk = async () => {
  await formRef.value.validate();

  try {
    isConfirmLoading.value = true;

    await sleep(3000);

    isOpen.value = false;
    showSuccessMessage();
  } finally {
    isConfirmLoading.value = false;
  }
};

defineExpose({ openModal });
</script>
