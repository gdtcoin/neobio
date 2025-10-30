import { message } from 'ant-design-vue';
import axios from 'axios';

import { HttpCode } from '@/admin/enums';
import { useUserStore } from '@/admin/store/user';

const request = axios.create({
  timeout: 20000,
  // baseURL: process.env.API_URL,
  baseURL: process.env.NODE_ENV === 'production' ? '/' : process.env.API_URL,
});

request.interceptors.request.use(
  (config) => {
    const userStore = useUserStore();

    config.headers['token'] = userStore.token || 'default';

    return config;
  },
  (err) => {
    Promise.reject(err);
  },
);

request.interceptors.response.use(
  (res) => {
    const { data } = res;
    const userStore = useUserStore();

    if (data.code === HttpCode.Success) {
      return Promise.resolve(data);
    }

    if (data.code === HttpCode.Unauthorized) {
      userStore.logout();
      message.error('登录已过期，请重新登录');
      return Promise.reject(data);
    }

    message.error(data.message);

    return Promise.reject(data);
  },
  (err) => {
    message.error(err.message);

    return Promise.reject(err);
  },
);

export default request;
