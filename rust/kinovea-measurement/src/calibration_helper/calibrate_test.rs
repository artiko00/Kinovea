use crate::calibration_axis::CalibrationAxis;
use crate::calibration_helper_test::full_hd;
use kinovea_geometry::PointF;
use uuid::Uuid;

#[test]
fn line_calibration_through_distortion_free_helper_moves_the_origin() {
    let mut h = full_hd();
    h.calibrate_by_line(Uuid::nil(), 1.0, PointF::new(0.0, 0.0), PointF::new(100.0, 0.0), CalibrationAxis::LineHorizontal);
    let o = h.origin();
    assert!((o.x).abs() < 1e-3 && (o.y).abs() < 1e-3, "{o:?}");
    h.set_origin(PointF::new(50.0, 0.0));
    let w = h.point(PointF::new(100.0, 0.0));
    assert!((w.x - 0.5).abs() < 1e-4, "{w:?}");
}
