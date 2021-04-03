const path = require("path");

const WebpackBar = require("webpackbar");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

// Webpack generates `css_classes.rs` with this config.
// This config is used in command `yarn generate:css_classes`.
// See `webpack.config.js` for more info about individual settings

// The purpose of this webpack config is to bundle the imported CSS styles 
// found in `/static/index.css_classes.ts`.
//
// The module leverages `file-loader` for resolving `import` and `require` 
// within the project's `.ts` files. `style-loader` injects CSS into the DOM
// via lazy-loaded `<style></style>`s (on default, can be changed, but is 
// recommended). `css-loader` resolves `@import`s and `urls()` as within 
// `.css` files, as though they were `.ts` (or `.js`) files.
//
// The end result of this process is piped into `postcss-loader`, which 
// generates `/src/generated/css_classes.rs` (the configuration for which can
// be found in `postcss.config.js`) for usage within the client's Rust code.
//

module.exports = (env, argv) => {
  return {
    entry: path.resolve(__dirname, "./static/index.css_classes.ts"),
    output: {
      path: path.resolve(__dirname, "dist"),
      filename: "css_classes.js",
    },
    plugins: [new WebpackBar(), new CleanWebpackPlugin()],
    module: {
      rules: [
        {
          test: /\.(jpg|jpeg|png|woff|woff2|eot|ttf|svg)$/,
          use: [
            {
              loader: "file-loader",
              options: {
                emitFile: false,
                name: "[path][name].[ext]",
              },
            },
          ],
        },
        {
          test: /\.ts$/,
          loader: "ts-loader?configFile=tsconfig.css_classes.json",
        },
        {
          test: /\.css$/,
          use: [
            "style-loader",
            "css-loader",
            {
              loader: "postcss-loader",
              options: {
                config: {
                  ctx: { mode: argv.mode },
                  path: __dirname,
                },
              },
            },
          ],
        },
      ],
    },
  };
};
