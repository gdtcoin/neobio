<template>
  <a-button
    class="icon-component"
    v-bind="$attrs"
    type="text"
    shape="circle"
    @click="$emit('click', $event)"
  >
    <template #icon>
      <svg-icon v-if="typeof icon === 'string'" :name="icon" />
      <component :is="icon" v-else />
    </template>
  </a-button>
</template>

<script setup lang="ts">
defineProps({
  icon: {
    type: [String, Function],
    required: true,
  },
});

defineEmits(['click']);
</script>

<style lang="less" scoped>
.icon-component {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}
</style>
