use super::arrow::*;
use crate::backend::recording::{Call, RecordingRenderer};
use crate::renderer::{Brush, Pen, Transform};
use kinovea_geometry::PointF;
use kinovea_metadata::Color;

fn pen(width: f32) -> Pen {
    Pen::new(Color::rgb(10, 20, 30), width)
}

#[test]
fn arrow_head_is_a_filled_triangle_in_the_pen_colour() {
    // tip (10, 0), from (20, 0): v = (10, 0), ref = 4, base = (14, 0), top = (-2, 0), side = 0.6.
    let mut r = RecordingRenderer::new(100.0, 100.0);
    draw_arrow(
        &mut r,
        &pen(1.0),
        PointF::new(10.0, 0.0),
        PointF::new(20.0, 0.0),
    );
    assert_eq!(
        r.calls,
        vec![Call::FillPolygon(
            vec![
                PointF::new(-2.0, 0.0),
                PointF::new(14.0, -6.0),
                PointF::new(14.0, 6.0),
            ],
            Brush::solid(Color::rgb(10, 20, 30)),
            Transform::identity(),
        )]
    );
}

#[test]
fn arrow_reference_length_is_at_least_four_pixels() {
    // Width 2 is raised to 4 as `Math.Max(penWidth, 4)`, so the tip stays at -2 on x.
    let mut r = RecordingRenderer::new(100.0, 100.0);
    draw_arrow(
        &mut r,
        &pen(2.0),
        PointF::new(10.0, 0.0),
        PointF::new(20.0, 0.0),
    );
    let Call::FillPolygon(points, _, _) = &r.calls[0] else {
        panic!("expected a polygon");
    };
    assert_eq!(points[0], PointF::new(-2.0, 0.0));
}

#[test]
fn offset_moves_the_end_three_reference_lengths_inward() {
    // t = 3 * 4 / 10 = 1.2, so v * t = (12, 0).
    assert_eq!(
        arrow_offset(1.0, PointF::new(0.0, 0.0), PointF::new(10.0, 0.0)),
        PointF::new(12.0, 0.0)
    );
}

#[test]
fn offset_of_a_zero_length_segment_is_empty() {
    let p = PointF::new(3.0, 3.0);
    assert_eq!(arrow_offset(1.0, p, p), PointF::EMPTY);
}

#[test]
fn update_moves_both_arrowed_ends_and_truncates_them() {
    // Offset (12, 0). a = (1.5, 2.5) + 12 = (13.5, 2.5) -> (13, 2); b = (101.5, 2.5) - 12 = (89.5, 2.5) -> (89, 2).
    let mut a = PointF::new(1.5, 2.5);
    let mut b = PointF::new(101.5, 2.5);
    let can = update_start_end(1.0, &mut a, &mut b, true, true);
    assert!(can);
    assert_eq!((a, b), (PointF::new(13.0, 2.0), PointF::new(89.0, 2.0)));
}

#[test]
fn update_leaves_the_end_without_arrow_untouched() {
    let mut a = PointF::new(0.0, 0.0);
    let mut b = PointF::new(100.0, 0.0);
    update_start_end(1.0, &mut a, &mut b, false, true);
    assert_eq!((a, b), (PointF::new(0.0, 0.0), PointF::new(88.0, 0.0)));
}

#[test]
fn short_segment_cannot_carry_an_arrow() {
    // Length 10 is not longer than the offset 12: ends are kept as they are.
    let mut a = PointF::new(0.0, 0.0);
    let mut b = PointF::new(10.0, 0.0);
    let can = update_start_end(1.0, &mut a, &mut b, true, true);
    assert!(!can);
    assert_eq!((a, b), (PointF::new(0.0, 0.0), PointF::new(10.0, 0.0)));
}
