//! Kinovea `measurement` crate: spatial calibration (line or plane), lens distortion and camera
//! pose. [`CalibrationHelper`] implements `kinovea_kinematics::MeasurementContext`, so kinematics
//! computed with it are expressed in the calibrated units.
//!
//! Module map: [`calibration_helper`] (+ `transform`, `convert`, `text`, `xml`),
//! [`calibrator_plane`] (+ `quad`, `xml`), [`distortion`] (`parameters`, `helper`, `undistort`,
//! `serializer`, `import_agisoft`), [`camera_poser`], [`source`] (tracked calibration source),
//! [`units`], [`calibration_axis`], [`calibrator_type`].

#[cfg(test)]
mod lib_test;

/// Declares a module and its test sibling.
macro_rules! declare {
    ($($m:ident : $t:ident),* $(,)?) => { $(pub mod $m; #[cfg(test)] mod $t;)* };
}

declare!(
    calibration_axis: calibration_axis_test,
    calibration_helper: calibration_helper_test,
    calibrator_plane: calibrator_plane_test,
    calibrator_type: calibrator_type_test,
    camera_poser: camera_poser_test,
    distortion: distortion_test,
    source: source_test,
    units: units_test,
);

/// Enum with C# member names as text form.
macro_rules! text_enum {
    ($(#[$doc:meta])* $name:ident [$($v:ident),+ $(,)?]) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name { $($v),+ }

        impl $name {
            pub const ALL: &'static [$name] = &[$(Self::$v),+];

            pub fn name(self) -> &'static str {
                match self { $(Self::$v => stringify!($v)),+ }
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, String> {
                $(if s == stringify!($v) { return Ok(Self::$v); })+
                Err(format!("unknown {}: {s}", stringify!($name)))
            }
        }
    };
}
pub(crate) use text_enum;

pub use calibration_axis::CalibrationAxis;
pub use calibration_helper::CalibrationHelper;
pub use calibrator_plane::CalibratorPlane;
pub use calibrator_type::CalibratorType;
pub use camera_poser::compute_camera_position;
pub use distortion::{DistortionHelper, DistortionParameters};
pub use source::{StaticSource, TrackedCalibrationSource};
pub use units::Units;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
