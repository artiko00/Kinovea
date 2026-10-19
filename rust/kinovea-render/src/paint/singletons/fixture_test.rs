//! Calibrations shared by the coordinate system tests. The flat plane is derived in `grid_finder_test`.

use kinovea_geometry::{PointF, Quadrilateral, Size, SizeF};
use kinovea_measurement::{CalibrationHelper, CalibratorType, Units};
use uuid::Uuid;

/// Plane of 200 by 100 with its quad at (20, 10) in a 240 by 120 image.
pub fn flat_plane() -> CalibrationHelper {
    calibrated(quad([
        (20.0, 10.0),
        (220.0, 10.0),
        (220.0, 110.0),
        (20.0, 110.0),
    ]))
}

/// Same plane with the quad crossed over: not convex, so the plane is not valid.
pub fn crossed_plane() -> CalibrationHelper {
    calibrated(quad([
        (20.0, 10.0),
        (220.0, 110.0),
        (220.0, 10.0),
        (20.0, 110.0),
    ]))
}

fn quad(points: [(f32, f32); 4]) -> Quadrilateral {
    let p = points.map(|(x, y)| PointF::new(x, y));
    Quadrilateral::new(p[0], p[1], p[2], p[3])
}

/// The calibration dialogs select the plane type before calibrating (`SetCalibratorFromType`).
fn calibrated(quad: Quadrilateral) -> CalibrationHelper {
    let mut calibration =
        CalibrationHelper::without_tracking(Size::new(240, 120), Units::default(), 25.0);
    calibration.set_calibrator_type(CalibratorType::Plane);
    calibration.calibrate_by_plane(Uuid::nil(), SizeF::new(200.0, 100.0), &quad);
    calibration
}

#[test]
fn flat_plane_has_the_plane_size_of_the_fixture() {
    assert_eq!(flat_plane().calibrator().size, SizeF::new(200.0, 100.0));
}
