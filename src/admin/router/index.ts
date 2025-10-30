import NProgress from 'nprogress';
import { createRouter, createWebHashHistory } from 'vue-router';

import 'nprogress/nprogress.css';

import { RoleName } from '@/admin/enums';
import { useUserStore } from '@/admin/store/user';

import routes from './routes';

NProgress.configure({ showSpinner: false });

const router = createRouter({
  history: createWebHashHistory(),
  routes,
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition;
    }

    return { top: 0 };
  },
});

router.beforeEach(async (to, from, next) => {
  NProgress.start();

  const userStore = useUserStore();

  if (userStore.token) {
    if (!userStore.userInfo) {
      await useUserStore().fetchUserInfo();
    }

    if (to.meta && to.meta.roles && !userStore.hasRole(to.meta.roles as RoleName[])) {
      next('/404');
      return;
    }

    next();
  } else {
    if (to.path !== '/user/login') {
      next('/user/login');
      return;
    }

    next();
  }
});

router.afterEach(() => {
  NProgress.done();
});

export default router;
