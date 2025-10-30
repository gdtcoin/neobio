export default {
  plugins: {
    autoprefixer: {},
    '@tailwindcss/postcss': {},
    'postcss-pxtorem': {
      rootValue: 37.5,
      propList: ['*'],
    },
  },
};
