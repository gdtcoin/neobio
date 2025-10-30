import { useClipboard as _useClipboard } from '@vueuse/core';
import { message } from 'ant-design-vue';
import { ref } from 'vue';

import { i18n } from '@/admin/locales';

export const useClipboard = () => {
  const copyToClipboard = async (value = '', successText = i18n.global.t('copied')) => {
    const source = ref(value);
    const { copy } = _useClipboard({ source });

    await copy(source.value);
    message.success(successText);
  };

  return {
    copy: copyToClipboard,
  };
};
