export default {
  '*.{js,mjs,cjs,ts,vue}': ['eslint --fix', 'prettier --write'],
  '*.{less,css,vue,html}': ['stylelint --fix', 'prettier --write'],
  '*.{md,json}': ['prettier --write'],
};
