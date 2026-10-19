use super::helper::{text_distance, AngleHelper};
use kinovea_geometry::{PointF, RectangleF, SizeF};
use kinovea_metadata::Metadata;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

fn o() -> PointF {
    PointF::new(100.0, 100.0)
}

/// Second leg north (100, 50), first leg east (150, 100).
fn helper(signed: bool, ccw: bool, supplementary: bool) -> AngleHelper {
    let metadata = Metadata::default();
    AngleHelper::new(
        o(),
        PointF::new(150.0, 100.0),
        PointF::new(100.0, 50.0),
        signed,
        ccw,
        supplementary,
        &metadata.calibration,
    )
}

#[test]
fn counter_clockwise_quarter_turn_has_a_positive_value() {
    // C#: AngleHelper.cs:122-135; GeometryHelper.cs:91; CalibrationHelper.cs:188-195; CalibratorPlane.cs:331.
    // The uncalibrated world is y-up: `CalibrationHelper.Reset` maps y through `GridToWorld`, which
    // negates it. So the pixel-space -90 degrees of the legs is +90 degrees in world space.
    assert!(close(helper(true, true, false).calibrated_angle, FRAC_PI_2));
}

#[test]
fn clockwise_quarter_turn_has_a_negative_value() {
    // C#: AngleHelper.cs:135.
    assert!(close(
        helper(true, false, false).calibrated_angle,
        -FRAC_PI_2
    ));
}

#[test]
fn unsigned_negative_value_wraps_into_a_full_turn() {
    // C#: AngleHelper.cs:137-138.
    // -90 degrees + 360 degrees = 270 degrees.
    assert!(close(
        helper(false, false, false).calibrated_angle,
        3.0 * PI / 2.0
    ));
}

#[test]
fn supplementary_angle_uses_the_mirrored_second_leg() {
    // C#: AngleHelper.cs:74-83.
    // Legs east (150, 100) and south-west (50, 50): -135 degrees in pixel space, +135 in world space.
    // Supplementary legs (50, 50) and (50, 100) give -45 pixel degrees, +45 world degrees.
    let metadata = Metadata::default();
    let b = PointF::new(50.0, 50.0);
    let plain = AngleHelper::new(
        o(),
        PointF::new(150.0, 100.0),
        b,
        true,
        true,
        false,
        &metadata.calibration,
    );
    let supplement = AngleHelper::new(
        o(),
        PointF::new(150.0, 100.0),
        b,
        true,
        true,
        true,
        &metadata.calibration,
    );
    assert!(close(plain.calibrated_angle, 3.0 * FRAC_PI_4));
    assert!(close(supplement.calibrated_angle, FRAC_PI_4));
}

#[test]
fn leg_coinciding_with_the_origin_leaves_the_sweep_empty() {
    // C#: AngleHelper.cs:71-72.
    let metadata = Metadata::default();
    let h = AngleHelper::new(
        o(),
        o(),
        PointF::new(100.0, 50.0),
        true,
        true,
        false,
        &metadata.calibration,
    );
    assert_eq!(h.calibrated_angle, 0.0);
    assert_eq!(h.sweep.bounding_box, RectangleF::EMPTY);
}

#[test]
fn text_is_centred_on_the_bisector_at_the_text_distance() {
    // C#: AngleHelper.cs:147-158.
    // Bisector of start 270 and sweep 90 is 315 degrees: (40 cos, 40 sin) minus half the label.
    // Label of 44 x 19.2 gives (28.2843 - 22, -28.2843 - 9.6).
    let p = helper(true, true, false).text_position(40, SizeF::new(44.0, 19.2));
    assert!(close(p.x, 6.284_271));
    assert!(close(p.y, -37.884_27));
}

#[test]
fn text_distance_scales_with_the_font_size() {
    // C#: AngleHelper.cs:161-163.
    assert_eq!(text_distance(12), 40);
    assert_eq!(text_distance(24), 80);
    assert_eq!(text_distance(6), 20);
}
