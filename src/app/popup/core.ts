import { createApp } from 'vue';

import { i18n } from '@/app/locales';
import router from '@/app/router';

export const showPopup = (Com, props = {}) => {
  let root = document.createElement('div');
  document.body.appendChild(root);
  let app;

  const destroy = () => {
    if (app) {
      app.unmount();
      app = null;
    }

    if (root) {
      document.body.removeChild(root);
      root = null;
    }
  };

  app = createApp(Com, {
    ...props,
    onClosed() {
      destroy();
    },
  });

  const instance = app.use(i18n).use(router).mount(root);

  instance.open();
  return instance;
};
