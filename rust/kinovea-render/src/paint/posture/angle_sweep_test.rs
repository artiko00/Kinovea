//! `AngleHelper.Update` and `SweepAngle.Update` on uncalibrated pixels. Screen angles are measured
//! clockwise from +x with y down (`GetAbsoluteAngle`); the calibrated value is `GetAngle` in radians.

use super::angle_sweep::{compute, AngleFlags};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_measurement::{CalibrationHelper, Units};
use std::f32::consts::PI;

fn pixels() -> CalibrationHelper {
    CalibrationHelper::without_tracking(
        kinovea_geometry::Size::new(240, 120),
        Units::default(),
        25.0,
    )
}

fn flags(radius: i32, signed: bool, ccw: bool, supplementary: bool) -> AngleFlags {
    AngleFlags {
        radius,
        signed,
        ccw,
        supplementary,
    }
}

fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

fn origin() -> PointF {
    PointF::new(0.0, 0.0)
}

#[test]
fn sweep_goes_clockwise_from_the_first_leg_to_the_second() {
    // Leg a at 0 degrees, leg b at 90 degrees (screen, y down). Sweep 90 from 0.
    // The calibration works in world space, y up: b maps to (0, -100). Calibrated:
    // GetAngle(o, b, a) = atan2(10000, 0) = pi/2, unsigned and positive so no wrap.
    let a = PointF::new(100.0, 0.0);
    let b = PointF::new(0.0, 100.0);
    let angle = compute(origin(), a, b, flags(20, false, false, false), &pixels())
        .expect("distinct points");
    assert!(near(angle.sweep.start, 0.0));
    assert!(near(angle.sweep.sweep, 90.0));
    assert!(near(angle.calibrated, PI / 2.0));
    assert_eq!(
        angle.sweep.bounding,
        RectangleF::new(-20.0, -20.0, 40.0, 40.0)
    );
}

#[test]
fn signed_sweep_over_180_takes_the_short_arc_on_the_other_side() {
    // Leg b at 270 degrees: the unsigned sweep is 270; signed it becomes start 270, sweep 90.
    let a = PointF::new(100.0, 0.0);
    let b = PointF::new(0.0, -100.0);
    let angle =
        compute(origin(), a, b, flags(20, true, false, false), &pixels()).expect("distinct points");
    assert!(near(angle.sweep.start, 270.0));
    assert!(near(angle.sweep.sweep, 90.0));
    // World y up: b maps to (0, 100), so the signed value is atan2(-10000, 0) = -pi/2.
    assert!(near(angle.calibrated, -PI / 2.0));
}

#[test]
fn supplementary_mirrors_the_first_leg_through_the_origin() {
    // Mirror of a (100, 0) through o is (-100, 0): the legs become (0, 100) and (-100, 0).
    // Sweep from 90 degrees to 180 degrees.
    let a = PointF::new(100.0, 0.0);
    let b = PointF::new(0.0, 100.0);
    let angle =
        compute(origin(), a, b, flags(20, false, false, true), &pixels()).expect("distinct points");
    assert!(near(angle.sweep.start, 90.0));
    assert!(near(angle.sweep.sweep, 90.0));
}

#[test]
fn negative_radius_counts_from_the_smallest_leg() {
    // Both legs are 100 long: 100 + (-20) = 80 is the radius.
    let a = PointF::new(100.0, 0.0);
    let b = PointF::new(0.0, 100.0);
    let angle = compute(origin(), a, b, flags(-20, false, false, false), &pixels())
        .expect("distinct points");
    assert_eq!(
        angle.sweep.bounding,
        RectangleF::new(-80.0, -80.0, 160.0, 160.0)
    );
}

#[test]
fn coincident_points_have_no_angle() {
    let b = PointF::new(0.0, 100.0);
    assert!(compute(
        origin(),
        origin(),
        b,
        flags(20, false, false, false),
        &pixels()
    )
    .is_none());
}
