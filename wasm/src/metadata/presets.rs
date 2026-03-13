#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub(super) static PRESET_TIMESTAMPS: &[exif::Tag] = &[
    exif::Tag::DateTime,
    exif::Tag::DateTimeOriginal,
    exif::Tag::DateTimeDigitized,
    exif::Tag::OffsetTime,
    exif::Tag::OffsetTimeOriginal,
    exif::Tag::OffsetTimeDigitized,
    exif::Tag::SubSecTime,
    exif::Tag::SubSecTimeOriginal,
    exif::Tag::SubSecTimeDigitized,
];

pub(super) static PRESET_DEVICE: &[exif::Tag] = &[
    exif::Tag::Make,
    exif::Tag::Model,
    exif::Tag::Software,
    exif::Tag::CameraOwnerName,
    exif::Tag::BodySerialNumber,
    exif::Tag::LensMake,
    exif::Tag::LensModel,
    exif::Tag::LensSerialNumber,
];

pub(super) static PRESET_AUTHOR: &[exif::Tag] = &[
    exif::Tag::ImageDescription,
    exif::Tag::Artist,
    exif::Tag::Copyright,
];

/// GPS tags for the GPS preset. Stripped as a unit by `strip_gps_tags` on save,
/// so they intentionally do not appear in `EDITABLE_TAGS`.
pub(super) static PRESET_GPS: &[exif::Tag] = &[
    exif::Tag::GPSVersionID,
    exif::Tag::GPSLatitudeRef,
    exif::Tag::GPSLatitude,
    exif::Tag::GPSLongitudeRef,
    exif::Tag::GPSLongitude,
    exif::Tag::GPSAltitudeRef,
    exif::Tag::GPSAltitude,
    exif::Tag::GPSTimeStamp,
    exif::Tag::GPSSatellites,
    exif::Tag::GPSStatus,
    exif::Tag::GPSMeasureMode,
    exif::Tag::GPSDOP,
    exif::Tag::GPSSpeedRef,
    exif::Tag::GPSSpeed,
    exif::Tag::GPSTrackRef,
    exif::Tag::GPSTrack,
    exif::Tag::GPSImgDirectionRef,
    exif::Tag::GPSImgDirection,
    exif::Tag::GPSMapDatum,
    exif::Tag::GPSDateStamp,
];

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[derive(serde::Serialize)]
pub struct MetadataPresets {
    pub timestamps: Vec<String>,
    pub device: Vec<String>,
    pub author: Vec<String>,
    pub gps: Vec<String>,
}

fn resolve_preset(tags: &[exif::Tag]) -> Vec<String> {
    tags.iter()
        .filter_map(|tag| tag.description())
        .map(str::to_string)
        .collect()
}

pub(super) fn build_metadata_presets() -> MetadataPresets {
    MetadataPresets {
        timestamps: resolve_preset(PRESET_TIMESTAMPS),
        device: resolve_preset(PRESET_DEVICE),
        author: resolve_preset(PRESET_AUTHOR),
        gps: resolve_preset(PRESET_GPS),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = metadataPresets)]
pub fn metadata_presets() -> MetadataPresets {
    build_metadata_presets()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::editable::EDITABLE_TAGS;

    #[test]
    fn verify_preset_tags_have_descriptions() {
        let all_presets = [PRESET_TIMESTAMPS, PRESET_DEVICE, PRESET_AUTHOR, PRESET_GPS];
        for tags in all_presets {
            for tag in tags {
                assert!(
                    tag.description().is_some(),
                    "{tag:?} has no description in kamadak-exif — was it removed or renamed?"
                );
            }
        }
        // No tag should be silently dropped by filter_map.
        let presets = build_metadata_presets();
        assert_eq!(
            presets.timestamps.len(),
            PRESET_TIMESTAMPS.len(),
            "timestamps: some tags lost their description"
        );
        assert_eq!(
            presets.device.len(),
            PRESET_DEVICE.len(),
            "device: some tags lost their description"
        );
        assert_eq!(
            presets.author.len(),
            PRESET_AUTHOR.len(),
            "author: some tags lost their description"
        );
        assert_eq!(
            presets.gps.len(),
            PRESET_GPS.len(),
            "gps: some tags lost their description"
        );
    }

    #[test]
    fn verify_preset_membership() {
        let editable_set: std::collections::HashSet<exif::Tag> =
            EDITABLE_TAGS.iter().map(|e| e.tag).collect();

        // Non-GPS preset tags must all be in EDITABLE_TAGS.
        for (name, tags) in [
            ("timestamps", PRESET_TIMESTAMPS),
            ("device", PRESET_DEVICE),
            ("author", PRESET_AUTHOR),
        ] {
            for tag in tags {
                assert!(
                    editable_set.contains(tag),
                    "{tag:?} is in the '{name}' preset but not in EDITABLE_TAGS"
                );
            }
        }

        // GPS tags must NOT be in EDITABLE_TAGS (stripped as a unit, not individually).
        for tag in PRESET_GPS {
            assert!(
                !editable_set.contains(tag),
                "{tag:?} is in the GPS preset but also appears in EDITABLE_TAGS"
            );
        }
    }
}
