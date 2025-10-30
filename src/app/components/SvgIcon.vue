<template>
  <svg class="svg-icon" :class="'svg-icon-' + props.name" v-bind="$attrs">
    <use :xlink:href="'#' + props.name" />
  </svg>
</template>

<script setup lang="ts">
import svgIcons from '@/app/assets/svgIcons';

// https://free-icons.github.io/free-icons/
const props = defineProps({
  name: {
    type: String,
    required: true,
    default: '',
    validator(val) {
      return svgIcons[val];
    },
  },
});
</script>

<style lang="less" scoped>
.svg-icon {
  display: inline-block;
  width: 16px;
  height: 16px;
  fill: currentcolor;
}
</style>
