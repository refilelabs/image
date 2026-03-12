# @refilelabs/image
[![NPM Version](https://img.shields.io/npm/v/%40refilelabs%2Fimage)](https://www.npmjs.com/package/@refilelabs/image)

A WebAssembly-powered library for advanced image manipulation and format conversion. Powered by Rust and compiled to WebAssembly — runs in the browser, Node.js, and any modern bundler environment.

Used under the hood at [re;file labs](https://refilelabs.com/image) to power all image processing features.

## Installation

```bash
npm install @refilelabs/image
```

The package ships three builds and automatically selects the right one via the `exports` field in `package.json`:

| Environment | Build used |
|---|---|
| Vite / Webpack / Rollup (bundler) | `bundler` — bundler handles WASM loading, no `init()` needed |
| Node.js | `node` — uses `fs` to load the WASM binary, no `init()` needed |
| Raw browser (`<script type="module">`) | `web` — explicit `init()` required |

## Setup

### Vite / Nuxt

Vite requires `vite-plugin-wasm` to handle `.wasm` imports:

```bash
npm install -D vite-plugin-wasm
```

```js
// vite.config.js
import wasm from 'vite-plugin-wasm'

export default {
  plugins: [wasm()],
}
```

For Nuxt, add it to `nuxt.config.ts`:

```ts
import wasm from 'vite-plugin-wasm'

export default defineNuxtConfig({
  vite: {
    plugins: [wasm()],
  },
})
```

### Webpack 5

No additional config needed — Webpack 5 handles `.wasm` imports natively.

### Raw browser (no bundler)

No plugin needed. Call `init()` explicitly before using any functions.

## Usage

### Bundler (Vite, Webpack, Rollup)

```javascript
import { convertImage, loadMetadata, resizeImage, saveMetadata } from '@refilelabs/image'

const file = new Uint8Array(/* ... */)
const cb = ({ progress, message }) => console.log(`${progress}% — ${message}`)

const metadata = loadMetadata(file, 'image/jpeg', cb)
console.log(metadata.width, metadata.height, metadata.gps)
```

### Node.js

```javascript
import { convertImage } from '@refilelabs/image'

const file = new Uint8Array(/* ... */)
const converted = convertImage(file, 'image/png', 'image/webp', ({ progress }) => {}, null)
```

### Raw browser (no bundler)

```javascript
import init, { resizeImage } from '@refilelabs/image/web'

await init() // must call init() explicitly in web target

const resized = resizeImage(file, 'image/png', 800, 600, ({ progress, message }) => {})
```

---

## API Reference

### `loadMetadata(file, src_type, cb): Metadata`

Extracts dimensions and EXIF metadata from an image file.

**Parameters:**
- `file` (`Uint8Array`): The image file bytes.
- `src_type` (`string`): MIME type of the source image (e.g. `"image/jpeg"`).
- `cb` (`(update: { progress: number, message: string }) => void`): Progress callback.

**Returns:** `Metadata`

---

### `saveMetadata(file, src_type, changes, stripAll, stripGps, cb): Uint8Array`

Saves metadata changes to an image file. Supports JPEG (in-place byte edit, no re-encode), PNG, and WebP (re-encode with EXIF embedded via `set_exif_metadata`).

**Parameters:**
- `file` (`Uint8Array`): The image file bytes.
- `src_type` (`string`): MIME type of the source image.
- `changes` (`MetadataChange[]`): Array of tag changes to apply. Each item: `{ tag: string, value: string | null }`. Pass `null` value to remove a tag.
- `stripAll` (`boolean`): If `true`, re-encodes the image stripping all metadata. Ignores `changes`.
- `stripGps` (`boolean`): If `true`, removes all GPS tags after applying `changes`.
- `cb` (`(update: { progress: number, message: string }) => void`): Progress callback.

**Returns:** `Uint8Array` — the modified image bytes.

---

### `convertImage(file, src_type, target_type, cb, convert_settings?): Uint8Array`

Converts an image from one format to another.

**Parameters:**
- `file` (`Uint8Array`): The image file bytes.
- `src_type` (`string`): MIME type of the source image.
- `target_type` (`string`): Target MIME type (e.g. `"image/webp"`).
- `cb` (`(update: { progress: number, message: string }) => void`): Progress callback.
- `convert_settings` (`Settings | null`, optional): Format-specific settings (e.g. SVG rasterization size).

**Returns:** `Uint8Array` — the converted image bytes.

---

### `resizeImage(file, src_type, width, height, cb): Uint8Array`

Resizes an image to exact pixel dimensions using the Lanczos3 filter.

**Parameters:**
- `file` (`Uint8Array`): The image file bytes.
- `src_type` (`string`): MIME type of the source image.
- `width` (`number`): Target width in pixels.
- `height` (`number`): Target height in pixels.
- `cb` (`(update: { progress: number, message: string }) => void`): Progress callback.

**Returns:** `Uint8Array` — the resized image bytes, in the original format.

**Notes:**
- Resizes to exact dimensions without preserving aspect ratio.
- SVG input is rasterized before resizing; output is PNG.

---

### `getPixels(file, src_type): ImageData`

Decodes an image to raw RGBA pixel data.

**Parameters:**
- `file` (`Uint8Array`): The image file bytes.
- `src_type` (`string`): MIME type of the source image.

**Returns:** `ImageData`

---

## Interfaces

### `Metadata`

- `width` (`number`): Image width in pixels.
- `height` (`number`): Image height in pixels.
- `other` (`Record<string, string> | null`): Non-GPS EXIF fields, keyed by tag description.
- `gps` (`Record<string, string> | null`): GPS EXIF fields, keyed by tag description (e.g. `"Latitude"`, `"Longitude"`). `null` if no GPS data is present.
- `errors` (`string[] | null`): Non-fatal EXIF parse errors, if any.

### `MetadataChange`

- `tag` (`string`): EXIF tag description (e.g. `"Image Description"`, `"Artist"`).
- `value` (`string | null`): New value, or `null` to remove the tag.

### `ImageData`

- `width` (`number`): Image width.
- `height` (`number`): Image height.
- `aspect_ratio` (`number`): Width / height.
- `color_depth` (`number`): Bits per channel.
- `pixels` (`number[]`): Raw RGBA pixel values.

### `SvgSettings`

- `width` (`number`): Rasterization width.
- `height` (`number`): Rasterization height.

### `Settings`

- `type`: `"svg"`
- Includes all `SvgSettings` properties.

---

## License

MIT
