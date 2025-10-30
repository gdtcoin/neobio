import { useMediaQuery } from '@vueuse/core';

export function useDevice() {
  const isMobile = useMediaQuery('(max-width: 767px)');
  const isPC = useMediaQuery('(min-width: 768px)');

  return {
    isMobile,
    isPC,
  };
}
