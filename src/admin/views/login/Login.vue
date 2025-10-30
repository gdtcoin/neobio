<template>
  <div class="flex h-screen items-center justify-center">
    <div class="fixed top-[16px] right-[16px]">
      <Settings />
    </div>

    <div>
      <div class="mb-[30px] flex items-center justify-center gap-3">
        <img src="@/admin/assets/images/logo.png" alt="logo" width="48" />
        <span class="text-xl font-bold">NeoBio</span>
      </div>

      <a-form :model="form" autocomplete="off" class="w-[360px]" @finish="onFinish">
        <a-form-item name="username" :rules="{ required: true, message: '' }">
          <a-input v-model:value="form.username" :placeholder="$t('username')">
            <template #prefix>
              <UserOutlined />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item name="password" :rules="{ required: true, message: '' }">
          <a-input-password v-model:value="form.password" :placeholder="$t('password')">
            <template #prefix>
              <LockOutlined />
            </template>
          </a-input-password>
        </a-form-item>

        <div class="my-[30px] flex items-center justify-between">
          <a-checkbox v-model:checked="form.remember">
            {{ $t('rememberMe') }}
          </a-checkbox>

          <a class="!text-primary">
            {{ $t('forgotPassword') }}
          </a>
        </div>

        <a-button type="primary" html-type="submit" block>
          {{ $t('login') }}
        </a-button>
      </a-form>
    </div>
  </div>
</template>
<script setup lang="ts">
import { LockOutlined, UserOutlined } from '@ant-design/icons-vue';
import { useStorage } from '@vueuse/core';
import { reactive } from 'vue';
import { useRouter } from 'vue-router';

import Settings from '@/admin/layouts/Settings.vue';
import { useUserStore } from '@/admin/store/user';

const loginInfo = useStorage('loginInfo', { username: '', password: '' });

const userStore = useUserStore();
const router = useRouter();

const form = reactive({
  username: loginInfo.value.username,
  password: loginInfo.value.password,
  remember: true,
});

const onFinish = async () => {
  await userStore.login({
    username: form.username,
    password: form.password,
  });

  loginInfo.value = form;

  router.push('/');
};
</script>
