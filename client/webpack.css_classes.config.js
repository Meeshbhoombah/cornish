//
// `webpack.css_classes.config.js`
// The purpose of this webpack config is to bundle and transpile the imported 
// CSS styles found in `css/styles.css` for usage within the main Rust logic.
//
// All client styling is aggregated in `styles.css` with `@import` statements.
//
// When the `geenerate css` script is executed, this module starts in 
// `/static/index.css_clases.ts`.
//  leverages `file-loader` for resolving `import` and `require` 
// within the `index.css_classes.ts`. `style-loader` injects CSS into the DOM 
// via lazy-loaded `<style></style>`s (as per default, can be changed, but using
// recommended config). `css-loader` resolves `@import`s and `urls()` within `.css` 
// files, as though they were `.ts` (or `.js`) files.
//
// The end result of this process is piped into `postcss-loader`, which 
// generates `/src/generated/css_classes.rs` for usage within the client's Rust 
// code after applying both TailwindCSS and PostCSS. `postcss.config.js` contains
// this portion of the pipeline's configuration.
//
// Usage:
//  Styling can be added to `/css`. 
//
//  Import the styles into `styles.css`.
//
//  CSS bindings generated from the pipeline can be used in Rust.
//      ```/example/lib.rs
//      use generated::css_classes::C;
//      pub fn view() {
//          div![
//                C![
//                    IF!(not(model.in_prerendering) => C.fade_in),
//                    C.min_h_screen,
//                    C.flex,
//                    C.flex_col,
//                ],
//                match model.page {
//                    Page::Home => page::home::view(&model.base_url),
//                    Page::About => page::about::view(),
//                    Page::NotFound => page::not_found::view(),
//                },
//                page::partial::header::view(model),
//                page::partial::footer::view(),
//          ]
//      }
//
//      ```
//


const path = require("path");

const WebpackBar = require("webpackbar");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");



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
