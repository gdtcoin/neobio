import Antd from 'ant-design-vue';
import { createPinia } from 'pinia';
import { createApp } from 'vue';

import App from './App.vue';
import components from './components';
import { i18n } from './locales';
import router from './router';
import { useThemeStore } from './store/theme';

import 'ant-design-vue/dist/reset.css';
import './styles/index.less';
import './styles/tailwind.css';

// @ts-ignore
window.env = process.env;

const bootstrap = () => {
  const pinia = createPinia();
  const app = createApp(App);

  app.use(Antd).use(i18n).use(pinia).use(router).use(components);

  const themeStore = useThemeStore();
  themeStore.initTheme();
  themeStore.initColorPrimary();

  app.mount('#app');
};

bootstrap();
