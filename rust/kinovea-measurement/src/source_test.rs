use crate::calibrator_type::CalibratorType;
use crate::source::{StaticSource, TrackedCalibrationSource};
use kinovea_geometry::{PointF, Quadrilateral};
use uuid::Uuid;

#[test]
fn static_source_reports_nothing_tracked() {
    let s = StaticSource;
    assert_eq!(s.calibration_origin(5), PointF::EMPTY);
    assert_eq!(s.calibration_quad(5, CalibratorType::Plane, Uuid::nil()), Quadrilateral::empty());
    assert!(!s.has_tracking_data(Uuid::new_v4()));
}
