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

1. Install prerequisites:
   - [Rust](https://www.rust-lang.org/tools/install)
   - [Node.js](https://nodejs.org/) (for Tailwind CSS compilation)
   - [Platform-specific dependencies](https://dioxuslabs.com/learn/0.7/getting_started/#platform-specific-dependencies) for Dioxus

2. Install the Dioxus CLI:

   ```bash
   cargo install dioxus-cli
   ```

3. Clone this repository and enter its root directory.

4. Install npm dependencies:

   ```bash
   npm install
   ```

## Web App

[![Build static site](https://github.com/esimkowitz/dev-widgets/actions/workflows/build-site.yml/badge.svg)](https://github.com/esimkowitz/dev-widgets/actions/workflows/build-site.yml)

Dev Widgets is hosted at <https://widgets.fyi>, automatically deployed from the main branch via GitHub Actions.

### Run locally

```bash
npm run serve:web
```

This compiles TailwindCSS with a file watcher and then starts the Dioxus dev server and runs the web app with hot-reloading enabled.

### Build for release

This compiles the TailwindCSS and builds the web app as a single-page app for release to static hosting like GitHub Pages.

```bash
npm run build:web
```

## Desktop App

Dev Widgets also runs as a native desktop app via Tauri. Validated on macOS and Windows.

### Run desktop app locally

```bash
npm run serve:desktop
```

This compiles TailwindCSS with a file watcher and then starts the Dioxus dev server and runs the desktop app with hot-reloading enabled.

### Bundle app

This compiles the TailwindCSS and builds the desktop app for release.

```bash
npm run build:desktop
```

## Roadmap

This app is heavily inspired by [DevToys](https://github.com/veler/DevToys) and [DevToysMac](https://github.com/ObuchiYuki/DevToysMac) and my roadmap for widgets I plan to support will align with these projects.

I will be tracking new development in the [dev-widgets project board](https://github.com/users/esimkowitz/projects/2). New widgets will be organized under the "Widgets" area, and all other development will be tracked under the "Platform" area.
