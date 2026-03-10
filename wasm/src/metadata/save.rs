use std::io::Cursor;

use image::{codecs, ImageEncoder, ImageFormat};
use little_exif::{
    exif_tag::ExifTag,
    filetype::FileExtension,
    metadata::Metadata as LittleExifMetadata,
};

use crate::error::WasmImageError;
use super::editable::{description_to_exif_tag, tag_for_remove};

#[cfg(feature = "wasm")]
use {wasm_bindgen::prelude::*, js_sys::Uint8Array};

#[cfg_attr(feature = "wasm", derive(tsify::Tsify, serde::Deserialize))]
#[cfg_attr(feature = "wasm", tsify(from_wasm_abi))]
#[cfg_attr(not(feature = "wasm"), derive(serde::Deserialize))]
pub struct MetadataChange {
    pub tag: String,
    pub value: Option<String>,
}

#[cfg(feature = "wasm")]
#[derive(tsify::Tsify, serde::Deserialize)]
#[tsify(from_wasm_abi)]
pub struct MetadataChanges(pub Vec<MetadataChange>);

fn file_ext_for_format(format: ImageFormat) -> Option<FileExtension> {
    match format {
        ImageFormat::Jpeg => Some(FileExtension::JPEG),
        ImageFormat::Png  => Some(FileExtension::PNG { as_zTXt_chunk: false }),
        ImageFormat::WebP => Some(FileExtension::WEBP),
        ImageFormat::Tiff => Some(FileExtension::TIFF),
        _ => None,
    }
}

fn strip_gps_tags(exif_meta: &mut LittleExifMetadata) {
    for tag in [
        ExifTag::GPSVersionID(vec![]),
        ExifTag::GPSLatitudeRef(String::new()),
        ExifTag::GPSLatitude(vec![]),
        ExifTag::GPSLongitudeRef(String::new()),
        ExifTag::GPSLongitude(vec![]),
        ExifTag::GPSAltitudeRef(vec![]),
        ExifTag::GPSAltitude(vec![]),
        ExifTag::GPSTimeStamp(vec![]),
        ExifTag::GPSSatellites(String::new()),
        ExifTag::GPSStatus(String::new()),
        ExifTag::GPSMeasureMode(String::new()),
        ExifTag::GPSDOP(vec![]),
        ExifTag::GPSSpeedRef(String::new()),
        ExifTag::GPSSpeed(vec![]),
        ExifTag::GPSTrackRef(String::new()),
        ExifTag::GPSTrack(vec![]),
        ExifTag::GPSImgDirectionRef(String::new()),
        ExifTag::GPSImgDirection(vec![]),
        ExifTag::GPSMapDatum(String::new()),
        ExifTag::GPSDateStamp(String::new()),
    ] {
        exif_meta.remove_tag(tag);
    }
}

fn apply_tag_changes(exif_meta: &mut LittleExifMetadata, changes: &[MetadataChange]) {
    for change in changes {
        match &change.value {
            Some(value) => {
                if let Some(tag) = description_to_exif_tag(&change.tag, value.clone()) {
                    exif_meta.set_tag(tag);
                }
            }
            None => {
                if let Some(tag) = tag_for_remove(&change.tag) {
                    exif_meta.remove_tag(tag);
                }
            }
        }
    }
}

/// Re-encodes the image without preserving any metadata. Used for strip_all.
fn encode_image_to_vec(img: &image::DynamicImage, format: ImageFormat) -> Result<Vec<u8>, WasmImageError> {
    let mut output: Vec<u8> = Vec::new();
    match format {
        ImageFormat::Jpeg => {
            let mut encoder = codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 90);
            encoder.encode_image(img).map_err(WasmImageError::LibError)?;
        }
        ImageFormat::Tiff => {
            let mut cursor = Cursor::new(Vec::<u8>::new());
            let encoder = codecs::tiff::TiffEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                img.color().into(),
            ).map_err(WasmImageError::LibError)?;
            output = cursor.into_inner();
        }
        _ => {
            img.write_to(&mut Cursor::new(&mut output), format)
                .map_err(WasmImageError::LibError)?;
        }
    }
    Ok(output)
}

/// Re-encodes the image and embeds EXIF via ImageEncoder::set_exif_metadata.
/// `exif` must be raw TIFF-structured bytes (the output of LittleExifMetadata::encode()).
/// The encoder is responsible for wrapping them in the format-specific container
/// (APP1 for JPEG, eXIf chunk for PNG, EXIF chunk for WebP).
/// PNG and WebP are supported. TIFF falls back to re-encode without EXIF.
fn encode_image_with_exif_to_vec(
    img: &image::DynamicImage,
    format: ImageFormat,
    exif: Vec<u8>,
) -> Result<Vec<u8>, WasmImageError> {
    let mut output: Vec<u8> = Vec::new();
    match format {
        ImageFormat::Png => {
            let mut encoder = codecs::png::PngEncoder::new(&mut output);
            let _ = encoder.set_exif_metadata(exif);
            encoder.write_image(img.as_bytes(), img.width(), img.height(), img.color().into())
                .map_err(WasmImageError::LibError)?;
        }
        ImageFormat::WebP => {
            let mut encoder = codecs::webp::WebPEncoder::new_lossless(&mut output);
            let _ = encoder.set_exif_metadata(exif);
            encoder.write_image(img.as_bytes(), img.width(), img.height(), img.color().into())
                .map_err(WasmImageError::LibError)?;
        }
        _ => {
            // TIFF: set_exif_metadata is unsupported (returns UnsupportedError).
            // Re-encode without EXIF. The frontend restricts editing to formats
            // that support set_exif_metadata, so this is a defensive fallback.
            img.write_to(&mut Cursor::new(&mut output), format)
                .map_err(WasmImageError::LibError)?;
        }
    }
    Ok(output)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = saveMetadata)]
#[allow(clippy::needless_pass_by_value)]
pub fn save_metadata(
    file: &Uint8Array,
    src_type: &str,
    changes: MetadataChanges,
    strip_all: bool,
    strip_gps: bool,
    cb: &js_sys::Function,
) -> Result<Uint8Array, JsValue> {
    let format = ImageFormat::from_mime_type(src_type)
        .ok_or_else(|| JsValue::from_str("Metadata editing is not supported for this format"))?;

    let file_ext = file_ext_for_format(format)
        .ok_or_else(|| JsValue::from_str("Metadata editing is not supported for this format"))?;

    let this = JsValue::NULL;
    let _ = cb.call2(&this, &JsValue::from_f64(10.0), &JsValue::from_str("Starting metadata save"));

    let file_bytes = file.to_vec();

    let output = if strip_all {
        // Re-encode to strip all metadata. The image crate never preserves EXIF
        // when encoding, and re-encoding also clears XMP, IPTC, and other
        // non-EXIF segments embedded in APP markers.
        let _ = cb.call2(&this, &JsValue::from_f64(30.0), &JsValue::from_str("Loading image"));
        let img = image::load_from_memory_with_format(&file_bytes, format)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let _ = cb.call2(&this, &JsValue::from_f64(70.0), &JsValue::from_str("Encoding image"));
        encode_image_to_vec(&img, format)
            .map_err(|e| JsValue::from_str(&e.to_string()))?
    } else if format == ImageFormat::Jpeg {
        // JPEG metadata edit: write EXIF directly into a copy of the original
        // bytes. little_exif's write_to_vec supports JPEG, and skipping
        // re-encode preserves the original quantization tables exactly.
        let _ = cb.call2(&this, &JsValue::from_f64(40.0), &JsValue::from_str("Applying metadata changes"));
        let mut output = file_bytes.clone();
        let mut exif_meta = LittleExifMetadata::new_from_vec(&file_bytes, file_ext.clone())
            .unwrap_or_else(|_| LittleExifMetadata::new());
        apply_tag_changes(&mut exif_meta, &changes.0);
        if strip_gps {
            strip_gps_tags(&mut exif_meta);
        }
        exif_meta.write_to_vec(&mut output, file_ext)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        output
    } else {
        // Non-JPEG metadata edit: re-encode and embed EXIF via set_exif_metadata.
        // PNG and WebP both implement this. TIFF falls back to re-encode only.
        let _ = cb.call2(&this, &JsValue::from_f64(30.0), &JsValue::from_str("Loading image"));
        let img = image::load_from_memory_with_format(&file_bytes, format)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let _ = cb.call2(&this, &JsValue::from_f64(55.0), &JsValue::from_str("Applying metadata changes"));
        let mut exif_meta = LittleExifMetadata::new_from_vec(&file_bytes, file_ext)
            .unwrap_or_else(|_| LittleExifMetadata::new());
        apply_tag_changes(&mut exif_meta, &changes.0);
        if strip_gps {
            strip_gps_tags(&mut exif_meta);
        }
        let exif_bytes = exif_meta.encode().unwrap_or_default();
        let _ = cb.call2(&this, &JsValue::from_f64(80.0), &JsValue::from_str("Encoding image"));
        encode_image_with_exif_to_vec(&img, format, exif_bytes)
            .map_err(|e| JsValue::from_str(&e.to_string()))?
    };

    let _ = cb.call2(&this, &JsValue::from_f64(100.0), &JsValue::from_str("Metadata save complete"));

    Ok(Uint8Array::from(output.as_slice()))
}
