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

/// Producer written in the header; the oracle fixes the version to `0.0`.
pub const PRODUCER: &str = "Kinovea";
