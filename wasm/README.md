# refilelabs-image

[![Crates.io](https://img.shields.io/crates/v/refilelabs-image.svg)](https://crates.io/crates/refilelabs-image)
[![Documentation](https://docs.rs/refilelabs-image/badge.svg)](https://docs.rs/refilelabs-image)

A Rust library for advanced image manipulation and format conversion. This crate provides tools for loading image metadata, converting images, resizing images, and retrieving raw pixel data.

It is used under the hood at [re;file labs' image tools](https://refilelabs.com/image) to power the different image processing features.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
refilelabs-image = "0.1.0"  # Replace with actual version
```

## Features

- Load image metadata
- Retrieve raw RGBA pixel data and image properties
- Convert images between different formats
- Resize images to exact pixel dimensions
- Supports custom conversion settings

## Important Note

The main API functions are only available when the crate is not compiled with the 

wasm

 feature. These functions are designed for native Rust usage.

## API Reference

### `convert_image(file: &[u8], src_type: &str, target_type: &str, convert_settings: &Option<Settings>) -> Result<Vec<u8>, WasmImageError>`

Converts an image from one format to another.

#### Parameters:
- `file` (`&[u8]`): The image file to convert
- `src_type` (`&str`): The MIME type of the source image (e.g., `"image/png"`)
- `target_type` (`&str`): The target MIME type (e.g., `"image/webp"`)
- `convert_settings` (`&Option<Settings>`): Settings for the conversion

#### Returns:
- `Result<Vec<u8>, WasmImageError>`: The converted image data or an error

---

### `load_metadata(file: &[u8], src_type: &str) -> Result<Metadata, WasmImageError>`

Loads the metadata of an image file.

#### Parameters:
- `file` (`&[u8]`): The image file to analyze
- `src_type` (`&str`): The MIME type of the source image

#### Returns:
- `Result<Metadata, WasmImageError>`: An object containing image metadata or an error

---

### `get_pixels(file: &[u8], src_type: &str) -> Result<ImageData, WasmImageError>`

Converts an image file to raw RGBA pixel data.

#### Parameters:
- `file` (`&[u8]`): The image file to convert
- `src_type` (`&str`): The MIME type of the source image

#### Returns:
- `Result<ImageData, WasmImageError>`: An object containing raw pixel data and image properties or an error

---

### `resize_image(file: &[u8], src_type: &str, width: u32, height: u32) -> Result<Vec<u8>, WasmImageError>`

Resizes an image to exact pixel dimensions, preserving the source format.

#### Parameters:
- `file` (`&[u8]`): The image file to resize
- `src_type` (`&str`): The MIME type of the source image (e.g., `"image/png"`)
- `width` (`u32`): Target width in pixels
- `height` (`u32`): Target height in pixels

#### Returns:
- `Result<Vec<u8>, WasmImageError>`: The resized image data or an error

#### Notes:
- Uses the Lanczos3 filter for high-quality downscaling and upscaling
- Resizes to exact dimensions without preserving aspect ratio
- SVG input is rasterized before resizing and output as PNG

---

## Data Structures

### `Metadata`

Represents metadata of an image.

- `width` (`u32`): Image width
- `height` (`u32`): Image height
- `other` (`Option<HashMap<String, String>>`): Additional metadata

---

### `ImageData`

Represents raw image data.

- `width` (`u32`): Image width
- `height` (`u32`): Image height
- `aspect_ratio` (`f32`): Aspect ratio
- `color_depth` (`u8`): Color depth
- `pixels` (`Vec<u8>`): Raw RGBA pixel values

---

### `Settings`

Settings for conversion.

- Contains various settings depending on the image format

---

## Usage Example

```rust
use refilelabs_image::{convert_image, get_pixels, load_metadata, resize_image, Settings};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read image file
    let file = fs::read("input.png")?;
    let src_type = "image/png";
    let target_type = "image/webp";

    // Get metadata
    let metadata = load_metadata(&file, src_type)?;
    println!("Image dimensions: {}x{}", metadata.width, metadata.height);

    // Get pixel data
    let image_data = get_pixels(&file, src_type)?;
    println!("Image has {} pixels", image_data.pixels.len() / 4);

    // Convert image
    let converted = convert_image(&file, src_type, target_type, &None)?;
    fs::write("output.webp", converted)?;

    // Resize image to 800x600
    let resized = resize_image(&file, src_type, 800, 600)?;
    fs::write("output_resized.png", resized)?;

    Ok(())
}
```

## License

MIT
