const path = require("path");
const dist = path.resolve(__dirname, "dist");

const WebpackBar = require("webpackbar");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = (env, argv) => {
  return {
    performance: {
        // Don't break compilation because of WASM file bigger than 244 KB.
        hints: false
    },
    entry: {
      app: path.resolve(__dirname, "index.ts")
    },
    output: {
      publicPath: '/',
      path: dist,
      filename:'[name].[contenthash].js'
    },
    devServer: {
      contentBase: dist,
      host: "0.0.0.0",
      port: 8000,
      // must reflect `output.publicPath` above, for SPA support during development  
      historyApiFallback: {
        index: '/'
      },
      noInfo: true,
      stats: "errors-only",
      overlay: {
        // Commented to prevent error:
        // `./crate/pkg/index_bg.js 382:14-53   Critical dependency: the request of a dependency is an expression`
        // warnings: true,
        errors: true
      },
    },
    plugins: [
      // Compilation progress 
      new WebpackBar(),
      // Clean `dist` folder before compilation
      new CleanWebpackPlugin(),
      // Extract CSS styles into a file.
      new MiniCssExtractPlugin({
        filename:'[name].[contenthash].css'
      }),
      // Add scripts, css, ... to html template.
      new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "static/index.hbs")
      }),
      // Compile Rust.
      new WasmPackPlugin({
        crateDirectory: __dirname
      }),

      // You can find files from folder `../static` on url `http://my-site.com/static/`.
      // And favicons in the root.
      new CopyWebpackPlugin([
        {
          from: "static",
          to: "static"
        },
        {
          from: "favicons",
          to: ""
        }
      ]),
    ],
    resolve: {
      extensions: [".ts", ".js", ".wasm"],
      alias: {
        crate: __dirname,
      }
    },
    module: {
      rules: [
        {
          test: /\.hbs$/,
          use: [
            {
              loader: "handlebars-loader",
              options: {
                rootRelative: './templates/'
              }
            }
          ]
        },
        {
          test: /\.(jpg|jpeg|png|woff|woff2|eot|ttf|svg)$/,
          use: [
            {
              loader: "file-loader",
              options: {
                // Don't copy files to `dist`, we do it through `CopyWebpackPlugin` (see above)
                // - we only want to resolve urls to these files.
                emitFile: false,
                name: "[path][name].[ext]"
              }
            }
          ]
        },
        {
          test: /\.ts$/,
          loader: "ts-loader?configFile=tsconfig.json"
        },
        {
          test: /\.css$/,
          use: [
            MiniCssExtractPlugin.loader,
            "css-loader",
            {
              loader: "postcss-loader",
              options: {
                config: {
                  // Path to postcss.config.js.
                  path: __dirname,
                  // Pass mode into `postcss.config.js` (see more info in that file).
                  ctx: { mode: argv.mode }
                }
              }
            }
          ]
        }
      ]
    }
  };
};
