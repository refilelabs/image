use std::io::Cursor;

use crate::error::WasmImageError;
use crate::load::{load_image, SourceImage};
use crate::source_type::SourceType;
use image::{imageops::FilterType, ImageFormat};

#[cfg(feature = "wasm")]
use {js_sys::Uint8Array, wasm_bindgen::prelude::*};

fn resize_and_write(
    img: &SourceImage,
    source_type: Option<ImageFormat>,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, WasmImageError> {
    let img = img.rasterize(None)?;

    let resized = img.resize_exact(width, height, FilterType::Lanczos3);

    let format = source_type.unwrap_or(ImageFormat::Png);

    let mut output: Vec<u8> = Vec::new();

    resized.write_to(&mut Cursor::new(&mut output), format)?;

    Ok(output)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = resizeImage)]
#[allow(clippy::needless_pass_by_value)]
/// Resize an image to exact dimensions, preserving the source format.
/// # Arguments
/// * `file` - The image file bytes.
/// * `src_type` - The MIME type of the source image.
/// * `width` - Target width in pixels.
/// * `height` - Target height in pixels.
/// * `cb` - A callback function to report progress.
/// # Errors
/// Returns an error if loading, resizing, or encoding fails.
pub fn resize_image(
    file: &Uint8Array,
    src_type: &str,
    width: u32,
    height: u32,
    cb: &js_sys::Function,
) -> Result<Uint8Array, JsValue> {
    let src_mime_type = SourceType::from_mime_type(src_type);

    crate::progress::report(cb, 10.0, "Starting resize");

    let file = file.to_vec();

    crate::progress::report(cb, 35.0, "Loading image");

    let img = load_image(&file, src_mime_type.as_ref())
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    crate::progress::report(cb, 60.0, "Resizing image");

    let output = resize_and_write(&img, ImageFormat::from_mime_type(src_type), width, height)
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    crate::progress::report(cb, 100.0, "Resize complete");

    Ok(Uint8Array::from(output.as_slice()))
}

#[cfg(not(feature = "wasm"))]
/// Resize an image to exact dimensions, preserving the source format.
/// # Arguments
/// * `file` - The image file bytes.
/// * `src_type` - The MIME type of the source image.
/// * `width` - Target width in pixels.
/// * `height` - Target height in pixels.
/// # Errors
/// Returns an error if loading, resizing, or encoding fails.
pub fn resize_image(
    file: &[u8],
    src_type: &str,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, WasmImageError> {
    let src_mime_type = SourceType::from_mime_type(src_type);
    let img = load_image(file, src_mime_type.as_ref())?;
    resize_and_write(&img, ImageFormat::from_mime_type(src_type), width, height)
}
