use std::io::Cursor;

use crate::error::WasmImageError;
use crate::load::{load_image, SourceImage};
use crate::source_type::SourceType;
use image::ImageFormat;
pub use settings::Settings;

#[cfg(feature = "wasm")] 
use {
    js_sys::Uint8Array,
    wasm_bindgen::prelude::*,
};

pub mod settings;
pub(crate) mod svg;

fn write_image(
    img: &image::DynamicImage,
    file_type: Option<ImageFormat>,
) -> Result<Vec<u8>, WasmImageError> {
    let mut output: Vec<u8> = Vec::new();

    let target_type = file_type.unwrap_or(ImageFormat::Png);

    img.write_to(&mut Cursor::new(&mut output), target_type)?;

    Ok(output)
}

/// Image pre-processing, to ensure that the image can be converted to the target format.
fn process_image(
    img: &SourceImage,
    source_type: Option<ImageFormat>,
    target_type: Option<ImageFormat>,
    settings: Option<&Settings>,
) -> Result<image::DynamicImage, WasmImageError> {
    let img = img.rasterize(settings)?;

    let target_type = target_type.unwrap_or(ImageFormat::Png);

    let img = match source_type {
        Some(ImageFormat::Hdr) => image::DynamicImage::ImageRgba8(img.to_rgba8()),
        _ => img,
    };

    let processed = match target_type {
        ImageFormat::Jpeg
        | ImageFormat::Qoi
        | ImageFormat::Farbfeld
        | ImageFormat::Pnm
        | ImageFormat::Tga => image::DynamicImage::ImageRgb8(img.to_rgb8()),
        ImageFormat::Ico => img.resize(256, 256, image::imageops::FilterType::Lanczos3),
        ImageFormat::OpenExr => image::DynamicImage::ImageRgba32F(img.to_rgba32f()),
        ImageFormat::Hdr => image::DynamicImage::ImageRgb32F(img.to_rgb32f()),
        _ => img,
    };

    Ok(processed)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = convertImage)]
#[allow(clippy::needless_pass_by_value)]
/// Convert an image from one format to another.
/// # Arguments
/// * `file` - The image file to convert.
/// * `src_type` - The MIME type of the source image.
/// * `target_type` - The MIME type of the target image.
/// * `cb` - A callback function to report progress.
/// * `convert_settings` - Settings for the conversion.
/// # Errors
/// Returns an error if the conversion fails.
pub fn convert_image(
    file: &Uint8Array,
    src_type: &str,
    target_type: &str,
    cb: &js_sys::Function,
    convert_settings: Option<Settings>,
) -> Result<Uint8Array, JsValue> {
    let src_mime_type = SourceType::from_mime_type(src_type);

    let this = JsValue::NULL;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(10.0),
        &JsValue::from_str("Starting conversion"),
    );

    let file = file.to_vec();

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(35.0),
        &JsValue::from_str("Loading image"),
    );

    let img = load_image(&file, src_mime_type.as_ref())
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(50.0),
        &JsValue::from_str("Processing image"),
    );

    let img = process_image(
        &img,
        ImageFormat::from_mime_type(src_type),
        ImageFormat::from_mime_type(target_type),
        convert_settings.as_ref(),
    )
    .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(70.0),
        &JsValue::from_str("Converting image"),
    );

    let output = write_image(&img, ImageFormat::from_mime_type(target_type))
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(100.0),
        &JsValue::from_str("Conversion complete"),
    );

    Ok(Uint8Array::from(output.as_slice()))
}

#[cfg(not(feature = "wasm"))]
/// Convert an image from one format to another.
/// # Arguments
/// * `file` - The image file to convert.
/// * `src_type` - The MIME type of the source image.
/// * `target_type` - The MIME type of the target image.
/// * `convert_settings` - Settings for the conversion.
/// # Errors
/// Returns an error if the conversion fails.
/// # Example
/// ```rust
/// ```
pub fn convert_image(
    file: &[u8],
    src_type: &str,
    target_type: &str,
    convert_settings: &Option<Settings>,
) -> Result<Vec<u8>, WasmImageError> {
    let src_mime_type = SourceType::from_mime_type(src_type);

    let img = load_image(file, src_mime_type.as_ref())?;

    let img = process_image(
        &img,
        ImageFormat::from_mime_type(src_type),
        ImageFormat::from_mime_type(target_type),
        convert_settings.as_ref(),
    )?;

    write_image(&img, ImageFormat::from_mime_type(target_type))
}