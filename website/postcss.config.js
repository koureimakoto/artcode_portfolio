module.exports = (ctx) => ({
    parser: ctx.parser ? 'sugarss' : false,
    map: ctx.env === 'development' ? ctx.map : false,
    plugins: {
      'postcss-simple-vars': { 
          variables: () => require('./styles.constants.cjs')
      },
      'tailwindcss': {},
      'autoprefixer': {},
      'postcss-import': {},
      'postcss-preset-env': {},
      'postcss-utilities': {},
      'postcss-nested': {},
      'postcss-mixins': {},
      cssnano: ctx.env === 'production' ? {} : false
    }
  })
