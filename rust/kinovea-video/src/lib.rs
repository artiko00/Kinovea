//! Kinovea `video` crate: the [`reader::VideoReader`] contract every video source implements, its
//! frame containers ([`cache::Cache`], dense, and [`prebuffer::PreBuffer`], bounded and fed by a
//! decoding thread), the capture [`pipeline`] (ring buffer and batch consumers), the reader
//! [`registry`] and a [`synthetic::SyntheticReader`] whose frames carry their own timestamp.
//!
//! Supersedes `Kinovea.Video` (`VideoReaders/{VideoReader,VideoTypeManager,VideoCapabilities,
//! SupportedExtensionsAttribute}.cs`, `Caching/*`, `VideoGeometry/*`, `VideoInfo.cs`,
//! `VideoSummary.cs`, `Fraction.cs`, `FitHelper.cs`, `WorkingZoneRequest.cs`, the result enums,
//! `TimestampRelation.cs`, `Delegates.cs`) and all of `Kinovea.Pipeline`. The readers that decode
//! files live in later changes.

#[cfg(test)]
mod lib_test;

pub mod cache;
#[cfg(test)]
mod cache_test;
pub mod capabilities;
#[cfg(test)]
mod capabilities_test;
pub mod closest;
#[cfg(test)]
mod closest_test;
pub mod container;
#[cfg(test)]
mod container_test;
pub mod decoding;
#[cfg(test)]
mod decoding_test;
pub mod enumerate;
#[cfg(test)]
mod enumerate_test;
pub mod fit_helper;
#[cfg(test)]
mod fit_helper_test;
pub mod frame;
#[cfg(test)]
mod frame_test;
pub mod geometry;
#[cfg(test)]
mod geometry_test;
pub mod info;
#[cfg(test)]
mod info_test;
pub mod pipeline;
pub mod prebuffer;
#[cfg(test)]
mod prebuffer_test;
pub mod reader;
#[cfg(test)]
mod reader_test;
pub mod registry;
#[cfg(test)]
mod registry_test;
pub mod relation;
#[cfg(test)]
mod relation_test;
pub mod results;
#[cfg(test)]
mod results_test;
pub mod snapshot;
#[cfg(test)]
mod snapshot_test;
pub mod state;
#[cfg(test)]
mod state_test;
pub mod summary;
#[cfg(test)]
mod summary_test;
pub mod synthetic;
pub mod synthetic_reader;
#[cfg(test)]
mod synthetic_reader_test;
#[cfg(test)]
mod synthetic_test;
#[cfg(test)]
mod temp_dir;
#[cfg(test)]
mod temp_dir_test;
pub mod wildcard;
#[cfg(test)]
mod wildcard_test;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
