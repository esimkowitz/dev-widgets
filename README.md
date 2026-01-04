# dev-widgets

## Overview

Collection of useful developer utilities built as a pure Rust app on the [Dioxus framework](https://github.com/DioxusLabs/dioxus).

### Available Widgets

#### Generators

- UUID/GUID Generator - Generate unique identifiers
- QR Code Generator - Generate QR codes from text
- Hash Generator - Generate cryptographic hashes of strings
- Password Generator - Generate secure, customizable passwords
- Lorem Ipsum Generator - Generate placeholder text

#### Converters

- Number Base Converter - Convert between binary, octal, decimal, and hexadecimal
- JSON <> YAML Converter - Convert between JSON and YAML formats
- Date Converter - Convert dates between formats

#### Encoders/Decoders

- Base64 Encoder/Decoder - Encode and decode base64 strings
- CIDR Decoder - Decode CIDR notation to IP address range

#### Media

- Color Picker - Pick a color and get output in different formats

## Development Setup

1. Install prerequisites:
   - [Rust](https://www.rust-lang.org/tools/install)
   - [Node.js](https://nodejs.org/) (for Tailwind CSS compilation)
   - [Platform-specific dependencies](https://dioxuslabs.com/learn/0.7/getting_started/#platform-specific-dependencies) for Dioxus

2. Install the Dioxus CLI:

   See the [Dioxus Getting Started guide](https://dioxuslabs.com/learn/0.7/getting_started/#install-the-cli) for the latest instructions. Generally, this can be done via `cargo-binstall`:

   ```bash
   cargo binstall dioxus-cli --force
   ```

   If you don't have `cargo-binstall`, you can install it via:

   ```bash
   cargo install cargo-binstall
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
