//! Port of `Kinovea.Video/VideoInfo.cs` (`MakeEmpty`, `IsSingleFrame`) and
//! `Kinovea.Video/Fraction.cs`.

use kinovea_core::image_options::ImageRotation;
use kinovea_geometry::Size;

/// `Kinovea.Video.Fraction`: a rational used for the sample aspect ratio.
/// `Default` is `0/0`, which is empty as in `new Fraction()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

impl Fraction {
    pub const fn new(numerator: i64, denominator: i64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// True when either part is zero.
    pub fn is_empty(self) -> bool {
        self.numerator == 0 || self.denominator == 0
    }
}

/// `VideoInfo`: what the file itself says about its timing and geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct VideoInfo {
    /// Full path on the file system.
    pub file_path: String,
    /// Raw image size without rotation or pixel aspect ratio.
    pub original_size: Size,
    /// Rotation flag stored in the file.
    pub original_rotation: ImageRotation,
    pub pixel_aspect_ratio: f64,
    pub sample_aspect_ratio: Fraction,
    pub average_timestamps_per_frame: f64,
    pub average_timestamps_per_second: f64,
    pub frames_per_second: f64,
    pub frame_interval_ms: f64,
    pub first_timestamp: i64,
    pub last_timestamp: i64,
    pub duration_timestamps: i64,
}

impl VideoInfo {
    /// `VideoInfo.MakeEmpty()`.
    pub fn empty() -> Self {
        Self {
            file_path: String::new(),
            original_size: Size::default(),
            original_rotation: ImageRotation::Rotate0,
            pixel_aspect_ratio: 1.0,
            sample_aspect_ratio: Fraction::default(),
            average_timestamps_per_frame: 0.0,
            average_timestamps_per_second: 0.0,
            frames_per_second: 0.0,
            frame_interval_ms: 0.0,
            first_timestamp: 0,
            last_timestamp: 0,
            duration_timestamps: 0,
        }
    }

    /// `IsSingleFrame`: the duration is exactly one timestamp.
    pub fn is_single_frame(&self) -> bool {
        self.duration_timestamps == 1
    }
}

/// Milliseconds between frames at `frames_per_second`. Non-positive rates give 0,
/// the value `MakeEmpty` uses, instead of an infinite interval.
pub fn frame_interval_ms(frames_per_second: f64) -> f64 {
    if frames_per_second > 0.0 {
        1000.0 / frames_per_second
    } else {
        0.0
    }
}
