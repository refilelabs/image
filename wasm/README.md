# @refilelabs/image

A WebAssembly-powered library for advanced image manipulation and format conversion. This package provides tools for loading image metadata, converting images, and retrieving raw pixel data—all in the browser or Node.js environment.

It is used under the hood at [re;file labs' image tools](https://refilelabs.com/image) to power the different image processing features.

## Installation

```bash
npm install @refilelabs/image
```

## Features

- Load image metadata
- Retrieve raw RGBA pixel data and image properties
- Convert images between different formats
- Supports custom conversion settings

## API Reference

### `loadMetadata(file: Uint8Array, src_type: string, cb: Function): Metadata`

Loads the metadata of an image file.

#### Parameters:
- `file` (`Uint8Array`): The image file to analyze.
- `src_type` (`string`): The MIME type of the source image (e.g., `image/png`, `image/jpeg`).
- `cb` (`Function`): A callback function to report progress.

#### Returns:
- `Metadata`: An object containing image metadata (e.g., width, height, other information).

---

### `getPixels(file: Uint8Array, src_type: string): ImageData`

Converts an image file to raw RGBA pixel data.

#### Parameters:
- `file` (`Uint8Array`): The image file to convert.
- `src_type` (`string`): The MIME type of the source image.

#### Returns:
- `ImageData`: An object containing raw pixel data and image properties (e.g., width, height, color depth).

---

### `convertImage(file: Uint8Array, src_type: string, target_type: string, cb: Function, convert_settings?: Settings): Uint8Array`

Converts an image from one format to another.

#### Parameters:
- `file` (`Uint8Array`): The image file to convert.
- `src_type` (`string`): The MIME type of the source image.
- `target_type` (`string`): The target MIME type (e.g., `image/webp`, `image/png`).
- `cb` (`Function`): A callback function to report progress.
- `convert_settings` (`Settings`, optional): Settings for the conversion (e.g., for SVG).

#### Returns:
- `Uint8Array`: The converted image data.

---

### Interfaces

#### `Metadata`

Represents metadata of an image.

- `width` (`number`): Image width.
- `height` (`number`): Image height.
- `other` (`Record<string, string> | null`): Additional metadata.

---

#### `ImageData`

Represents raw image data.

- `width` (`number`): Image width.
- `height` (`number`): Image height.
- `aspect_ratio` (`number`): Aspect ratio.
- `color_depth` (`number`): Color depth.
- `pixels` (`number[]`): Raw RGBA pixel values.

---

#### `SvgSettings`

Settings specific to SVG format.

- `width` (`number`): SVG width.
- `height` (`number`): SVG height.

---

#### `Settings`

Settings for conversion.

- `type`: `"svg"` for SVG settings.
- Includes all properties of `SvgSettings`.

---

## Usage Example

```javascript
import init, { loadMetadata, getPixels, convertImage } from '@refilelabs/image';

await init();

const file = new Uint8Array(/* ... */);
const srcType = 'image/png';
const targetType = 'image/webp';

const metadata = loadMetadata(file, srcType, (progress) => console.log(`Progress: ${progress}%`));
console.log('Metadata:', metadata);

const imageData = getPixels(file, srcType);
console.log('Image Data:', imageData);

const converted = convertImage(file, srcType, targetType, (progress) => console.log(`Progress: ${progress}%`));
console.log('Converted Image:', converted);
```

**Note:** When using the library in a Node.js environment, you need to initialize the wasm module as follows (see [issue](https://github.com/refilelabs/image/issues/6)):
```javascript
const wasmBuffer = fs.readFileSync('node_modules/@refilelabs/image/image_bg.wasm')

await init(wasmBuffer)
```

## License

MIT
