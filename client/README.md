# Client
Cross-platform browser client

## Requirements
- [Rust](https://www.rust-lang.org) 
 - Check: `$ rustc -V` => `rustc 1.43.1 (8d69840ab 2020-05-04)`
 - Install: https://www.rust-lang.org/tools/install
- [cargo-make](https://sagiegurari.github.io/cargo-make/)
 - Check: `$ cargo make -V` => `cargo-make 0.30.7`
 - Install: `$ cargo install cargo-make`
       
Platform-specific tools like `ssl` and `pkg-config`:
- Follow recommendations in build errors
- _Note_: Don't hesitate to write notes or a tutorial for your platform and create a PR .

## Development
1. Open a new terminal tab / window and run: `cargo make serve`
1. Open a second terminal tab and run: `cargo make watch`
1. If you see errors, try to fix them or write on our [chat](https://discord.gg/JHHcHp5) or [forum](https://seed.discourse.group/).
1. Modify files like `README.md` and `Cargo.toml` as you wish.

### Live Edit
1. Open [localhost:8000](http://localhost:8000) in a browser (I recommend Firefox and Chrome).
1. Modify source files (e.g. `/src/lib.rs` or `/index.html`).
1. Watch compilation in the terminal tab where you run `cargo make watch`.
1. You can watch dev-server responses in the tab where you run `cargo make serve`.
1. Refresh your browser and see changes.
1. Go to step 2.

### ECS
Du is built with the ECS model.

#### Stylehguide
- `Entity.rs`
- `component.rs`
- `System/` (a directory)

## Deployment
1. Run `cargo make verify` in your terminal to format and lint the code.
1. Run `cargo make build_release`.
1. Upload `index.html` and `pkg` into your server's public folder.
   - Don't forget to upload also configuration files for your hosting, see the [Netlify](https://www.netlify.com/) one below.

```toml
# netlify.toml
[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```
