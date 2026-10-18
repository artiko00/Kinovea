//! Kinovea `kinematics` crate: trajectory filtering and linear/angular kinematics.
//!
//! The crate has no global state. Everything that the original read from `CalibrationHelper`
//! and `PreferencesManager` enters through the [`MeasurementContext`] trait: image to world
//! mapping, frame timing, unit conversions and the two preferences (`filtering_enabled`,
//! `high_speed_smoothing_enabled`). `add-calibration-and-lens-distortion` implements the trait
//! for real calibrations; [`PixelContext`] serves tests and the uncalibrated mode.
//!
//! Module map: [`butterworth`] (filter + automatic cutoff), [`moving_average`], [`stats`],
//! [`filtering_result`], [`trajectory`] (`FilteredTrajectory`), [`component`] (`Kinematics`
//! keys), [`time_series`], [`linear`], [`angular`], [`context`].

#[cfg(test)]
mod lib_test;

macro_rules! with_tests {
    ($($m:ident => $t:ident;)*) => { $(pub mod $m; #[cfg(test)] mod $t;)* };
}

with_tests! {
    angular => angular_test;
    butterworth => butterworth_test;
    component => component_test;
    context => context_test;
    filtering_result => filtering_result_test;
    linear => linear_test;
    moving_average => moving_average_test;
    stats => stats_test;
    time_series => time_series_test;
    trajectory => trajectory_test;
}

pub use angular::{build_angular_kinematics, AngleOptions, AngularError};
pub use butterworth::{filter_samples as butterworth_filter, Filtered, TooFewSamples};
pub use component::{Component, Kinematics};
pub use context::{MeasurementContext, PixelContext};
pub use filtering_result::FilteringResult;
pub use linear::build_linear_kinematics;
pub use time_series::{pad, TimeSeriesCollection};
pub use trajectory::FilteredTrajectory;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
