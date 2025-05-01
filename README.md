# Probability (decision) tree solver

[![dependency status](https://deps.rs/repo/github/fledermaus101/probabilitytree_web/status.svg)](https://deps.rs/repo/github/fledermaus101/probabilitytree_web)
[![Build Status](https://github.com/fledermaus101/probabilitytree_web/workflows/CI/badge.svg)](https://github.com/fledermaus101/probabilitytree_web/actions?workflow=CI)

This is an app which solves a tree diagram (from probability thoery) given enough information using [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), [egui](https://github.com/emilk/egui/) and of course the Rust programming language with its support for web assembly.

### Running locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Running locally on the Web

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Build
1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
3. Copy it to a suitable webserver
