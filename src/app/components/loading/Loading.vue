<template>
  <div v-if="show" class="base-loading" @touchmove.stop.prevent>
    <div class="base-loading-wrapper">
      <div class="base-loading-spin" />
      <div v-if="message" class="base-loading-text">{{ message }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps({
  message: {
    type: String,
    default: '',
  },
  destroy: {
    type: Function,
    default: () => {},
  },
});

const show = ref(false);

const open = () => {
  show.value = true;
};

const close = () => {
  show.value = false;
  props.destroy();
};

defineExpose({
  open,
  close,
});
</script>

<style lang="less" scoped>
@keyframes spin-rotate {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}

@spin-color: #ccc;

.base-loading {
  position: fixed;
  inset: 0;
  z-index: 10000;
  display: flex;
  align-items: center;
  justify-content: center;

  &-wrapper {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100px;
    height: 100px;
    color: white;
    text-align: center;
    background: rgb(0 0 0 / 70%);
    border-radius: 8px;
  }

  &-spin {
    width: 32px;
    height: 32px;
    margin: 0 auto;
    border: 5px solid transparent;
    border-top-color: @spin-color;
    border-bottom-color: @spin-color;
    border-left-color: @spin-color;
    border-radius: 50%;
    animation: spin-rotate 0.8s infinite linear;
  }

  &-text {
    margin-top: 12px;
    font-size: 13px;
  }
}
</style>
