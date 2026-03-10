use little_exif::exif_tag::ExifTag;

#[cfg(feature = "wasm")]
use {js_sys::Array, wasm_bindgen::prelude::*, JsValue};

pub(super) struct EditableTag {
    pub tag:         exif::Tag,
    pub constructor: fn(String) -> ExifTag,
}

/// Single source of truth for editable EXIF tags.
/// The display string comes from `tag.description()` at runtime — never hardcoded here.
pub(super) static EDITABLE_TAGS: &[EditableTag] = &[
    // IFD0
    EditableTag { tag: exif::Tag::ImageDescription, constructor: ExifTag::ImageDescription },
    EditableTag { tag: exif::Tag::Make,             constructor: ExifTag::Make             },
    EditableTag { tag: exif::Tag::Model,            constructor: ExifTag::Model            },
    EditableTag { tag: exif::Tag::Software,         constructor: ExifTag::Software         },
    EditableTag { tag: exif::Tag::DateTime,         constructor: ExifTag::ModifyDate       },
    EditableTag { tag: exif::Tag::Artist,           constructor: ExifTag::Artist           },
    EditableTag { tag: exif::Tag::Copyright,        constructor: ExifTag::Copyright        },
    // EXIF IFD
    EditableTag { tag: exif::Tag::DateTimeOriginal,    constructor: ExifTag::DateTimeOriginal    },
    EditableTag { tag: exif::Tag::DateTimeDigitized,   constructor: ExifTag::CreateDate          },
    EditableTag { tag: exif::Tag::OffsetTime,          constructor: ExifTag::OffsetTime          },
    EditableTag { tag: exif::Tag::OffsetTimeOriginal,  constructor: ExifTag::OffsetTimeOriginal  },
    EditableTag { tag: exif::Tag::OffsetTimeDigitized, constructor: ExifTag::OffsetTimeDigitized },
    EditableTag { tag: exif::Tag::SubSecTime,          constructor: ExifTag::SubSecTime          },
    EditableTag { tag: exif::Tag::SubSecTimeOriginal,  constructor: ExifTag::SubSecTimeOriginal  },
    EditableTag { tag: exif::Tag::SubSecTimeDigitized, constructor: ExifTag::SubSecTimeDigitized },
    EditableTag { tag: exif::Tag::SpectralSensitivity, constructor: ExifTag::SpectralSensitivity },
    EditableTag { tag: exif::Tag::RelatedSoundFile,    constructor: ExifTag::RelatedSoundFile    },
    EditableTag { tag: exif::Tag::ImageUniqueID,       constructor: ExifTag::ImageUniqueID       },
    EditableTag { tag: exif::Tag::CameraOwnerName,     constructor: ExifTag::OwnerName           },
    EditableTag { tag: exif::Tag::BodySerialNumber,    constructor: ExifTag::SerialNumber        },
    EditableTag { tag: exif::Tag::LensMake,            constructor: ExifTag::LensMake            },
    EditableTag { tag: exif::Tag::LensModel,           constructor: ExifTag::LensModel           },
    EditableTag { tag: exif::Tag::LensSerialNumber,    constructor: ExifTag::LensSerialNumber    },
];

pub(super) fn description_to_exif_tag(desc: &str, value: String) -> Option<ExifTag> {
    EDITABLE_TAGS
        .iter()
        .find(|e| e.tag.description() == Some(desc))
        .map(|e| (e.constructor)(value))
}

/// Tags that cannot be edited as strings but can still be deleted.
/// little_exif only needs a correctly-typed dummy value for remove_tag —
/// the actual content doesn't matter, just that the variant matches.
pub(super) struct DeletableTag {
    pub tag:     exif::Tag,
    pub deleter: fn() -> ExifTag,
}

pub(super) static DELETABLE_TAGS: &[DeletableTag] = &[
    // IFD0
    DeletableTag { tag: exif::Tag::Orientation,       deleter: || ExifTag::Orientation(vec![])       },
    // EXIF IFD — exposure
    DeletableTag { tag: exif::Tag::ExposureTime,      deleter: || ExifTag::ExposureTime(vec![])      },
    DeletableTag { tag: exif::Tag::FNumber,           deleter: || ExifTag::FNumber(vec![])           },
    DeletableTag { tag: exif::Tag::ExposureProgram,   deleter: || ExifTag::ExposureProgram(vec![])   },
    DeletableTag { tag: exif::Tag::ISOSpeed,           deleter: || ExifTag::ISO(vec![])               },
    DeletableTag { tag: exif::Tag::ShutterSpeedValue, deleter: || ExifTag::ShutterSpeedValue(vec![]) },
    DeletableTag { tag: exif::Tag::ApertureValue,     deleter: || ExifTag::ApertureValue(vec![])     },
    DeletableTag { tag: exif::Tag::BrightnessValue,   deleter: || ExifTag::BrightnessValue(vec![])   },
    DeletableTag { tag: exif::Tag::ExposureBiasValue, deleter: || ExifTag::ExposureCompensation(vec![]) },
    DeletableTag { tag: exif::Tag::MaxApertureValue,  deleter: || ExifTag::MaxApertureValue(vec![])  },
    DeletableTag { tag: exif::Tag::SubjectDistance,   deleter: || ExifTag::SubjectDistance(vec![])   },
    DeletableTag { tag: exif::Tag::MeteringMode,      deleter: || ExifTag::MeteringMode(vec![])      },
    DeletableTag { tag: exif::Tag::LightSource,       deleter: || ExifTag::LightSource(vec![])       },
    DeletableTag { tag: exif::Tag::Flash,             deleter: || ExifTag::Flash(vec![])             },
    DeletableTag { tag: exif::Tag::FocalLength,       deleter: || ExifTag::FocalLength(vec![])       },
    // EXIF IFD — post-processing / scene
    DeletableTag { tag: exif::Tag::ColorSpace,        deleter: || ExifTag::ColorSpace(vec![])        },
    DeletableTag { tag: exif::Tag::ExposureMode,      deleter: || ExifTag::ExposureMode(vec![])      },
    DeletableTag { tag: exif::Tag::WhiteBalance,      deleter: || ExifTag::WhiteBalance(vec![])      },
    DeletableTag { tag: exif::Tag::DigitalZoomRatio,  deleter: || ExifTag::DigitalZoomRatio(vec![])  },
    DeletableTag { tag: exif::Tag::SceneCaptureType,  deleter: || ExifTag::SceneCaptureType(vec![])  },
    DeletableTag { tag: exif::Tag::Contrast,          deleter: || ExifTag::Contrast(vec![])          },
    DeletableTag { tag: exif::Tag::Saturation,        deleter: || ExifTag::Saturation(vec![])        },
    DeletableTag { tag: exif::Tag::Sharpness,         deleter: || ExifTag::Sharpness(vec![])         },
];

/// Returns an ExifTag suitable for remove_tag, checking both editable and deletable-only tags.
pub(super) fn tag_for_remove(desc: &str) -> Option<ExifTag> {
    description_to_exif_tag(desc, String::new())
        .or_else(|| DELETABLE_TAGS.iter().find(|e| e.tag.description() == Some(desc)).map(|e| (e.deleter)()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = editableFields)]
pub fn editable_fields() -> Array {
    EDITABLE_TAGS
        .iter()
        .filter_map(|e| e.tag.description())
        .map(JsValue::from_str)
        .collect()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = deletableFields)]
pub fn deletable_fields() -> Array {
    EDITABLE_TAGS
        .iter()
        .filter_map(|e| e.tag.description())
        .chain(DELETABLE_TAGS.iter().filter_map(|e| e.tag.description()))
        .map(JsValue::from_str)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_editable_tags_have_descriptions() {
        for e in EDITABLE_TAGS {
            assert!(
                e.tag.description().is_some(),
                "{:?} has no description in kamadak-exif — was it removed or renamed?",
                e.tag
            );
        }
        // Descriptions must be unique so the lookup in description_to_exif_tag is unambiguous.
        let descs: Vec<&str> = EDITABLE_TAGS.iter().filter_map(|e| e.tag.description()).collect();
        let unique: std::collections::HashSet<&str> = descs.iter().copied().collect();
        assert_eq!(descs.len(), unique.len(), "EDITABLE_TAGS contains duplicate descriptions");
        assert_eq!(EDITABLE_TAGS.len(), 23, "Expected 23 editable tags — update this count if you add or remove entries");
    }

    #[test]
    fn verify_deletable_tags() {
        let editable_set: std::collections::HashSet<exif::Tag> =
            EDITABLE_TAGS.iter().map(|e| e.tag).collect();

        for e in DELETABLE_TAGS {
            assert!(
                e.tag.description().is_some(),
                "{:?} has no description in kamadak-exif — was it removed or renamed?",
                e.tag
            );
            assert!(
                !editable_set.contains(&e.tag),
                "{:?} is in both EDITABLE_TAGS and DELETABLE_TAGS — remove it from DELETABLE_TAGS",
                e.tag
            );
        }
    }
}
