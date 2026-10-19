//! Value conversions shared by the collectors: the time axis, the export space and the
//! calibrated angle (`Metadata.GetNumericalTime`, `MeasurementSerializationHelper`,
//! `AngleHelper.ComputeCalibratedAngle`).

use crate::measured_data::ExportPrefs;
use kinovea_core::preferences::enums::ExportSpace;
use kinovea_core::timecode::TimecodeFormat;
use kinovea_core::units::AngleUnit;
use kinovea_geometry::helpers::{angle_at, degrees};
use kinovea_geometry::PointF;
use kinovea_metadata::Metadata;
use std::f64::consts::TAU;

/// The origin a timestamp is measured from (`TimeType.UserOrigin` / `TimeType.Absolute`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeBase {
    UserOrigin,
    Absolute,
}

/// How the legs of an angle are read (`AngleHelper.Update` arguments).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AngleFlags {
    pub signed: bool,
    pub counter_clockwise: bool,
    pub supplementary: bool,
}

/// What a collector needs from the document and the export preferences.
pub struct Scope<'a> {
    pub metadata: &'a Metadata,
    pub prefs: &'a ExportPrefs,
}

impl Scope<'_> {
    /// `GetNumericalTime`: a timestamp as a number in the preferred timecode format.
    pub fn time(&self, timestamps: i64, base: TimeBase) -> f32 {
        let video = &self.metadata.video;
        let format = match self.prefs.timecode_format {
            TimecodeFormat::Normalized => TimecodeFormat::ClassicTime,
            other => other,
        };
        let actual = match base {
            TimeBase::UserOrigin => timestamps - video.time_origin,
            TimeBase::Absolute => timestamps,
        };
        let frames = if video.average_timestamps_per_frame != 0.0 {
            (actual as f64 / video.average_timestamps_per_frame).round_ties_even()
        } else {
            0.0
        };
        let milliseconds = frames * video.baseline_frame_interval_ms / video.high_speed_factor;
        let time = match format {
            TimecodeFormat::Frames => frames,
            TimecodeFormat::Milliseconds => milliseconds,
            TimecodeFormat::Microseconds => milliseconds * 1000.0,
            TimecodeFormat::TenThousandthOfHours => milliseconds / 360.0,
            TimecodeFormat::HundredthOfMinutes => milliseconds / 600.0,
            TimecodeFormat::Timestamps => timestamps as f64,
            _ => milliseconds / 1000.0,
        };
        time as f32
    }

    /// `GetPoint` in world space (lens distortion first), the point itself in image space.
    pub fn point(&self, p: PointF) -> PointF {
        match self.prefs.export_space {
            ExportSpace::WorldSpace => self.metadata.calibration.point(p),
            ExportSpace::ImageSpace => p,
        }
    }

    /// `GetPointAtTime` in world space: a tracked calibration object is read at `timestamp`.
    pub fn point_at(&self, p: PointF, timestamp: i64) -> PointF {
        match self.prefs.export_space {
            ExportSpace::WorldSpace => self.metadata.calibration.point_at_time(p, timestamp),
            ExportSpace::ImageSpace => p,
        }
    }

    /// `AngleHelper.Update` followed by `ComputeCalibratedAngle`, in the calibration's angle
    /// unit. The legs are read in world space whatever the export space, as the original does.
    /// A degenerate angle (a leg of length 0) has no value and reads 0, which is what a fresh
    /// helper of the original holds.
    pub fn angle(&self, o: PointF, a: PointF, b: PointF, flags: AngleFlags) -> f32 {
        if o == a || o == b {
            return 0.0;
        }
        let (a, b) = if flags.supplementary {
            (b, PointF::new(2.0 * o.x - a.x, 2.0 * o.y - a.y))
        } else {
            (a, b)
        };
        let calibration = &self.metadata.calibration;
        let (o, a, b) = (
            calibration.point(o),
            calibration.point(a),
            calibration.point(b),
        );
        let radians = if flags.counter_clockwise {
            angle_at(o, a, b)
        } else {
            angle_at(o, b, a)
        };
        let radians = if !flags.signed && radians < 0.0 {
            (TAU + f64::from(radians)) as f32
        } else {
            radians
        };
        match self.metadata.calibration.units().angle {
            AngleUnit::Degree => degrees(radians),
            AngleUnit::Radian => radians,
        }
    }
}
