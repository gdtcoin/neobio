export default {
  presets: [
    [
      '@babel/preset-env',
      {
        useBuiltIns: 'usage',
        modules: 'auto',
        corejs: { version: 3, proposals: true },
        targets: {
          node: 'current',
        },
      },
    ],
  ],
  plugins: ['@babel/plugin-transform-runtime', '@vue/babel-plugin-jsx'],
  exclude: [/core-js/],
};
