//! `SweepAngle.Update` with a zero radius, as the angle painter always passes it. Origin (100, 100),
//! first leg east (150, 100), second leg north (100, 50): a quarter turn.

use super::sweep::Sweep;
use kinovea_geometry::{PointF, RectangleF};

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

fn quarter_turn(signed: bool, ccw: bool) -> Sweep {
    Sweep::update(
        PointF::new(100.0, 100.0),
        PointF::new(150.0, 100.0),
        PointF::new(100.0, 50.0),
        0.0,
        signed,
        ccw,
    )
}

#[test]
fn counter_clockwise_quarter_turn_starts_at_north_and_sweeps_ninety() {
    // C#: SweepAngle.cs:113-125.
    // SweepAngle.Update: oa = angle(o, b) = 270, ob = angle(o, a) = 0, so sweep = (360 - 270) + 0 = 90.
    let s = quarter_turn(true, true);
    assert!(close(s.start, 270.0));
    assert!(close(s.sweep, 90.0));
}

#[test]
fn clockwise_quarter_turn_flips_to_the_same_short_arc() {
    // C#: SweepAngle.cs:117-133.
    // ccw = false: oa = 0, ob = 270, sweep = 270 > 180 and signed, so start = ob and sweep = 360 - 270.
    let s = quarter_turn(true, false);
    assert!(close(s.start, 270.0));
    assert!(close(s.sweep, 90.0));
}

#[test]
fn signed_reflex_angle_flips_to_the_short_arc() {
    // C#: SweepAngle.cs:128-133.
    // Second leg west (50, 100): oa = 270 (north), ob = 180 (west), sweep = 270 > 180.
    let s = Sweep::update(
        PointF::new(100.0, 100.0),
        PointF::new(50.0, 100.0),
        PointF::new(100.0, 50.0),
        0.0,
        true,
        true,
    );
    assert!(close(s.start, 180.0));
    assert!(close(s.sweep, 90.0));
}

#[test]
fn unsigned_reflex_angle_keeps_the_long_arc() {
    // C#: SweepAngle.cs:128 (guard "signed &&").
    let s = Sweep::update(
        PointF::new(100.0, 100.0),
        PointF::new(50.0, 100.0),
        PointF::new(100.0, 50.0),
        0.0,
        false,
        true,
    );
    assert!(close(s.start, 270.0));
    assert!(close(s.sweep, 270.0));
}

#[test]
fn bounding_box_is_the_circle_of_radius_forty() {
    // C#: SweepAngle.cs:170-178, 209.
    // Both legs are 50 long: radius = 50 - 10 = 40, box = o.Box(40).
    let s = quarter_turn(true, true);
    assert_eq!(s.bounding_box, RectangleF::new(60.0, 60.0, 80.0, 80.0));
}

#[test]
fn arrow_end_sits_on_the_measure_leg_at_the_radius() {
    // C#: SweepAngle.cs:181.
    // Unit vector o -> b is (0, -1); times radius 40 from o (100, 100).
    let s = quarter_turn(true, true);
    assert_eq!(s.arrow_end, PointF::new(100.0, 60.0));
}

#[test]
fn arrow_start_is_one_hundred_units_along_the_tangent_a_degree_away() {
    // C#: SweepAngle.cs:196-205.
    // Computed by hand: start = (199.9962, 60.8727), 100 units from the arrow end.
    let s = quarter_turn(true, true);
    assert!(close(s.arrow_start.x, 199.996_2));
    assert!(close(s.arrow_start.y, 60.872_7));
}
