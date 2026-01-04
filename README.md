# dev-widgets

## Overview

Collection of useful conversions and widgets built as a pure Rust app on the [Dioxus framework](https://github.com/DioxusLabs/dioxus).

The following widgets are now stable:

- Number Base Converter
- Base64 Encoder/Decoder
- QR Code Generator
- Date/Timestamp Converter
- UUID Generator
- Hash Generator
- CIDR Decoder
- Color Picker

## Development Setup

If you haven't already, [install Rust](https://www.rust-lang.org/tools/install).

Follow the Dioxus instructions to [install platform-specific dependencies](https://dioxuslabs.com/docs/0.3/guide/en/getting_started/desktop.html#platform-specific-dependencies).

Install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

Clone this repository and enter its root directory.

## Desktop App

The primary target for this project is a universal desktop app. Currently, it has been manually validated on macOS and Windows. I plan to setup automated releases soon.

### Run app from command-line

Run `cargo run` to start the application. The first build should take a couple minutes as it fetches Bootstrap and all other Rust packages, subsequent builds should only take a few seconds.

If you would like to enable hot-reloading, you can do so by setting the `USE_HOT_RELOAD` flag in [main.rs](src/main.rs). This is only necessary for the desktop app, hot-reload is on by default for web development.

### Bundle app

You can bundle the app into an executable for your platform using the Dioxus CLI

```bash
dx bundle --platform desktop --release
```

## Web App

[![Build static site](https://github.com/esimkowitz/dev-widgets/actions/workflows/build-site.yml/badge.svg)](https://github.com/esimkowitz/dev-widgets/actions/workflows/build-site.yml)

Dev Widgets now works as a web app! You can find it hosted at <https://widgets.fyi>. It will stay up to date with the main branch using GitHub Actions.

### Run from command line - Dioxus CLI

You can run the web app locally using the [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli):

```bash
dx serve --platform web
```

This will automatically enable hot-reloading for any changes you make to the source code.

### Validate release buld

When packaging for release, I use the Dioxus CLI:

```bash
dx build --platform web --release
```

## Roadmap

This app is heavily inspired by [DevToys](https://github.com/veler/DevToys) and [DevToysMac](https://github.com/ObuchiYuki/DevToysMac) and my roadmap for widgets I plan to support will align with these projects.

Currently, I have only validated on macOS, and performed very crude validations on Windows, though I now have a fairly stable programming model so I plan to set up some automated testing for macOS, Windows, and Linux soon, as well as start publishing releases.

I also plan to publish this as a single-page application using dioxus-web and Github Pages.

I will be tracking new development in the [dev-widgets project board](https://github.com/users/esimkowitz/projects/2). New widgets will be organized under the "Widgets" area, and all other development will be tracked under the "Platform" area.
