use std::io::{BufWriter, Cursor, Write};

use image::ImageFormat;
use imagepipe::SRGBImage;

use crate::{
    convert::{
        settings::{Settings, SvgSettings},
        svg::svg_to_png,
    },
    error::WasmImageError,
    source_type::{CustomRasterType, RasterType, SourceType},
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
        Some(SourceType::Raster(RasterType::BuiltIn(file_type))) => {
            SourceImage::Raster(image::load_from_memory_with_format(file, *file_type)?)
        }
        Some(SourceType::Raster(RasterType::Custom(custom_type))) => match custom_type {
            CustomRasterType::Raw => {
                let mut cursor = Cursor::new(file);

                let img = rawloader::decode(&mut cursor)
                    .map_err(|e| WasmImageError::UnknownFileType(e.to_string()))?;

                let mut pipeline =
                    imagepipe::Pipeline::new_from_source(imagepipe::ImageSource::Raw(img))
                        .map_err(|e| WasmImageError::UnknownFileType(e.to_string()))?;

                let SRGBImage {
                    data,
                    width,
                    height,
                } = pipeline
                    .output_8bit(None)
                    .map_err(|e| WasmImageError::UnknownFileType(e.to_string()))?;

                let img = image::RgbImage::from_raw(width as u32, height as u32, data).ok_or_else(
                    || WasmImageError::UnknownFileType("Failed to create image".into()),
                )?;

                return Ok(SourceImage::Raster(image::DynamicImage::ImageRgb8(img)));
            }
        },
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
    Raster(&'s [u8], RasterType),
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

            Ok(RawSourceImage::Raster(file, RasterType::BuiltIn(img)))
        }
    }
}
