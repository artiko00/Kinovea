use super::*;
use kinovea_geometry::PointF;

fn assert_point(actual: PointF, x: f32, y: f32) {
    assert!(
        (actual.x - x).abs() < 1e-4 && (actual.y - y).abs() < 1e-4,
        "{actual:?} != ({x}, {y})"
    );
}

#[test]
fn identity_keeps_points() {
    assert_point(
        Transform::identity().apply(PointF::new(3.0, -4.0)),
        3.0,
        -4.0,
    );
}

#[test]
fn translate_moves_points() {
    assert_point(
        Transform::translate(10.0, 20.0).apply(PointF::new(1.0, 2.0)),
        11.0,
        22.0,
    );
}

#[test]
fn rotate_turns_clockwise_on_a_y_down_surface() {
    // `RotateTransform(90)` maps +x onto +y.
    assert_point(
        Transform::rotate(90.0).apply(PointF::new(1.0, 0.0)),
        0.0,
        1.0,
    );
}

#[test]
fn scale_multiplies_each_axis() {
    assert_point(
        Transform::scale(2.0, 3.0).apply(PointF::new(1.0, 1.0)),
        2.0,
        3.0,
    );
}

#[test]
fn multiply_applies_the_inner_transform_first() {
    // Rotate first, then translate: (1, 0) -> (0, 1) -> (10, 21).
    let t = Transform::translate(10.0, 20.0).multiply(&Transform::rotate(90.0));
    assert_point(t.apply(PointF::new(1.0, 0.0)), 10.0, 21.0);
}
