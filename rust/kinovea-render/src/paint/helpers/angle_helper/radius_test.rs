//! `AngleHelper` with a radius, as the posture angles use it. Origin (0, 0), legs (100, 0) and (0, 100)
//! unless a test says otherwise: a quarter turn with both legs 100 long.

use super::super::angle_helper::{AngleFlags, AngleHelper};
use super::sweep::Sweep;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::Metadata;
use std::f32::consts::PI;

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

fn flags(radius: i32, signed: bool, ccw: bool, supplementary: bool) -> AngleFlags {
    AngleFlags {
        radius,
        signed,
        ccw,
        supplementary,
    }
}

fn origin() -> PointF {
    PointF::new(0.0, 0.0)
}

#[test]
fn sweep_goes_clockwise_from_the_first_leg_to_the_second() {
    // Sweep from 0 to 90 degrees; the calibrated value is pi/2 (world y up, b maps to (0, -100)).
    // Box o.Box(20) = (-20, -20, 40, 40); only a non-positive radius places the arrow.
    let metadata = Metadata::default();
    let h = AngleHelper::new(
        origin(),
        PointF::new(100.0, 0.0),
        PointF::new(0.0, 100.0),
        flags(20, false, false, false),
        &metadata.calibration,
    );
    assert!(close(h.sweep.start, 0.0));
    assert!(close(h.sweep.sweep, 90.0));
    assert!(close(h.calibrated_angle, PI / 2.0));
    assert_eq!(
        h.sweep.bounding_box,
        RectangleF::new(-20.0, -20.0, 40.0, 40.0)
    );
    assert_eq!(h.sweep.arrow_end, PointF::EMPTY);
}

#[test]
fn signed_sweep_over_180_takes_the_short_arc_on_the_other_side() {
    // Leg b at 270 degrees: unsigned sweep 270, signed start 270 and sweep 90. World y up: b maps to
    // (0, 100), so the signed value is atan2(-10000, 0) = -pi/2.
    let metadata = Metadata::default();
    let h = AngleHelper::new(
        origin(),
        PointF::new(100.0, 0.0),
        PointF::new(0.0, -100.0),
        flags(20, true, false, false),
        &metadata.calibration,
    );
    assert!(close(h.sweep.start, 270.0));
    assert!(close(h.sweep.sweep, 90.0));
    assert!(close(h.calibrated_angle, -PI / 2.0));
}

#[test]
fn supplementary_mirrors_the_first_leg_through_the_origin() {
    // Mirror of a (100, 0) through o is (-100, 0): the legs become (0, 100) and (-100, 0).
    // Sweep from 90 degrees to 180 degrees.
    let metadata = Metadata::default();
    let h = AngleHelper::new(
        origin(),
        PointF::new(100.0, 0.0),
        PointF::new(0.0, 100.0),
        flags(20, false, false, true),
        &metadata.calibration,
    );
    assert!(close(h.sweep.start, 90.0));
    assert!(close(h.sweep.sweep, 90.0));
}

#[test]
fn negative_radius_counts_from_the_smallest_leg() {
    // Both legs are 100 long: 100 + (-20) = 80 is the radius, box (-80, -80, 160, 160).
    let metadata = Metadata::default();
    let h = AngleHelper::new(
        origin(),
        PointF::new(100.0, 0.0),
        PointF::new(0.0, 100.0),
        flags(-20, false, false, false),
        &metadata.calibration,
    );
    assert_eq!(
        h.sweep.bounding_box,
        RectangleF::new(-80.0, -80.0, 160.0, 160.0)
    );
}

#[test]
fn coincident_points_have_no_angle() {
    // A leg on the origin leaves the helper with an empty sweep; the posture painter skips such angles.
    let metadata = Metadata::default();
    let h = AngleHelper::new(
        origin(),
        origin(),
        PointF::new(0.0, 100.0),
        flags(20, false, false, false),
        &metadata.calibration,
    );
    assert_eq!(h.sweep, Sweep::empty());
}
