use crate::calibration_helper_test::full_hd;
use crate::units::Units;
use kinovea_core::units::*;
use kinovea_core::units_convert::*;
use kinovea_kinematics::{MeasurementContext, PixelContext};
use kinovea_geometry::{PointF, Size, SizeF};

#[test]
fn conversions_follow_kinovea_core() {
    let mut h = full_hd();
    let mut u = Units {
        length: LengthUnit::Centimeters,
        speed: SpeedUnit::KilometersPerHour,
        acceleration: AccelerationUnit::FeetPerSecondSquared,
        angle: AngleUnit::Radian,
        angular_velocity: AngularVelocityUnit::RevolutionsPerMinute,
        angular_acceleration: AngularAccelerationUnit::RadiansPerSecondSquared,
        ..Units::default()
    };
    h.set_units(u);
    assert_eq!(h.convert_speed(250.0), convert_velocity(250.0, LengthUnit::Centimeters, SpeedUnit::KilometersPerHour));
    assert_eq!(h.convert_acceleration(30.0), convert_acceleration(30.0, LengthUnit::Centimeters, AccelerationUnit::FeetPerSecondSquared));
    let m = convert_for_length_unit(5.0, SpeedUnit::KilometersPerHour, LengthUnit::Centimeters);
    assert_eq!(h.convert_acceleration_from_velocity(5.0), convert_acceleration(m, LengthUnit::Centimeters, AccelerationUnit::FeetPerSecondSquared));
    assert_eq!(h.convert_angle(1.5), 1.5);
    assert_eq!(h.convert_angular_velocity(2.0), convert_angular_velocity(2.0, AngularVelocityUnit::RevolutionsPerMinute) as f32);
    assert_eq!(h.convert_angular_acceleration(3.0), 3.0);
    u.angle = AngleUnit::Degree;
    h.set_units(u);
    assert!((h.convert_angle(std::f32::consts::PI) - 180.0).abs() < 1e-4);
}

#[test]
fn default_calibration_behaves_like_the_pixel_context() {
    let h = full_hd();
    let ctx = h.with_preferences(true, false);
    let px = PixelContext::new(SizeF::new(1920.0, 1080.0), 100.0);
    for (x, y) in [(0.0, 0.0), (1060.0, 440.0), (1919.0, 1079.0), (333.3, 777.7)] {
        let a = ctx.point_at_time(PointF::new(x, y), 3);
        let b = px.point_at_time(PointF::new(x, y), 3);
        assert!((a.x - b.x).abs() < 1e-3 && (a.y - b.y).abs() < 1e-3, "{a:?} vs {b:?}");
    }
    assert_eq!(ctx.frames_to_seconds(2), px.frames_to_seconds(2));
    assert_eq!(ctx.capture_fps(), 100.0);
    assert_eq!(ctx.convert_speed(12.5), 12.5);
    assert_eq!(ctx.convert_acceleration(-3.0), -3.0);
    assert_eq!(ctx.convert_acceleration_from_velocity(7.0), 7.0);
    assert_eq!(ctx.convert_angle(1.0), px.convert_angle(1.0));
    assert_eq!(ctx.convert_angular_velocity(1.0), px.convert_angular_velocity(1.0));
    assert_eq!(ctx.convert_angular_acceleration(1.0), px.convert_angular_acceleration(1.0));
    assert!(ctx.filtering_enabled() && !ctx.high_speed_smoothing_enabled());
    let _ = Size::new(1, 1);
}
