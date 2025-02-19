use std::{
    collections::HashMap,
    io::{BufReader, Cursor},
};

use exif::Exif;
use image::{codecs, GenericImageView, ImageDecoder, ImageFormat};
use resvg::usvg::Options;

use crate::{
    error::WasmImageError,
    load::{load_raw_image, RawSourceImage},
    source_type::SourceType,
};

#[cfg(feature = "wasm")] 
use {
    js_sys::Uint8Array,
    wasm_bindgen::prelude::*,
};

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(from_wasm_abi, into_wasm_abi))]
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Metadata {
    pub width: u32,
    pub height: u32,
    pub other: Option<HashMap<String, String>>,
    pub errors: Option<Vec<String>>,
}

fn exif_to_hashmap(exif: &Exif) -> HashMap<String, String> {
    exif.fields()
        .map(|field| {
            (
                field.tag.description().unwrap_or_default().to_string(),
                field.value.display_as(field.tag).to_string(),
            )
        })
        .collect()
}

fn get_decoder<'a>(
    format: ImageFormat,
    img: &'a [u8],
) -> Result<Box<dyn ImageDecoder + 'a>, WasmImageError> {
    let decoder: Box<dyn ImageDecoder> = match format {
        ImageFormat::Bmp => Box::new(
            codecs::bmp::BmpDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("BMP".to_string(), e.to_string()))?,
        ),
        ImageFormat::Gif => Box::new(
            codecs::gif::GifDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("GIF".to_string(), e.to_string()))?,
        ),
        ImageFormat::Ico => Box::new(
            codecs::ico::IcoDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("ICO".to_string(), e.to_string()))?,
        ),
        ImageFormat::Jpeg => Box::new(
            codecs::jpeg::JpegDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("JPEG".to_string(), e.to_string()))?,
        ),
        ImageFormat::OpenExr => Box::new(
            codecs::openexr::OpenExrDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("OpenEXR".to_string(), e.to_string()))?,
        ),
        ImageFormat::Png => Box::new(
            codecs::png::PngDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("PNG".to_string(), e.to_string()))?,
        ),
        ImageFormat::Tiff => Box::new(
            codecs::tiff::TiffDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("TIFF".to_string(), e.to_string()))?,
        ),
        ImageFormat::WebP => Box::new(
            codecs::webp::WebPDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("WebP".to_string(), e.to_string()))?,
        ),
        ImageFormat::Dds => Box::new(
            codecs::dds::DdsDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("DDS".to_string(), e.to_string()))?,
        ),
        ImageFormat::Farbfeld => Box::new(
            codecs::farbfeld::FarbfeldDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("Farbfeld".to_string(), e.to_string()))?,
        ),
        ImageFormat::Hdr => Box::new(
            codecs::hdr::HdrDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("HDR".to_string(), e.to_string()))?,
        ),
        ImageFormat::Qoi => Box::new(
            codecs::qoi::QoiDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("QOI".to_string(), e.to_string()))?,
        ),
        ImageFormat::Tga => Box::new(
            codecs::tga::TgaDecoder::new(Cursor::new(img))
                .map_err(|e| WasmImageError::DecoderError("TGA".to_string(), e.to_string()))?,
        ),
        _ => {
            return Err(WasmImageError::DecoderError(
                "Unknown".to_string(),
                format!("Unknown format: {format:?}"),
            ))
        }
    };

    Ok(decoder)
}

fn get_exif_errors(error: exif::Error) -> Result<Vec<String>, WasmImageError> {
    let mut errors = Vec::new();
    error
        .distill_partial_result(|exif_errors| {
            errors = exif_errors
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
        })
        .map_err(WasmImageError::ExifError)?;
    Ok(errors)
}

impl TryFrom<RawSourceImage<'_>> for Metadata {
    type Error = WasmImageError;

    fn try_from(img: RawSourceImage) -> Result<Self, Self::Error> {
        match img {
            RawSourceImage::Raster(img, format) => {
                // See https://docs.rs/image/latest/image/trait.ImageDecoder.html#implementors
                let decoder = get_decoder(format, img).ok();

                let metadata = if let Some(mut decoder) = decoder {
                    let (width, height) = decoder.dimensions();

                    let mut reader = exif::Reader::new();
                    reader.continue_on_error(true);

                    let exif = match format {
                        // Also HEIF, HEIC, not supported by image-rs though
                        ImageFormat::Tiff
                        | ImageFormat::Jpeg
                        | ImageFormat::Avif
                        | ImageFormat::Png
                        | ImageFormat::WebP => {
                            let data_reader = Cursor::new(img);
                            let mut data_reader = BufReader::new(data_reader);
                            reader.read_from_container(&mut data_reader).map_err(|e| {
                                get_exif_errors(e).unwrap_or_else(|e| vec![e.to_string()])
                            })
                        }
                        _ => {
                            if let Ok(Some(exif)) = decoder.exif_metadata() {
                                reader.read_raw(exif).map_err(|e| {
                                    get_exif_errors(e).unwrap_or_else(|e| vec![e.to_string()])
                                })
                            } else {
                                Err(Vec::new())
                            }
                        }
                    };

                    let (other, errors) = match exif {
                        Ok(exif) => (Some(exif_to_hashmap(&exif)), None),
                        Err(errors) => (None, Some(errors)),
                    };

                    Self {
                        width,
                        height,
                        other,
                        errors,
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

#[cfg(feature = "wasm")]
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

#[cfg(not(feature = "wasm"))]
/// Loads the metadata of an image file.
/// # Arguments
/// * `file` - The image file to convert.
/// * `src_type` - The MIME type of the source image.
/// # Returns
/// The metadata of the image.
/// # Errors
/// Returns an error if the metadata could not be loaded.
pub fn load_metadata(file: &[u8], src_type: &str) -> Result<Metadata, WasmImageError> {
    let src_type = SourceType::from_mime_type(src_type);

    let img = load_raw_image(file, src_type.as_ref())?;

    Metadata::try_from(img)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_metadata() {
        let file = include_bytes!("../assets/test.jpeg");
        let mut decoder =
            codecs::jpeg::JpegDecoder::new(Cursor::new(file)).expect("Failed to create decoder");

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

    #[test]
    fn test_load_webp_metadata() {
        let file = include_bytes!("../assets/exif.webp");

        let mut reader = exif::Reader::new();
        reader.continue_on_error(true);

        let exif = reader
            .read_from_container(&mut BufReader::new(Cursor::new(file)))
            .expect("Failed to read EXIF data");

        for field in exif.fields() {
            println!("{field:?}");
        }
    }
}
