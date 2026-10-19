//! Kinovea `video` crate. Behavior is added by later OpenSpec changes.

#[cfg(test)]
mod lib_test;

pub mod capabilities;
#[cfg(test)]
mod capabilities_test;
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
pub mod results;
#[cfg(test)]
mod results_test;
pub mod state;
#[cfg(test)]
mod state_test;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
