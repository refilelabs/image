use image::ImageFormat;

pub enum SourceType {
    Raster(RasterType),
    Svg,
}

#[derive(Clone, Copy)]
pub enum RasterType {
    BuiltIn(ImageFormat),
    Custom(CustomRasterType),
}

#[derive(Clone, Copy)]
pub enum CustomRasterType {
    Raw,
}

impl SourceType {
    pub fn from_mime_type(mime_type: &str) -> Option<Self> {
        match ImageFormat::from_mime_type(mime_type) {
            Some(format) => Some(Self::Raster(RasterType::BuiltIn(format))),
            None => from_custom_mime_type(mime_type),
        }
    }
}

/// This function is used to handle types that don't usually have an associated mime type
fn from_custom_mime_type(mime_type: &str) -> Option<SourceType> {
    match mime_type.to_lowercase().as_str() {
        // SVG
        "image/svg+xml" => Some(SourceType::Svg),
        // Farbfeld (often application/octet-stream is not inferred by image-rs, see https://docs.rs/image/0.25.5/src/image/image.rs.html#166-193)
        "image/farbfeld" => Some(SourceType::Raster(RasterType::BuiltIn(
            ImageFormat::Farbfeld,
        ))),
        // Raw (often is image/x-dcraw, but differs between cameras, https://stackoverflow.com/questions/43473056/which-mime-type-should-be-used-for-a-raw-image - we just default to image/x-dcraw here)
        "image/raw" | "image/arw" | "image/cr2" | "image/x-dcraw" => Some(SourceType::Raster(
            RasterType::Custom(CustomRasterType::Raw),
        )),
        _ => None,
    }
}
