//! KVA 2.x documents: reading into [`Metadata`](crate::metadata::Metadata) with context
//! remapping, and writing with the element order and formatting of the original application.

mod context;
#[cfg(test)]
mod context_test;
mod reader;
#[cfg(test)]
mod reader_test;
mod writer;
#[cfg(test)]
mod writer_test;

pub use context::{header_metadata, ImportContext};
pub use reader::{read_kva, KvaError};
pub use writer::{write_kva, ExportFlags};

use crate::metadata::Metadata;

/// Loads a KVA as the oracle's `KvaArea.Load` does: the video is set up (`PostSetupVideo`) before the
/// document is merged, as the player does when a video opens.
pub fn load_kva(xml: &str) -> Result<Metadata, KvaError> {
    let xml = xml.trim_start_matches('\u{feff}');
    let mut m = header_metadata(xml);
    m.post_setup_video();
    read_kva(&mut m, xml)?;
    Ok(m)
}

/// Producer written in the header; the oracle fixes the version to `0.0`.
pub const PRODUCER: &str = "Kinovea";
