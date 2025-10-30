import { useStorage } from '@vueuse/core';
import { Locale } from 'vant';
import { createI18n } from 'vue-i18n';

import zhCN from './zh-CN';

const DEFAULT_LANG = 'zh-CN';
const localeStorage = useStorage('locale', DEFAULT_LANG);

// https://github.com/lokalise/i18n-ally/tree/main/locales
export const LANGUAGE_LIST = [
  // { name: '🇨🇳 繁體中文', value: 'zh-TW' },
  { name: '🇨🇳 简体中文', value: 'zh-CN', shortName: '🇨🇳 中文' },
  { name: '🇺🇸 English', value: 'en', shortName: '🇺🇸 EN' },
  // { name: '🇯🇵 日本語', value: 'ja' },
  // { name: '🇰🇷 한국어', value: 'ko' },
  // { name: '🇹🇭 ไทย', value: 'th' },
  // { name: '🇷🇺 Русский', value: 'ru' },
  // { name: '🇸🇦 العربية', value: 'ar' },
  // { name: '🇻🇳 Tiếng Việt', value: 'vi' },
  // { name: '🇫🇷 Français', value: 'fr' },
  // { name: '🇩🇪 Deutsch', value: 'de' },
  // { name: '🇲🇾 Bahasa Melayu', value: 'ms' },
  // { name: '🇮🇳 हिन्दी', value: 'hi' },
  // { name: '🇪🇸 Español', value: 'es' },
];

function setupLocale() {
  const locale = localeStorage.value;

  const i18n = createI18n({
    legacy: false,
    locale,
    fallbackLocale: DEFAULT_LANG,
    messages: {
      'zh-CN': { ...zhCN },
    },
  });

  setLocale(locale);

  return i18n;
}

export const i18n = setupLocale();

async function loadLocaleMessages(locale) {
  const supported = LANGUAGE_LIST.map(({ value }) => value);
  if (!supported.includes(locale)) {
    locale = DEFAULT_LANG;
  }

  return await import(`./${locale}.ts`);
}

export async function setLocale(locale) {
  const messages = await loadLocaleMessages(locale);

  document.querySelector('html').setAttribute('lang', locale);

  i18n.global.setLocaleMessage(locale, messages.default);
  i18n.global.locale.value = locale;

  setVantLocale(locale);
  localeStorage.value = locale;
}

export function setVantLocale(locale) {
  const vantLocales = {
    'zh-CN': require('vant/es/locale/lang/zh-CN').default,
    en: require('vant/es/locale/lang/en-US').default,
  };

  Locale.use(locale, vantLocales[locale] || vantLocales['en']);
}
