use image::ImageError;
use resvg::usvg::Error as SvgError;

#[derive(thiserror::Error, Debug)]
pub enum WasmImageError {
    #[error("Unknown file type: {0}")]
    UnknownFileType(String),
    #[error("Image library error: {0}")]
    LibError(#[from] ImageError),
    #[error("Parsing error: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("SVG error: {0}")]
    SvgError(#[from] SvgError),
    #[error("Encoding error: {0}")]
    EncodingError(String),
    #[error("Decoder error: Could not create {0} decoder: {1}")]
    DecoderError(String, String),
    #[error("Exif error: {0}")]
    ExifError(#[from] exif::Error),
}
