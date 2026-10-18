use crate::calibration_axis::CalibrationAxis;
use crate::calibrator_plane::make_quad;
use kinovea_geometry::PointF;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

#[test]
fn horizontal_line_is_the_bottom_edge() {
    let q = make_quad(p(100.0, 500.0), p(300.0, 500.0), CalibrationAxis::LineHorizontal);
    assert_eq!(q.vertices, [p(100.0, 300.0), p(300.0, 300.0), p(300.0, 500.0), p(100.0, 500.0)]);
    assert!(q.is_axis_aligned_rectangle());
}

#[test]
fn vertical_line_is_the_left_edge() {
    let q = make_quad(p(100.0, 500.0), p(100.0, 300.0), CalibrationAxis::LineVertical);
    assert_eq!(q.vertices, [p(100.0, 300.0), p(300.0, 300.0), p(300.0, 500.0), p(100.0, 500.0)]);
}

#[test]
fn image_axes_use_only_the_length() {
    let q = make_quad(p(100.0, 500.0), p(130.0, 540.0), CalibrationAxis::ImageAxes);
    assert_eq!(q.vertices, [p(100.0, 450.0), p(150.0, 450.0), p(150.0, 500.0), p(100.0, 500.0)]);
}

#[test]
fn slanted_horizontal_line_rotates_the_square() {
    let q = make_quad(p(0.0, 0.0), p(3.0, 4.0), CalibrationAxis::LineHorizontal);
    assert_eq!(q.d(), p(0.0, 0.0));
    assert_eq!(q.c(), p(3.0, 4.0));
    assert_eq!(q.a(), p(4.0, -3.0));
    assert_eq!(q.b(), p(7.0, 1.0));
}
