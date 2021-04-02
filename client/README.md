# cornish-client 
## Seed Quickstart with Webpack
[**LIVE DEMO:** quickstart-webpack.seed-rs.org](https://quickstart-webpack.seed-rs.org)

## Getting Started
### Prerequisites
- [Yarn](https://yarnpkg.com/lang/en/docs/install) - run `$ yarn -v` in terminal. 
  It should output something like `1.22.4`
- [Node.js](https://nodejs.org) - `$ node -v` => `v12.13.0`
- [Rust](https://www.rust-lang.org/tools/install) - `$ rustc -V` => 
  `rustc 1.43.1 (8d69840ab 2020-05-04)`
- Rust target `wasm` - `$ rustup target list` => 
 `.. wasm32-unknown-unknown (installed) ..`
    - Install: `$ rustup target add wasm32-unknown-unknown`
       
####  Platform-specific tools like `ssl` and `pkg-config`:
- Follow recommendations in build errors (during the next chapter).
- _Note_: Don't hesitate to write a tutorial and create PR or write a 
  Wiki page for your platform.

#### These tools are required by some commands:
[wasm-pack](https://rustwasm.github.io/wasm-pack/)
- Check: `$ wasm-pack -V` => `wasm-pack 0.9.1`
- Install: `$ cargo install wasm-pack`

[cargo-make](https://sagiegurari.github.io/cargo-make/)
- Check: `$ cargo make -V` => `cargo-make 0.30.7`
- Install: `$ cargo install cargo-make`

[nightly rustfmt](https://github.com/rust-lang/rustfmt#on-the-nightly-toolchain)
- Check: `$ cargo +nightly fmt -- -V` => `rustfmt 1.4.14-nightly (a5cb5d26 2020-04-14)`
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
1. **Always run `$ cargo make verify`** before push to make sure CI pipeline will 
   accept your code.
    - It'll format your code, lint it and start headless tests in Firefox.
    - You can change its behaviour in `Makefile.tom` - task `verify` 
      (similar task `verify_only` is used in CI).

#### Travis CI
<details>
<summary>Content (this guide and related config file are not maintained)</summary>

1. Sync your [TravisCI](https://travis-ci.org/) account with your GitHub one.
1. Find repository with your app in [your list](https://travis-ci.org/account/repositories) and click on `Settings`.
1. Add _Environment Variable_ `NETLIFY_SITE_ID` and set it's value to **Site id**.
1. Add _Environment Variable_ `NETLIFY_ACCESS_TOKEN` and set it's value to **Access token**.
1. Switch to tab `Current` and click `Activate repository`.
1. _[Optional]_ Add badge to project's `/README.md` (Repository detail > Click on 
   badge next to the rep. name > `IMAGE URL` change to `MARKDOWN`)
1. _[Optional]_ Modify `/.travis.yml`.
    - Replace `yarn build:prerender` with `yarn build:release` if you don't want to prerender pages.
    - _Tip:_ If jobs don't want to start after push, check Repository 
      detail > `More options` > `Requests`.
   
</details>

## Usage
### Main application entrypoint: `src/lib.rs`
1. Change `TITLE_SUFFIX` value.
1. Delete `MAIL_TO_KAVIK` and `MAIL_TO_HELLWEB`.
1. Write a new body for function `view`.

### Favicons
1. Delete or replace files in `/favicons`.
1. Open `static/templates/favicons.hbs` in your IDE.
1. Delete content or update it.
   - _Note_: Templates are written in [Handlebars](https://handlebarsjs.com/).

### Loading page
1. Open `static/templates/loading_page.hbs` in your IDE.
1. Delete content or update it.

### Social media & basic settings
1. Open `static/templates/social_media.hbs` in your IDE.
1. Delete content or update it.

### HTML Template
1. Open `static/index.hbs` in your IDE.
1. Update content.

### Fonts
1. Delete or replace files and directories in `static/fonts`.
1. Open `/css/fonts.css` in your IDE.
1. Delete content or update it.

### Images & other files
1. Delete or replace files in `static/images`.
1. Delete `static/Martin_Kavik_resume.pdf`.

### TailwindCSS
1. Open `tailwind.config.js` in your IDE.
1. Update content or replace it with the default one:

```js
module.exports = {
  theme: {},
  variants: {},
  plugins: []
};
```

### Custom CSS
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

