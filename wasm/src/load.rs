use image::ImageFormat;

use crate::{
    convert::{
        settings::{Settings, SvgSettings},
        svg::svg_to_png,
    },
    error::WasmImageError,
    source_type::SourceType,
};

pub(crate) enum SourceImage<'s> {
    Raster(image::DynamicImage),
    Svg(&'s [u8]),
}

impl SourceImage<'_> {
    pub(crate) fn rasterize(
        &self,
        settings: Option<&Settings>,
    ) -> Result<image::DynamicImage, WasmImageError> {
        match self {
            Self::Raster(img) => Ok(img.clone()),
            Self::Svg(svg) => {
                let svg_settings = match settings {
                    Some(Settings::Svg(settings)) => *settings,
                    _ => SvgSettings::default(),
                };
                let img = svg_to_png(svg, svg_settings)?;
                image::load_from_memory_with_format(&img, ImageFormat::Png).map_err(Into::into)
            }
        }
    }
}

pub(crate) fn load_image<'a>(
    file: &'a [u8],
    source_type: Option<&'a SourceType>,
) -> Result<SourceImage<'a>, WasmImageError> {
    let loaded_image = match source_type {
        Some(SourceType::Raster(file_type)) => {
            SourceImage::Raster(image::load_from_memory_with_format(file, *file_type)?)
        }
        Some(SourceType::Svg) => SourceImage::Svg(file),
        None => {
            let img = image::load_from_memory(file)
                .map_err(|e| WasmImageError::UnknownFileType(e.to_string()))?;
            SourceImage::Raster(img)
        }
    };

    Ok(loaded_image)
}

pub(crate) enum RawSourceImage<'s> {
    Raster(&'s [u8], ImageFormat),
    Svg(&'s [u8]),
}

pub(crate) fn load_raw_image<'a>(
    file: &'a [u8],
    source_type: Option<&'a SourceType>,
) -> Result<RawSourceImage<'a>, WasmImageError> {
    match source_type {
        Some(SourceType::Raster(file_type)) => Ok(RawSourceImage::Raster(file, *file_type)),
        Some(SourceType::Svg) => Ok(RawSourceImage::Svg(file)),
        None => {
            let img = image::guess_format(file)
                .map_err(|e| WasmImageError::UnknownFileType(e.to_string()))?;

            Ok(RawSourceImage::Raster(file, img))
        }
    }
}
