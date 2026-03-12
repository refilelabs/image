# refilelabs-image

[![Crates.io](https://img.shields.io/crates/v/refilelabs-image.svg)](https://crates.io/crates/refilelabs-image)
[![Documentation](https://docs.rs/refilelabs-image/badge.svg)](https://docs.rs/refilelabs-image)

A Rust library for advanced image manipulation and format conversion. Provides tools for loading image metadata, converting images, resizing images, and retrieving raw pixel data.

Used under the hood at [re;file labs](https://refilelabs.com/image) to power all image processing features.

## Installation

```toml
[dependencies]
refilelabs-image = "0.2.5"
```

## Features

- Load image metadata (dimensions, EXIF, GPS)
- Retrieve raw RGBA pixel data
- Convert images between formats
- Resize images to exact pixel dimensions
- Custom conversion settings (e.g. SVG rasterization size)

> **Note:** The native Rust API (`#[cfg(not(feature = "wasm"))]`) excludes `save_metadata`, which requires wasm-bindgen types. For JavaScript/WASM usage see the [npm package](https://www.npmjs.com/package/@refilelabs/image).

## API Reference

### `convert_image`

```rust
pub fn convert_image(
    file: &[u8],
    src_type: &str,
    target_type: &str,
    convert_settings: &Option<Settings>,
) -> Result<Vec<u8>, WasmImageError>
```

Converts an image from one format to another.

---

### `load_metadata`

```rust
pub fn load_metadata(file: &[u8], src_type: &str) -> Result<Metadata, WasmImageError>
```

Extracts dimensions and EXIF metadata from an image file.

---

### `resize_image`

```rust
pub fn resize_image(
    file: &[u8],
    src_type: &str,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, WasmImageError>
```

Resizes an image to exact pixel dimensions using the Lanczos3 filter. Preserves the source format. SVG input is rasterized to PNG.

---

### `get_pixels`

```rust
pub fn get_pixels(file: &[u8], src_type: &str) -> Result<ImageData, WasmImageError>
```

Decodes an image to raw RGBA pixel data.

---

## Data Structures

### `Metadata`

```rust
pub struct Metadata {
    pub width: u32,
    pub height: u32,
    pub other: Option<HashMap<String, String>>, // Non-GPS EXIF fields
    pub gps: Option<HashMap<String, String>>,   // GPS EXIF fields (None if not present)
    pub errors: Option<Vec<String>>,            // Non-fatal EXIF parse errors
}
```

### `ImageData`

```rust
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub color_depth: u8,
    pub pixels: Vec<u8>, // Raw RGBA
}
```

### `Settings`

Format-specific conversion settings. Currently supports SVG rasterization size.

---

## Usage Example

```rust
use refilelabs_image::{convert_image, get_pixels, load_metadata, resize_image};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read("input.png")?;
    let src_type = "image/png";

    let metadata = load_metadata(&file, src_type)?;
    println!("{}x{}", metadata.width, metadata.height);
    if let Some(gps) = metadata.gps {
        println!("GPS: {:?}", gps);
    }

    let converted = convert_image(&file, src_type, "image/webp", &None)?;
    fs::write("output.webp", converted)?;

    let resized = resize_image(&file, src_type, 800, 600)?;
    fs::write("output_resized.png", resized)?;

    let pixels = get_pixels(&file, src_type)?;
    println!("{} RGBA pixels", pixels.pixels.len() / 4);

    Ok(())
}
```

## License

MIT
