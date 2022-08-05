const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  // https://github.com/ChainSafe/web3.js/issues/3018#issuecomment-534267020
  module: {
    exprContextCritical: false,
  },
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
    // https://stackoverflow.com/questions/39352703/webpack-historyapifallback-configuration-for-deep-routes
    publicPath: '/',
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  // devServer: {
  //   // reference: https://webpack.js.org/configuration/dev-server/#devserverhttps
  //   // https: {
  //   //   key: fs.readFileSync('localhost-key.pem'),
  //   //   cert: fs.readFileSync('localhost.pem'),
  //   // },
  //   historyApiFallback: {
  //     index: '/index.html'
  //   },
  // },
  // https://github.com/webpack/webpack-dev-server/issues/216#issuecomment-309436276
  devServer: { historyApiFallback: { index: '' } },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: ('index.html'), to: 'index.html' },
        { from: ('index.css'), to: 'index.css' },
        { from: ('util.js'), to: 'util.js' },
        { from: ('parts.html'), to: 'parts.html' },
        { from: ('pages'), to: 'pages' },
        { from: ('parts_index.js'), to: 'parts_index.js' },
        { from: ('parts-bootstrap.js'), to: 'parts-bootstrap.js' },
        { from: ('parts.css'), to: 'parts.css' },
        { from: ('favicon.ico'), to: 'favicon.ico' },
      ]
    }),
  ],
};
