use std::collections::HashMap;

use image::{GenericImageView, ImageDecoder, ImageFormat};
use js_sys::Uint8Array;
use resvg::usvg::Options;
use wasm_bindgen::prelude::*;

use crate::{
    error::WasmImageError,
    load::{load_raw_image, RawSourceImage},
    source_type::SourceType,
};

#[derive(tsify::Tsify, serde::Deserialize, serde::Serialize, Default)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct Metadata {
    pub width: u32,
    pub height: u32,
    pub other: Option<HashMap<String, String>>,
}

impl TryFrom<RawSourceImage<'_>> for Metadata {
    type Error = WasmImageError;

    #[allow(clippy::too_many_lines)]
    fn try_from(img: RawSourceImage) -> Result<Self, Self::Error> {
        match img {
            RawSourceImage::Raster(img, format) => {
                // See https://docs.rs/image/latest/image/trait.ImageDecoder.html#implementors
                let decoder: Option<Box<dyn ImageDecoder>> = match format {
                    ImageFormat::Bmp => Some(Box::new(
                        image::codecs::bmp::BmpDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Gif => Some(Box::new(
                        image::codecs::gif::GifDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Ico => Some(Box::new(
                        image::codecs::ico::IcoDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Jpeg => Some(Box::new(
                        image::codecs::jpeg::JpegDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::OpenExr => Some(Box::new(
                        image::codecs::openexr::OpenExrDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Png => Some(Box::new(
                        image::codecs::png::PngDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Tiff => Some(Box::new(
                        image::codecs::tiff::TiffDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::WebP => Some(Box::new(
                        image::codecs::webp::WebPDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Dds => Some(Box::new(
                        image::codecs::dds::DdsDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Farbfeld => Some(Box::new(
                        image::codecs::farbfeld::FarbfeldDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Hdr => Some(Box::new(
                        image::codecs::hdr::HdrDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Qoi => Some(Box::new(
                        image::codecs::qoi::QoiDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    ImageFormat::Tga => Some(Box::new(
                        image::codecs::tga::TgaDecoder::new(std::io::Cursor::new(img))
                            .map_err(WasmImageError::LibError)?,
                    )),
                    _ => None,
                };

                let metadata = if let Some(mut decoder) = decoder {
                    let (width, height) = decoder.dimensions();

                    let exif = decoder.exif_metadata().map_err(WasmImageError::LibError)?;

                    let other = if let Some(exif) = exif {
                        let reader = exif::Reader::new();

                        let exif = reader.read_raw(exif).map_err(WasmImageError::ExifError)?;

                        let mut other = HashMap::new();

                        for field in exif.fields() {
                            let field_name = field.tag.description();
                            let field_value = field.value.display_as(field.tag);
                            other.insert(
                                field_name.unwrap_or_default().to_string(),
                                field_value.to_string(),
                            );
                        }

                        Some(other)
                    } else {
                        None
                    };

                    Self {
                        width,
                        height,
                        other,
                    }
                } else {
                    let img = image::load_from_memory_with_format(img, format)
                        .map_err(WasmImageError::LibError)?;
                    let (width, height) = img.dimensions();
                    Self {
                        width,
                        height,
                        ..Default::default()
                    }
                };

                Ok(metadata)
            }
            RawSourceImage::Svg(svg) => {
                let tree = resvg::usvg::Tree::from_data(svg, &Options::default()).unwrap();
                let size = tree.size();
                let (width, height) = (size.width() as u32, size.height() as u32);
                Ok(Self {
                    width,
                    height,
                    ..Default::default()
                })
            }
        }
    }
}

#[wasm_bindgen(js_name = loadMetadata)]
/// Loads the metadata of an image file.
/// # Arguments
/// * `file` - The image file to convert.
/// * `src_type` - The MIME type of the source image.
/// * `cb` - A callback function to report progress.
/// # Returns
/// The metadata of the image.
/// # Errors
/// Returns an error if the metadata could not be loaded.
pub fn load_metadata(
    file: &Uint8Array,
    src_type: &str,
    cb: &js_sys::Function,
) -> Result<Metadata, JsValue> {
    let src_type = SourceType::from_mime_type(src_type);

    let this = JsValue::NULL;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(10.0),
        &JsValue::from_str("Starting metadata extraction"),
    );

    let file = file.to_vec();

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(35.0),
        &JsValue::from_str("Loading image"),
    );

    let img = load_raw_image(&file, src_type.as_ref())
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(65.0),
        &JsValue::from_str("Extracting metadata"),
    );

    let metadata =
        Metadata::try_from(img).map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(100.0),
        &JsValue::from_str("Metadata extraction complete"),
    );

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_metadata() {
        let file = include_bytes!("../assets/test.jpeg");
        let mut decoder = image::codecs::jpeg::JpegDecoder::new(std::io::Cursor::new(file))
            .expect("Failed to create decoder");

        let exif = decoder
            .exif_metadata()
            .expect("Failed to extract EXIF data")
            .expect("No EXIF data found");

        let reader = exif::Reader::new();

        let exif = reader.read_raw(exif).expect("Failed to read EXIF data");

        let mut hashmap = HashMap::new();

        for field in exif.fields() {
            //println!("{:?}", field);
            let field_name = field.tag.description();
            let field_value = field.value.display_as(field.tag);
            hashmap.insert(
                field_name.unwrap_or_default().to_string(),
                field_value.to_string(),
            );
        }
        println!("{hashmap:?}");
    }
}
