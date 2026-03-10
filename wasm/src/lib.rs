#![warn(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
#![forbid(unsafe_code)]

pub mod convert;
pub mod error;
pub(crate) mod load;
pub mod metadata;
#[cfg(feature = "wasm")]
pub(crate) mod progress;
pub mod resize;
pub(crate) mod source_type;
pub mod view;

pub use {convert::convert_image, metadata::load_metadata, resize::resize_image, view::get_pixels};

#[cfg(feature = "wasm")]
pub use metadata::save_metadata;
