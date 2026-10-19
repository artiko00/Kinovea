//! Kinovea `tracking` crate. Behavior is added by later OpenSpec changes.

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
