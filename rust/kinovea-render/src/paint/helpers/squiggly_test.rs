use super::squiggly::*;
use crate::backend::recording::{Call, RecordingRenderer};
use crate::renderer::{Pen, Transform};
use kinovea_geometry::PointF;
use kinovea_metadata::Color;

fn pen(width: f32) -> Pen {
    Pen::new(Color::rgb(1, 2, 3), width)
}

#[test]
fn squiggle_zigzags_along_the_segment_with_tension_point_nine() {
    // Width 1 -> ref 4, half period 6, step (6, 0), up (0, -3), down (0, 3); norm 20 -> steps 3.33,
    // so one zigzag point: (12, -3). Then (15, 0) and b.
    let mut r = RecordingRenderer::new(100.0, 100.0);
    draw_squiggly_line(
        &mut r,
        &pen(1.0),
        PointF::new(0.0, 0.0),
        PointF::new(20.0, 0.0),
    );
    assert_eq!(
        r.calls,
        vec![Call::StrokeCurve(
            vec![
                PointF::new(0.0, 0.0),
                PointF::new(6.0, 0.0),
                PointF::new(12.0, -3.0),
                PointF::new(15.0, 0.0),
                PointF::new(20.0, 0.0),
            ],
            0.9,
            pen(1.0),
            Transform::identity(),
        )]
    );
}

#[test]
fn squiggles_skip_degenerate_segments_of_a_polyline() {
    // The repeated point adds nothing; the polyline keeps its first and last points.
    let mut r = RecordingRenderer::new(100.0, 100.0);
    let points = [
        PointF::new(0.0, 0.0),
        PointF::new(0.0, 0.0),
        PointF::new(20.0, 0.0),
    ];
    draw_squiggly_lines(&mut r, &pen(1.0), &points);
    let Call::StrokeCurve(curve, _, _, _) = &r.calls[0] else {
        panic!("expected a curve");
    };
    assert_eq!(curve.first(), Some(&PointF::new(0.0, 0.0)));
    assert_eq!(curve.last(), Some(&PointF::new(20.0, 0.0)));
    assert_eq!(curve.len(), 6);
}

#[test]
fn squiggly_polyline_with_one_point_paints_nothing() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    draw_squiggly_lines(&mut r, &pen(1.0), &[PointF::new(1.0, 1.0)]);
    assert!(r.calls.is_empty());
}
