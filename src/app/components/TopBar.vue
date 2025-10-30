<template>
  <div>
    <header
      class="header"
      :class="{
        fixed,
      }"
    >
      <div class="header-left text-primary text-[13px]">
        <slot name="left">
          <div v-show="back" class="back-wrapper" @click="handleBack">
            <svg-icon name="back" />
          </div>
        </slot>
      </div>

      <div class="header-middle text-primary">
        <slot>
          {{ title }}
        </slot>
      </div>

      <div class="header-right text-primary">
        <slot name="right">
          <locale />
        </slot>
      </div>
    </header>

    <div v-if="fixed" class="header-placeholder" />
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();

const props = defineProps({
  title: {
    type: String,
    default: '',
  },
  back: {
    type: Boolean,
    default: true,
  },
  border: {
    type: Boolean,
    default: true,
  },
  to: {
    type: String,
    default: '',
  },
  fixed: {
    type: Boolean,
    default: true,
  },
});

const handleBack = () => {
  if (props.to) {
    router.push(props.to);
  } else {
    router.go(-1);
  }
};
</script>

<style lang="less" scoped>
.header {
  display: flex;
  align-items: center;
  height: 50px;
  color: var(--text-primary);
  text-align: center;
  background: black;

  &.fixed {
    position: fixed;
    top: 0;
    right: 0;
    left: 0;
    z-index: 11;
  }

  &-left {
    display: flex;
    align-items: center;
    width: 100px;
    height: 100%;
    padding-left: 12px;

    .back-wrapper {
      position: relative;

      &::after {
        position: absolute;
        inset: -10px;
        overflow: hidden;
        pointer-events: none;
        content: '';
        background-color: var(--van-black);
        border-radius: 50%;
        opacity: 0;
      }

      &:active::after {
        opacity: 0.1;
      }
    }
  }

  &-middle {
    flex: 1;
    font-size: 15px;
    font-weight: bold;
  }

  &-right {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    width: 100px;
    height: 100%;
    padding-right: 12px;
    overflow: hidden;
    font-size: 13px;
  }
}

.header-placeholder {
  box-sizing: content-box;
  height: 50px;
}
</style>
