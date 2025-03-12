# refilelabs/image

[![Crates.io](https://img.shields.io/crates/v/refilelabs-image.svg)](https://crates.io/crates/refilelabs-image)
[![Documentation](https://docs.rs/refilelabs-image/badge.svg)](https://docs.rs/refilelabs-image)
[![NPM Version](https://img.shields.io/npm/v/%40refilelabs%2Fimage)](https://www.npmjs.com/package/@refilelabs/image)

The `refilelabs/image` repository provides core utilities for image manipulation within the [**re;file labs**](https://refilelabs.com) platform. It offers robust, high-performance image processing features powered by WebAssembly (WASM) for secure, browser-based operations.

## Features

- **Image Conversion**: Convert images between various formats (e.g., JPEG, PNG, WebP).
- **Image Editing**: Resize, crop, rotate, and flip images with ease. (Coming soon)
- **Image Compression**: Compress images to reduce file size while maintaining quality.
- **Image Viewing**: View images in a responsive, interactive viewer.
- **Metadata Editing**: View and edit metadata for supported image formats.
- **WebAssembly-Powered**: All operations run securely in the browser, leveraging WASM for speed and privacy.

## Getting Started

### Prerequisites

- **bun**: The package manager and runtime used within the **re;file labs** ecosystem.
- **Rust**: For compiling WebAssembly components.
- **wasm-pack**: For building and packaging WebAssembly modules.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/refilelabs/image.git
   cd image
   ```

2. Install dependencies (also builds WebAssembly modules):

   **Using Bun**:
   ```bash
   bun i
   ```

   **Using npm**:
   ```bash
   npm install
   ```

   **Using Yarn**:
   ```bash
   yarn install
   ```

   **Using pnpm**:
   ```bash
   pnpm install
   ```

1. Start the development server:

   **Using Bun**:
   ```bash
   bun dev
   ```

   **Using npm**:
   ```bash
   npm run dev
   ```

   **Using Yarn**:
   ```bash
   yarn dev
   ```

   **Using pnpm**:
   ```bash
   pnpm dev
   ```

## Usage

### Components

If you want to use the image utility components in your own project, either include this nuxt layer using the [extends feature](https://nuxt.com/docs/getting-started/layers#usage):
```ts
export default defineNuxtConfig({
  extends: [
   'github:refilelabs/image',
  ]
})
```

### Programmatic Usage

In case you want to use the actual wasm modules in your project, you can import them directly from the [`@refilelabs/image`](https://www.npmjs.com/package/@refilelabs/image/v/next) package.

## Project Structure

```
refilelabs/image
├── .playground/         # Temporary playground for testing
├── wasm/                # WebAssembly modules
├── components/          # Image processing components
├── composables/         # Shared Vue composables
├── workers/             # Web Workers for offloading compute-intensive tasks
└── tests/               # Unit tests
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

[**re;file labs**](https://refilelabs.com) — Your secure, private file utility suite.
