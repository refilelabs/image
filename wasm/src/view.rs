use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::{load::load_image, source_type::SourceType};

#[derive(tsify::Tsify, serde::Serialize, serde::Deserialize, Default)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub color_depth: u16,
    pub pixels: Vec<u8>,
}

#[wasm_bindgen(js_name = getPixels)]
/// Convert an image file to a raw RGBA image.
/// # Arguments
/// * `file` - The image file to convert.
/// * `src_type` - The MIME type of the source image.
/// # Returns
/// The raw RGBA image data.
/// # Errors
/// Returns an error if the image could not be loaded or rasterized.
pub fn get_pixels(file: &Uint8Array, src_type: &str) -> Result<ImageData, JsValue> {
    let src_mime_type = SourceType::from_mime_type(src_type);

    let file = file.to_vec();

    let img = load_image(&file, &src_mime_type)
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?
        .rasterize(&None)
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let pixels = img.to_rgba8().into_vec();

    let color_depth = match src_mime_type {
        Some(SourceType::Svg) => 32,
        _ => img.color().bits_per_pixel(),
    };

    Ok(ImageData {
        width: img.width(),
        height: img.height(),
        aspect_ratio: img.width() as f32 / img.height() as f32,
        color_depth,
        pixels,
    })
}
