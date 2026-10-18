//! Kinovea `metadata` crate: the annotation model (keyframes, drawings, styles, fading,
//! trackability) and the KVA file format. No UI type appears in the public API.

#[cfg(test)]
mod lib_test;

pub(crate) use kinovea_core::text_enum;

macro_rules! declare {
    ($($m:ident : $t:ident),* $(,)?) => { $(pub mod $m; #[cfg(test)] mod $t;)* };
}

pub mod drawing;
#[cfg(test)]
mod drawing_test;

declare!(
    color: color_test,
    keyframe: keyframe_test,
    kva: kva_test,
    metadata: metadata_test,
    multi: multi_test,
    singletons: singletons_test,
    trackability: trackability_test,
    fading: fading_test,
    ids: ids_test,
    kva_io: kva_io_test,
    measure_label: measure_label_test,
    style: style_test,
);

pub use color::Color;
pub use drawing::{Drawing, Header, KvaDrawing};
pub use fading::Fading;
pub use keyframe::Keyframe;
pub use kva::{read_kva, write_kva, ExportFlags, KvaError};
pub use metadata::Metadata;
pub use style::{StyleElement, StyleElements, StyleValue};

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
