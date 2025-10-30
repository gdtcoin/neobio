import { useStorage } from '@vueuse/core';
import { theme } from 'ant-design-vue';
import { defineStore } from 'pinia';

import { STORAGE_KEY } from '@/admin/constants';
import { AppTheme } from '@/admin/enums';

const COLOR_PRIMARY_LIST = [
  '#f7234f',
  '#FA541C',
  '#FAAD14',
  '#13C2C2',
  '#52C41A',
  '#1890FF',
  '#2F54EB',
  '#722ED1',
];

const isDarkTheme = (theme) => theme === AppTheme.Dark;

let cssVarStyleEl;

export const useThemeStore = defineStore('theme', {
  state: () => ({
    theme: useStorage(STORAGE_KEY.theme, AppTheme.Light),
    colorPrimary: useStorage(STORAGE_KEY.colorPrimary, '#1890FF'),
  }),
  actions: {
    setTheme(theme) {
      this.theme = theme;

      document.documentElement.setAttribute('data-theme', this.theme);
    },
    toggleTheme() {
      this.setTheme(this.isDarkTheme ? AppTheme.Light : AppTheme.Dark);
    },
    setColorPrimary(color) {
      this.colorPrimary = color;

      // document.documentElement.style.setProperty('--primary-color', this.colorPrimary);
    },
    initTheme() {
      this.setTheme(this.theme);
    },
    initColorPrimary() {
      this.setColorPrimary(this.colorPrimary);
    },
    applyAntdCssVariablesViaStyle(token) {
      const cssLines = [];

      Object.entries(token).forEach(([key, value]) => {
        const cssVar = `--${key}`;

        if (typeof value === 'number') {
          const isPx = /font|radius|width|height|size|margin|padding/i.test(key);
          value = isPx ? `${value}px` : value;
        }

        cssLines.push(`${cssVar}: ${value};`);
      });

      const css = `:root {\n  ${cssLines.join('\n  ')}\n}`;

      if (cssVarStyleEl) {
        cssVarStyleEl.innerHTML = css;
        return;
      }

      cssVarStyleEl = document.createElement('style');
      document.head.appendChild(cssVarStyleEl);
    },
  },
  getters: {
    colorPrimaryList: () => COLOR_PRIMARY_LIST,
    isDarkTheme: (state) => isDarkTheme(state.theme),
    antdThemeConfig: (state) => ({
      algorithm: isDarkTheme(state.theme) ? theme.darkAlgorithm : theme.defaultAlgorithm,
      token: {
        colorPrimary: state.colorPrimary,
        controlHeight: 34,
        borderRadius: 4,
      },
    }),
  },
});
