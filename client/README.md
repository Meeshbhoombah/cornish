# cornish-client
The cross-platform browser client for Architect x Cornish.

## Getting Started
Familiarize yourself with the [Architecture here](/ARCHITECTURE.md).

You'll find:
- An overview of the file system
- A top-down prespective of the `cornish-client` 

### Prerequisites
- [Yarn](https://yarnpkg.com/lang/en/docs/install) - run `$ yarn -v` in terminal. 
  It should output something like `1.22.4`
- [Node.js](https://nodejs.org) - `$ node -v` => `v12.13.0`
- [Rust](https://www.rust-lang.org/tools/install) - `$ rustc -V` => 
  `rustc 1.43.1 (8d69840ab 2020-05-04)`
- Rust target `wasm` - `$ rustup target list` => 
 `.. wasm32-unknown-unknown (installed) ..`
    - Install: `$ rustup target add wasm32-unknown-unknown`
       
Platform-specific tools like `ssl` and `pkg-config`:
- Follow recommendations in build errors (during the next chapter).
- _Note_: Don't hesitate to write a tutorial and create PR or write a 
  Wiki page for your platform.

These tools are required by some commands:
[wasm-pack](https://rustwasm.github.io/wasm-pack/)
- Check: `$ wasm-pack -V` => `wasm-pack 0.9.1`
- Install: `$ cargo install wasm-pack`

[cargo-make](https://sagiegurari.github.io/cargo-make/)
- Check: `$ cargo make -V` => `cargo-make 0.30.7`
- Install: `$ cargo install cargo-make`

[nightly rustfmt](https://github.com/rust-lang/rustfmt#on-the-nightly-toolchain)
- Check: `$ cargo +nightly fmt -- -V` => `rustfmt 1.4.14-nightly 
  (a5cb5d26 2020-04-14)`
- Install:
    1. `$ rustup toolchain install nightly`
    2. `$ rustup component add rustfmt --toolchain nightly`

### Installation
1. Install Webpack and other dependencies - `$ yarn`
1. Try to start dev-server - `$ yarn start` - and then open [localhost:8000](http://localhost:8000) 
   in a browser.
1. Stop server (try `Ctrl+c`).
1. Try to lint your project - `$ cargo make verify_only` - you shouldn't see 
   any errors.

### Development

### Testing
1. Run `$ cargo make test_h firefox` for headless testing in Firefox.
   - There are more similar commands - see `Makefile.toml`
   - _Note_: There is only one test in this project (`tests/test.rs`), see [seed-rs-realworld](https://github.com/seed-rs/seed-rs-realworld) for more examples.
1. If you want to test prerendered website:
   1. `$ yarn build:prerender`
   1. `$ yarn serve:dist`
   1. Open [localhost:8000](http://localhost:8000) in a browser.
   1. _Tip_: Always test it also in production environment because e.g. routing 
      is a little bit different among servers.
1. **Always run `$ cargo make verify`** before push to make sure CI pipeline 
   will accept your code.
    - It'll format your code, lint it and start headless tests in Firefox.
    - You can change its behaviour in `Makefile.tom` - task `verify` 
      (similar task `verify_only` is used in CI).

### Production
Read more about building for [Production here](/PRODUCTION.md).
om CSS
1. Open `/css/custom.css` in your IDE.
1. Delete content or update it.

## Deployment
How to format, lint and test your project.

And how to setup Github Actions with deploy into Netlify.

### Formatter & Linter
1. Format: `$ cargo make fmt` (it overwrites files) or only 
   `$ cargo make fmt_check`
1. You can modify format settings in:
   - `rustfmt.toml`
   - `Makefile.toml` - tasks `fmt` and `fmt_check`
1. Lint: `$ cargo make clippy`
1. You can modify linter settings in:
   - `Makefile.toml` - task `clippy`

### Build
1. Run `yarn build:realease`.
1. Check if build works by running `yarn serve:dist` and view in a browser.
1. Deploy the contents of the `dist` directory to a web server.

