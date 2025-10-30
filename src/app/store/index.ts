import { defineStore } from 'pinia';

import { loginApi } from '@/app/api';
import { formatWallet } from '@/app/utils';
import * as web3 from '@/web3';

// 管理员钱包地址列表
const ADMIN_ADDRESSES = [
  'EaeUAHS4prbWNezkcCgs1hzHGTyHGKKXL9XzhXfNnPYQ',
  '7dEiDwc8xzTnpbwxBjTbiLYBQ6PsVMPEkvXXttMB4ERy',
].map((addr) => addr.toLowerCase());

export const useStore = defineStore('store', {
  state: () => {
    return {
      walletAddress: '',
      lpTokenBalance: '0',
      userInfo: null,
    };
  },
  actions: {
    setWalletAddress(walletAddress) {
      this.walletAddress = walletAddress;
    },
    async connectWallet() {
      const walletAddress = await web3.connectWallet();
      this.setWalletAddress(walletAddress);

      await this.fetchUserInfo();
    },
    async fetchUserInfo() {
      const res = await loginApi({ sol_address: this.walletAddress });
      this.userInfo = res.data;

      web3.setSuperiorAddressConfig(res.data.parent_address);
    },
    getUserInfo() {
      return this.userInfo;
    },
  },
  getters: {
    formattedWallet: (state) => formatWallet(state.walletAddress),
    isAdmin: (state) => ADMIN_ADDRESSES.includes(state.walletAddress.toLowerCase()),
  },
});
