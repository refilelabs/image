use std::io::Cursor;

use crate::error::WasmImageError;
use crate::load::{load_image, SourceImage};
use crate::source_type::SourceType;
use image::ImageFormat;

#[cfg(feature = "wasm")]
use {js_sys::Uint8Array, wasm_bindgen::prelude::*};

fn rotate_and_write(
    img: &SourceImage,
    source_type: Option<ImageFormat>,
    degrees: u32,
    flip_h: bool,
    flip_v: bool,
) -> Result<Vec<u8>, WasmImageError> {
    let mut img = img.rasterize(None)?;

    if flip_h {
        img = img.fliph();
    }

    if flip_v {
        img = img.flipv();
    }

    img = match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img,
    };

    let format = source_type.unwrap_or(ImageFormat::Png);

    let mut output: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut output), format)?;

    Ok(output)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = rotateImage)]
#[allow(clippy::needless_pass_by_value)]
/// Rotate and/or flip an image, preserving the source format.
/// # Arguments
/// * `file` - The image file bytes.
/// * `src_type` - The MIME type of the source image.
/// * `degrees` - Rotation in degrees: 0, 90, 180, or 270.
/// * `flip_h` - Whether to flip the image horizontally before rotating.
/// * `flip_v` - Whether to flip the image vertically before rotating.
/// * `cb` - A callback function to report progress.
/// # Errors
/// Returns an error if loading, transforming, or encoding fails.
pub fn rotate_image(
    file: &Uint8Array,
    src_type: &str,
    degrees: u32,
    flip_h: bool,
    flip_v: bool,
    cb: &js_sys::Function,
) -> Result<Uint8Array, JsValue> {
    let src_mime_type = SourceType::from_mime_type(src_type);

    crate::progress::report(cb, 10.0, "Starting");

    let file = file.to_vec();

    if degrees == 0 && !flip_h && !flip_v {
        crate::progress::report(cb, 100.0, "Done");
        return Ok(Uint8Array::from(file.as_slice()));
    }

    crate::progress::report(cb, 35.0, "Loading image");

    let img = load_image(&file, src_mime_type.as_ref())
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    crate::progress::report(cb, 60.0, "Transforming image");

    let output = rotate_and_write(&img, ImageFormat::from_mime_type(src_type), degrees, flip_h, flip_v)
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    crate::progress::report(cb, 90.0, "Encoding");
    crate::progress::report(cb, 100.0, "Done");

    Ok(Uint8Array::from(output.as_slice()))
}

#[cfg(not(feature = "wasm"))]
/// Rotate and/or flip an image, preserving the source format.
/// # Arguments
/// * `file` - The image file bytes.
/// * `src_type` - The MIME type of the source image.
/// * `degrees` - Rotation in degrees: 0, 90, 180, or 270.
/// * `flip_h` - Whether to flip the image horizontally before rotating.
/// * `flip_v` - Whether to flip the image vertically before rotating.
/// # Errors
/// Returns an error if loading, transforming, or encoding fails.
pub fn rotate_image(
    file: &[u8],
    src_type: &str,
    degrees: u32,
    flip_h: bool,
    flip_v: bool,
) -> Result<Vec<u8>, WasmImageError> {
    if degrees == 0 && !flip_h && !flip_v {
        return Ok(file.to_vec());
    }
    let src_mime_type = SourceType::from_mime_type(src_type);
    let img = load_image(file, src_mime_type.as_ref())?;
    rotate_and_write(&img, ImageFormat::from_mime_type(src_type), degrees, flip_h, flip_v)
}
