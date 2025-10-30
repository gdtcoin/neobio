import axios from 'axios';
import { showToast } from 'vant';

import { closeLoading, showLoading } from '@/app/components';
import { HttpCode } from '@/app/types';

let loadCount = 0;

const request = axios.create({
  timeout: 30000,
  // baseURL: process.env.API_URL,
  baseURL: process.env.NODE_ENV === 'production' ? '/' : process.env.API_URL,

  loading: false,
  toast: true,
});

request.interceptors.request.use(
  (config) => {
    if (config.loading && ++loadCount > 0) {
      showLoading();
    }

    return config;
  },
  (err) => {
    Promise.reject(err);
  },
);

request.interceptors.response.use(
  (res) => {
    const { config, data } = res;

    if (config.loading && --loadCount <= 0) {
      closeLoading();
    }

    if (data.code === HttpCode.SUCCESS) {
      return data;
    }

    if (config.toast) {
      showToast(data.message);
    }

    return Promise.reject(data);
  },
  (err) => {
    const { config } = err;

    if (config.loading && --loadCount <= 0) {
      closeLoading();
    }

    if (config.toast) {
      showToast(err.message);
    }

    return Promise.reject(err);
  },
);

export default request;
