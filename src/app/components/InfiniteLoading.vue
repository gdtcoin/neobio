<template>
  <div ref="root" class="infinite">
    <template v-if="state.status === Status.Loading">
      <slot name="loading">
        <div class="loading-wrapper">
          <van-loading color="#d09043">{{ $t('loading') }}</van-loading>
        </div>
      </slot>
    </template>

    <template v-if="state.status === Status.NoMore">
      <slot name="no-more">
        <div class="no-more-wrapper">
          <div class="text">{{ props.noMoreText || $t('noMoreData') }}</div>
        </div>
      </slot>
    </template>

    <template v-if="state.status === Status.Empty">
      <slot name="empty">
        <div class="empty-wrapper">
          <img src="@/app/assets/images/empty.png" />
          <div class="text">{{ props.emptyText || $t('noData') }}</div>
        </div>
      </slot>
    </template>

    <template v-if="state.status === Status.Error">
      <slot name="error">
        <div class="error-wrapper">
          <img src="@/app/assets/images/empty.png" />
          <div class="text">{{ props.errorText || $t('noData') }}</div>
        </div>
      </slot>
    </template>
  </div>
</template>

<script setup lang="ts">
import { onActivated, onBeforeUnmount, onDeactivated, onMounted, reactive, ref } from 'vue';

const Status = {
  Ready: 0,
  Loading: 1,
  NoMore: 2,
  Empty: 3,
  Error: 4,
};

const props = defineProps({
  distance: {
    type: Number,
    default: 10,
  },
  immediate: {
    type: Boolean,
    default: false,
  },
  noMoreText: {
    type: String,
  },
  emptyText: {
    type: String,
  },
  errorText: {
    type: String,
  },
  container: {
    type: String,
    default: '',
  },
});

const emits = defineEmits(['load']);

const state = reactive({
  scrollParent: null,
  status: Status.Ready,
  hasData: false,
});
const root = ref(null);

onMounted(() => {
  state.scrollParent = getScrollParent();

  setTimeout(() => {
    if (props.immediate) {
      load();
    } else {
      handleScroll();
    }

    addListener();
  }, 1);

  addListener();
});

onBeforeUnmount(() => {
  removeListener();
});

onActivated(() => {
  addListener();
});

onDeactivated(() => {
  if (state.status === Status.Loading) {
    state.status = Status.Ready;
  }

  removeListener();
});

const isVisible = (el) => {
  const clientHeight =
    state.scrollParent === window
      ? window.innerHeight
      : state.scrollParent.getBoundingClientRect().bottom;

  return el.value.getBoundingClientRect().top <= clientHeight + props.distance;
};

const getScrollParent = () => {
  let el = window;
  if (props.container) {
    // @ts-ignore
    el = document.querySelector(props.container);
  }

  return el;
};

const load = () => {
  if (state.status === Status.Ready) {
    state.status = Status.Loading;
    emits('load', callback);
  }
};

const handleScroll = () => {
  if (isVisible(root)) {
    load();
  }
};

const removeListener = () => {
  state.scrollParent.removeEventListener('scroll', handleScroll);
};

const addListener = () => {
  state.scrollParent.addEventListener('scroll', handleScroll);
};

const callback = (params, error) => {
  const list = params.list;
  const pageSize = params.pageSize || 1000;
  const page = params.pageNum || 1;

  if (error) {
    state.status = Status.Error;
    removeListener();
    return;
  }

  if (page === 1 && list.length === 0) {
    state.status = Status.Empty;
  } else if (list.length < pageSize) {
    state.status = Status.NoMore;
    removeListener();
  } else {
    state.status = Status.Ready;
  }
};
</script>

<style lang="less" scoped>
@height: 50px;

.infinite {
  .loading-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    height: @height;
    font-size: 13px;
  }

  .no-more-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    height: @height;
    color: var(--van-primary-color);
  }

  .empty-wrapper {
    padding: 40px 0;
    text-align: center;

    img {
      width: 76px;
      opacity: 0.9;
    }

    .text {
      margin-top: 16px;
      font-size: 13px;
      color: var(--van-primary-color);
      opacity: 0.85;
    }
  }

  :deep(.van-loading__spinner) {
    width: 16px;
    height: 16px;
  }
}
</style>
