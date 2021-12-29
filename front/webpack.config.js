const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: ('index.html'), to: 'index.html' },
        { from: ('index.css'), to: 'index.css' },
        { from: ('util.js'), to: 'util.js' },
        // { from: ('index.js'), to: 'index.js' },
        // { from: ('bootstrap.js'), to: 'bootstrap.js' },
        { from: ('parts.html'), to: 'parts.html' },
        { from: ('parts_index.js'), to: 'parts_index.js' },
        { from: ('parts-bootstrap.js'), to: 'parts-bootstrap.js' },
        { from: ('parts.css'), to: 'parts.css' },
      ]
    }),
  ],
};
