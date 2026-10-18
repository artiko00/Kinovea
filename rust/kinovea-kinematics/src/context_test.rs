use crate::context::{MeasurementContext, PixelContext};
use kinovea_geometry::{PointF, SizeF};

fn ctx() -> PixelContext {
    PixelContext::new(SizeF::new(1920.0, 1080.0), 100.0)
}

#[test]
fn image_centre_maps_to_origin_with_y_up() {
    assert_eq!(ctx().point_at_time(PointF::new(960.0, 540.0), 0), PointF::new(0.0, 0.0));
    assert_eq!(ctx().point_at_time(PointF::new(970.0, 530.0), 5), PointF::new(10.0, 10.0));
}

#[test]
fn timing_and_identity_conversions() {
    let c = ctx();
    assert_eq!(c.frames_to_seconds(2), 0.02);
    assert_eq!(c.capture_fps(), 100.0);
    assert_eq!(c.convert_speed(3.5), 3.5);
    assert_eq!(c.convert_acceleration(-2.0), -2.0);
    assert_eq!(c.convert_acceleration_from_velocity(7.0), 7.0);
    assert!(c.filtering_enabled() && c.high_speed_smoothing_enabled());
}

#[test]
fn angles_convert_to_degrees() {
    let c = ctx();
    assert!((c.convert_angle(std::f32::consts::PI) - 180.0).abs() < 1e-4);
    assert!((c.convert_angular_velocity(std::f32::consts::PI) - 180.0).abs() < 1e-4);
    assert!((c.convert_angular_acceleration(std::f32::consts::FRAC_PI_2) - 90.0).abs() < 1e-4);
}
