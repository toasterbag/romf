module.exports = {
  devServer: {
    allowedHosts: [
      "192.168.1.117",
      "localhost"
    ],
  },
  configureWebpack: {
    module: {
      rules: [
        {
          //test: /\.worker\.js$/,
          //use: { loader: 'worker-loader' }
        }
      ]
    }
  },
  chainWebpack: config => {
    ["vue-modules", "vue", "normal-modules", "normal"].forEach((match) => {
      config.module.rule('scss').oneOf(match).use('sass-loader')
        .tap(opt => Object.assign(opt, { data: `@import "@/variables.scss";` }))
    })
  },
  css: {
    loaderOptions: {
      sass: {
        data: `@import "@/variables.scss"`,
      },
    },
  },
};
