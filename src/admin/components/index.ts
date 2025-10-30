import Icon from './Icon.vue';
import SvgIcon from './SvgIcon.vue';

export { Icon, SvgIcon };

export default {
  install(app) {
    app.component('Icon', Icon);
    app.component('SvgIcon', SvgIcon);
  },
};
