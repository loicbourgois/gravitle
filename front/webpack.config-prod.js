const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  // https://github.com/ChainSafe/web3.js/issues/3018#issuecomment-534267020
  module: {
    exprContextCritical: false,
  },
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "../docs"),
    filename: "bootstrap.js",
    // https://stackoverflow.com/questions/39352703/webpack-historyapifallback-configuration-for-deep-routes
    publicPath: '/',
  },
  mode: 'production',
  experiments: {
    asyncWebAssembly: true,
  },
  // devServer: { historyApiFallback: { index: '' } },
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
      ]
    }),
  ],
};
