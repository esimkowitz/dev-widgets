# dev-widgets

## Overview

Collection of useful conversions and widgets built as a pure Rust app on the [Dioxus framework](https://github.com/DioxusLabs/dioxus).

The following widgets are now stable:

* Number Base Converter
* Base64 Encoder/Decoder
* QR Code Generator
* Date/Timestamp Converter
* UUID Generator
* Hash Generator
* CIDR Decoder
* Color Picker

## Development Setup

If you haven't already, [install Rust](https://www.rust-lang.org/tools/install).

Follow the Dioxus instructions to [install platform-specific dependencies](https://dioxuslabs.com/docs/0.3/guide/en/getting_started/desktop.html#platform-specific-dependencies).

Clone this repository and enter its root directory.

## Desktop App

The primary target for this project is a universal desktop app. Currently, it has been manually validated on macOS and Windows. I plan to setup automated releases soon.

### Run app from command-line

Run `cargo run` to start the application. The first build should take a couple minutes as it fetches Bootstrap and all other Rust packages, subsequent builds should only take a few seconds.

If you would like to enable hot-reloading, you can do so by setting the `USE_HOT_RELOAD` flag in [main.rs](src/main.rs). This is only necessary for the desktop app, hot-reload is on by default for web development.

### Bundle app

You can bundle the app into an executable for your platform using [cargo-bundle](https://github.com/burtonageo/cargo-bundle). If you haven't already, run the following command to install cargo-bundle:

```bash
cargo install cargo-bundle
```

Once you have cargo-bundle installed, run the following command to package the application for your platform:

```bash
cargo bundle --release
```

## Web App

[![Build static site](https://github.com/esimkowitz/dev-widgets/actions/workflows/build-site.yml/badge.svg)](https://github.com/esimkowitz/dev-widgets/actions/workflows/build-site.yml)

Dev Widgets now works as a web app! You can find it hosted at <https://widgets.fyi>. It will stay up to date with the main branch using GitHub Actions.

### Run from command line - Dioxus CLI

You can run the web app locally using the [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli). Because this repo depends on unstable releases of Dioxus, you need to install Dioxus CLI via Git to make sure it is compatible:

```bash
cargo install dioxus-cli --git https://github.com/DioxusLabs/dioxus.git --rev b25501af48977817d9d0bb2534c94cff30317c8c
```

Once you have the CLI installed, you can launch the web app using the following command:

```bash
dioxus serve
```

This will automatically enable hot-reloading for any changes you make to the source code.

### Validate release buld - Trunk CLI

When packaging for release, I use Trunk as it is more-readily installable on the Github Actions agents. I found that installing the Dioxus CLI would compile the binary from scratch, which took too long and would hit out-of-memory errors unless I increased the swap file size.

You can install Trunk using the following command:

```bash
cargo install trunk --locked
```

You won't be able to run the app locally using Trunk, but you can validate that it builds correctly by running the following command:

```bash
trunk build --release
```

## Roadmap

This app is heavily inspired by [DevToys](https://github.com/veler/DevToys) and [DevToysMac](https://github.com/ObuchiYuki/DevToysMac) and my roadmap for widgets I plan to support will align with these projects.

Currently, I have only validated on macOS, and performed very crude validations on Windows, though I now have a fairly stable programming model so I plan to set up some automated testing for macOS, Windows, and Linux soon, as well as start publishing releases.

I also plan to publish this as a single-page application using dioxus-web and Github Pages.

I will be tracking new development in the [dev-widgets project board](https://github.com/users/esimkowitz/projects/2). New widgets will be organized under the "Widgets" area, and all other development will be tracked under the "Platform" area.
