<template>
  <div class="locale-wrapper" @click="state.show = true">
    <span>{{ localeName }}</span>
    <img src="@/app/assets/images/arrow.png" class="arrow" />
  </div>
  <van-action-sheet v-model:show="state.show" :actions="LANGUAGE_LIST" @select="handleSelect" />
</template>

<script setup lang="ts">
import { computed, reactive } from 'vue';
import { useI18n } from 'vue-i18n';

import { LANGUAGE_LIST, setLocale } from '@/app/locales';

const { locale } = useI18n();
const localeName = computed(
  () => LANGUAGE_LIST.find((item) => item.value === locale.value)?.shortName,
);

const state = reactive({
  show: false,
});

const handleSelect = (action) => {
  setLocale(action.value);
  state.show = false;
};
</script>

<style lang="less" scoped>
.locale-wrapper {
  display: inline-flex;
  gap: 3px;
  align-items: center;
  justify-content: center;
  height: 22px;
  padding: 0 7px;
  font-size: 12px;
  color: var(--text-primary);
  text-align: center;
  background: rgba(#dca75b, 0.4);
  background-size: 100% 100%;
  border-radius: 100px;

  .arrow {
    width: 6px;
  }
}
</style>
