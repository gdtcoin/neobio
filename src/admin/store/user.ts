import { useStorage } from '@vueuse/core';
import { defineStore } from 'pinia';

import { loginApi } from '@/admin/api';
import { STORAGE_KEY } from '@/admin/constants';
import { RoleName } from '@/admin/enums';
import router from '@/admin/router';

const defaultUserInfo = {
  username: 'admin',
  roles: [RoleName.Admin],
  avatar: 'https://joeschmoe.io/api/v1/random',
  description: 'admin',
};

export const useUserStore = defineStore('user', {
  state: () => {
    return {
      token: useStorage(STORAGE_KEY.token, ''),
      userInfo: useStorage('userInfo', defaultUserInfo),
    };
  },
  actions: {
    async fetchUserInfo() {
      // const res = await fetchUserInfoApi();
      // this.userInfo = res.data;
    },
    async login(data) {
      const res = await loginApi(data);

      this.token = res.data;
      this.userInfo = {
        ...defaultUserInfo,
        ...data,
      };
    },
    logout() {
      this.token = '';
      this.userInfo = null;

      router.push('/user/login');
    },
    hasRole(roles) {
      if (!this.token) {
        return false;
      }

      return this.userInfo.roles.some((role) => roles.includes(role));
    },
  },
});
