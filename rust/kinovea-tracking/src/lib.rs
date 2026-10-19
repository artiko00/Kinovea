//! Kinovea `tracking` crate: the object trackers of the original in pure Rust and the trackability
//! runtime that gives each trackable point its position at a time.
//!
//! - [`tracker::Tracker`]: the `AbstractTracker` contract. [`template::TemplateTracker`] (normalized
//!   cross-correlation in [`ncc`], parabola refinement in [`refine`], elliptic mask) and
//!   [`circle::CircleTracker`] (gradient Hough in [`hough`]) port `TrackerTemplateMatching` and
//!   `TrackerCircle`; [`pixels`] holds the OpenCV and GDI+ steps they use, bit-exact where tested.
//! - [`timeline::Timeline`], [`camera::CameraTransform`], [`trackable_point`], [`drawing_tracker`] and
//!   [`manager`]: the runtime over the trackability data of `kinovea-metadata`, which owns the types and
//!   their KVA form.
//! - [`synthetic::Scene`]: the formula frames of the `tracking` oracle cases.
//!
//! Supersedes `Kinovea.ScreenManager/Tracking/Tracking/*` (but `DrawingTrack.cs`, ported in
//! `kinovea-metadata`), `Tracking/Trackability/*` and `Metadata/Timeline.cs`.

pub mod camera;
#[cfg(test)]
pub(crate) mod camera_test;

pub mod drawing_tracker;
#[cfg(test)]
mod drawing_tracker_apply_test;
#[cfg(test)]
pub(crate) mod drawing_tracker_test;

pub mod manager;
#[cfg(test)]
mod manager_test;
#[cfg(test)]
mod manager_trim_test;

pub mod trackable_point;
#[cfg(test)]
mod trackable_point_test;

pub mod ncc;
mod ncc_integral;
mod ncc_map;
mod ncc_masked;
mod ncc_unmasked;
pub mod refine;

#[cfg(test)]
mod lib_test;
#[cfg(test)]
mod ncc_integral_test;
#[cfg(test)]
mod ncc_map_test;
#[cfg(test)]
mod ncc_masked_test;
#[cfg(test)]
mod ncc_reference_test;
#[cfg(test)]
mod ncc_test;
#[cfg(test)]
mod ncc_unmasked_test;
#[cfg(test)]
mod refine_test;

pub mod circle;

pub mod hough;

pub mod pixels;
#[cfg(test)]
mod pixels_test;

pub mod synthetic;
#[cfg(test)]
mod synthetic_test;

pub mod timeline;
#[cfg(test)]
mod timeline_test;

pub mod template;
#[cfg(test)]
mod template_test;

pub mod tracker;
#[cfg(test)]
mod tracker_test;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
