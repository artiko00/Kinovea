//! Where the calibration object and the coordinate system origin are at a given time
//! (the three callbacks of the original `CalibrationHelper.Initialize`).

use crate::calibrator_type::CalibratorType;
use kinovea_geometry::{PointF, Quadrilateral};
use uuid::Uuid;

pub trait TrackedCalibrationSource {
    /// Image position of the coordinate system origin at `time`.
    fn calibration_origin(&self, time: i64) -> PointF;
    /// Image quadrilateral (or segment in A, B) of the calibration drawing at `time`.
    fn calibration_quad(&self, time: i64, kind: CalibratorType, drawing: Uuid) -> Quadrilateral;
    /// Whether the drawing has tracking data.
    fn has_tracking_data(&self, drawing: Uuid) -> bool;
}

/// No tracking: empty origin, empty quad, nothing tracked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StaticSource;

impl TrackedCalibrationSource for StaticSource {
    fn calibration_origin(&self, _time: i64) -> PointF {
        PointF::EMPTY
    }

    fn calibration_quad(&self, _time: i64, _kind: CalibratorType, _drawing: Uuid) -> Quadrilateral {
        Quadrilateral::empty()
    }

    fn has_tracking_data(&self, _drawing: Uuid) -> bool {
        false
    }
}
