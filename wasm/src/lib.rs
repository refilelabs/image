#![feature(trivial_bounds)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

pub(crate) mod convert;
pub(crate) mod error;
pub(crate) mod load;
pub(crate) mod metadata;
pub(crate) mod source_type;

pub use convert::convert_image;
