use super::stroke::stroke_hit;
use kinovea_geometry::PointF;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// GDI+ Widen with flat caps: the stroke ends at the end points, perpendicular to the segment.
#[test]
fn open_end_is_flat_not_round() {
    let line = [p(0.0, 0.0), p(100.0, 0.0)];
    assert!(stroke_hit(&line, false, 5.0, p(50.0, 4.0)));
    assert!(!stroke_hit(&line, false, 5.0, p(-3.0, 0.0)));
    assert!(!stroke_hit(&line, false, 5.0, p(103.0, 0.0)));
}

// Miter join at a corner: the outer corner of a right angle is inside the miter square.
#[test]
fn miter_join_fills_the_outer_corner() {
    let elbow = [p(0.0, 0.0), p(100.0, 0.0), p(100.0, 100.0)];
    assert!(stroke_hit(&elbow, false, 5.0, p(104.0, -4.0)));
    assert!(!stroke_hit(&elbow, false, 5.0, p(106.0, -6.0)));
}

// A closed square outline is hit along its closing edge too.
#[test]
fn closed_outline_includes_the_closing_edge() {
    let square = [p(0.0, 0.0), p(100.0, 0.0), p(100.0, 100.0), p(0.0, 100.0)];
    assert!(stroke_hit(&square, true, 5.0, p(2.0, 50.0)));
    assert!(!stroke_hit(&square, true, 5.0, p(50.0, 50.0)));
}
