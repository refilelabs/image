# refilelabs/image

[![Crates.io](https://img.shields.io/crates/v/refilelabs-image.svg)](https://crates.io/crates/refilelabs-image)
[![Documentation](https://docs.rs/refilelabs-image/badge.svg)](https://docs.rs/refilelabs-image)
[![npm](https://npmx.dev/api/registry/badge/version/@refilelabs/image?label=npm)](https://npmx.dev/package/@refilelabs/image)
[![CI](https://github.com/refilelabs/image/actions/workflows/ci.yml/badge.svg)](https://github.com/refilelabs/image/actions/workflows/ci.yml)

The `refilelabs/image` repository provides core utilities for image manipulation within the [**re;file labs**](https://refilelabs.com) platform. All operations run securely in the browser, powered by WebAssembly.

## Features

- **Image Conversion**: Convert images between a wide range of formats (JPEG, PNG, WebP, AVIF, and more).
- **Image Resizing**: Resize images while preserving aspect ratio.
- **Image Compression**: Reduce file size without sacrificing quality.
- **Image Viewing**: View images in a responsive, interactive viewer.
- **Metadata Editing**: Read, edit, and strip metadata (EXIF, GPS) for supported formats.

## Getting Started

### Prerequisites

- [Bun](https://bun.sh) — package manager and runtime
- [Rust](https://rustup.rs) — for compiling WebAssembly
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) — for building WASM modules

### Installation

```bash
git clone https://github.com/refilelabs/image.git
cd image
bun install
```

`bun install` triggers `postinstall`, which runs `wasm:bundle` to compile the Rust crate into three WASM targets (`bundler`, `node`, `web`) under `wasm/pkg/`.

### Development

```bash
bun dev
```

## Usage

### As a Nuxt Layer

Include the image layer in your Nuxt project via the [extends feature](https://nuxt.com/docs/getting-started/layers#usage):

```ts
export default defineNuxtConfig({
  extends: ['github:refilelabs/image'],
})
```

### Programmatic Usage (WASM)

The [`@refilelabs/image`](https://www.npmjs.com/package/@refilelabs/image) package exports the WASM module. The correct build is selected automatically based on your environment:

| Environment | Build used |
|-------------|-----------|
| Bundler (Vite, Webpack, Rollup) | `bundler` — no `init()` needed |
| Node.js | `node` — uses `fs` for WASM loading |
| Raw browser (`<script type="module">`) | `web` — explicit `init()` required |

See [`wasm/README_JS.md`](wasm/README_JS.md) for the full JavaScript API reference.

## Project Structure

```
refilelabs/image
├── .playground/         # Dev playground for testing
├── wasm/                # Rust crate + WASM build output
│   ├── src/             # Rust source
│   ├── scripts/         # Build scripts (build.ts)
│   └── pkg/             # Generated WASM packages (bundler/, node/, web/)
├── components/          # Vue components
├── composables/         # Vue composables
└── workers/             # Web Workers for compute-intensive tasks
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

[**re;file labs**](https://refilelabs.com) — Your secure, private file utility suite.
