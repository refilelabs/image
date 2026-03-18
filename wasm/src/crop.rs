use std::io::Cursor;

use crate::error::WasmImageError;
use crate::load::{load_image, SourceImage};
use crate::source_type::SourceType;
use image::ImageFormat;

#[cfg(feature = "wasm")]
use {js_sys::Uint8Array, wasm_bindgen::prelude::*};

fn crop_and_write(
    img: &SourceImage,
    source_type: Option<ImageFormat>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, WasmImageError> {
    if width == 0 || height == 0 {
        return Err(WasmImageError::EncodingError(
            "Width and height must be greater than 0".to_string(),
        ));
    }
    let img = img.rasterize(None)?;

    if x + width > img.width() || y + height > img.height() {
        return Err(WasmImageError::EncodingError(
            "Crop region exceeds image bounds".to_string(),
        ));
    }

    let cropped = img.crop_imm(x, y, width, height);

    let format = source_type.unwrap_or(ImageFormat::Png);

    let mut output: Vec<u8> = Vec::new();

    cropped.write_to(&mut Cursor::new(&mut output), format)?;

    Ok(output)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = cropImage)]
#[allow(clippy::needless_pass_by_value)]
/// Crop an image to the specified region, preserving the source format.
/// # Arguments
/// * `file` - The image file bytes.
/// * `src_type` - The MIME type of the source image.
/// * `x` - Left offset of the crop region in pixels.
/// * `y` - Top offset of the crop region in pixels.
/// * `width` - Width of the crop region in pixels.
/// * `height` - Height of the crop region in pixels.
/// * `cb` - A callback function to report progress.
/// # Errors
/// Returns an error if loading, cropping, or encoding fails.
pub fn crop_image(
    file: &Uint8Array,
    src_type: &str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    cb: &js_sys::Function,
) -> Result<Uint8Array, JsValue> {
    let src_mime_type = SourceType::from_mime_type(src_type);

    crate::progress::report(cb, 10.0, "Starting crop");

    let file = file.to_vec();

    crate::progress::report(cb, 35.0, "Loading image");

    let img = load_image(&file, src_mime_type.as_ref())
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    crate::progress::report(cb, 60.0, "Cropping image");

    let output = crop_and_write(&img, ImageFormat::from_mime_type(src_type), x, y, width, height)
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    crate::progress::report(cb, 100.0, "Crop complete");

    Ok(Uint8Array::from(output.as_slice()))
}

#[cfg(not(feature = "wasm"))]
/// Crop an image to the specified region, preserving the source format.
/// # Arguments
/// * `file` - The image file bytes.
/// * `src_type` - The MIME type of the source image.
/// * `x` - Left offset of the crop region in pixels.
/// * `y` - Top offset of the crop region in pixels.
/// * `width` - Width of the crop region in pixels.
/// * `height` - Height of the crop region in pixels.
/// # Errors
/// Returns an error if loading, cropping, or encoding fails.
pub fn crop_image(
    file: &[u8],
    src_type: &str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, WasmImageError> {
    let src_mime_type = SourceType::from_mime_type(src_type);
    let img = load_image(file, src_mime_type.as_ref())?;
    crop_and_write(&img, ImageFormat::from_mime_type(src_type), x, y, width, height)
}
