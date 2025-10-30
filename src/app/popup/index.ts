import { showPopup } from './core';

export { showPopup };

export const showCrowdfundingPopup = (props) => {
  showPopup(require('./CrowdfundingPopup.vue').default, props);
};
