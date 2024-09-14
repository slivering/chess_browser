const path = require("path");
const CopyWebPackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "development",
  name: "chess_browser",
  entry: {
    index: "./static/bootstrap.js"
  },
  output: {
    path: dist,
    filename: "bootstrap.js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyWebPackPlugin([
      path.resolve(__dirname, "static")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      forceMode: "development",
      outName: "chess_browser"
    })
  ]
};
