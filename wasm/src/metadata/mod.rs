mod editable;
mod extract;
mod presets;
mod save;

pub use extract::{load_metadata, Metadata};
pub use presets::MetadataPresets;
pub use save::MetadataChange;

#[cfg(feature = "wasm")]
pub use save::{save_metadata, MetadataChanges};
