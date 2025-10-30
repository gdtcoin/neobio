import { onMounted } from 'vue';
import { useRouter } from 'vue-router';

import { useStore } from '@/app/store';

export const useWallet = () => {
  const store = useStore();
  const router = useRouter();

  onMounted(async () => {
    if (!store.walletAddress) {
      store.connectWallet();
    }

    setTimeout(() => {
      if (!store.walletAddress) {
        router.push('/');
      }
    }, 50);
  });
};
