use crate::helpers::*;
use crate::point::PointF;
use std::f32::consts::{FRAC_PI_2, PI};

#[test]
fn distance_middle_and_point_at_distance() {
    let a = PointF::new(0.0, 0.0);
    let b = PointF::new(10.0, 0.0);
    assert_eq!(distance(a, PointF::new(3.0, 4.0)), 5.0);
    assert_eq!(middle_point(a, b), PointF::new(5.0, 0.0));
    assert_eq!(point_at_distance(a, b, 5.0), PointF::new(5.0, 0.0));
}

#[test]
fn oriented_angle_sign_is_counter_clockwise_positive() {
    let o = PointF::new(0.0, 0.0);
    let a = PointF::new(1.0, 0.0);
    let c = PointF::new(0.0, 1.0);
    assert!((angle_at(o, a, c) - FRAC_PI_2).abs() < 1e-6);
    assert!((angle_at(o, c, a) + FRAC_PI_2).abs() < 1e-6);
    assert!((angle(o, a, o, PointF::new(-1.0, 0.0)) - PI).abs() < 1e-6);
    assert_eq!(angle(o, a, o, a), 0.0);
}

#[test]
fn rotate_quarter_turn() {
    let p = rotate(PointF::new(1.0, 1.0), PointF::new(2.0, 1.0), FRAC_PI_2);
    assert!((p.x - 1.0).abs() < 1e-6 && (p.y - 2.0).abs() < 1e-6);
}

#[test]
fn mixing_scalars_and_points() {
    assert_eq!(mix(0.0, 10.0, 0.25), 2.5);
    assert_eq!(
        mix_points(PointF::new(0.0, 0.0), PointF::new(4.0, 8.0), 0.5),
        PointF::new(2.0, 4.0)
    );
}

#[test]
fn degrees_radians_and_rounding() {
    assert!((degrees(PI) - 180.0).abs() < 1e-4);
    assert!((degrees_f64(std::f64::consts::PI) - 180.0).abs() < 1e-12);
    assert!((radians(90.0) - FRAC_PI_2).abs() < 1e-6);
    assert_eq!(round(2.345, 2), 2.35);
    assert_eq!(round(2.5, 0), 2.0);
    assert_eq!(round(3.5, 0), 4.0);
    assert_eq!(round(1234.0, -2), 1200.0);
    assert_eq!(round(1250.0, -2), 1200.0);
    assert_eq!(round(1350.0, -2), 1400.0);
}
