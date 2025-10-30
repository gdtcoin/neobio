export * from './loading';

import BottomBar from './bottomBar/BottomBar.vue';
import InfiniteLoading from './InfiniteLoading.vue';
import Locale from './Locale.vue';
import SvgIcon from './SvgIcon.vue';
import TopBar from './TopBar.vue';

export default {
  install(app) {
    app.component('BottomBar', BottomBar);
    app.component('TopBar', TopBar);
    app.component('InfiniteLoading', InfiniteLoading);
    app.component('SvgIcon', SvgIcon);
    app.component('Locale', Locale);
  },
};
