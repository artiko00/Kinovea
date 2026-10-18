use crate::helpers::{closest_point, intersection_rectangle_center};
use crate::point::PointF;
use crate::rect::RectangleF;
use crate::shapes::PointLinePosition;

const A: PointF = PointF::new(0.0, 0.0);
const B: PointF = PointF::new(10.0, 0.0);

#[test]
fn closest_point_respects_each_allowed_position() {
    let beyond = PointF::new(15.0, 3.0);
    let before = PointF::new(-5.0, 3.0);
    assert_eq!(
        closest_point(A, B, beyond, PointLinePosition::OnSegment, 0),
        B
    );
    assert_eq!(
        closest_point(A, B, before, PointLinePosition::OnSegment, 0),
        A
    );
    assert_eq!(
        closest_point(A, B, beyond, PointLinePosition::Anywhere, 0),
        PointF::new(15.0, 0.0)
    );
    assert_eq!(
        closest_point(A, B, beyond, PointLinePosition::BeforeSegment, 0),
        A
    );
    assert_eq!(
        closest_point(A, B, beyond, PointLinePosition::BeforeAndOnSegment, 0),
        B
    );
    assert_eq!(
        closest_point(A, B, before, PointLinePosition::AfterSegment, 0),
        B
    );
    assert_eq!(
        closest_point(A, B, before, PointLinePosition::AfterAndOnSegment, 0),
        A
    );
    assert_eq!(
        closest_point(A, B, beyond, PointLinePosition::AfterAndOnSegment, 0),
        PointF::new(15.0, 0.0)
    );
}

#[test]
fn margin_keeps_the_point_away_from_the_ends() {
    assert_eq!(
        closest_point(
            A,
            B,
            PointF::new(15.0, 0.0),
            PointLinePosition::OnSegment,
            2
        ),
        PointF::new(8.0, 0.0)
    );
    assert_eq!(
        closest_point(
            A,
            B,
            PointF::new(-5.0, 0.0),
            PointLinePosition::OnSegment,
            2
        ),
        PointF::new(2.0, 0.0)
    );
    assert_eq!(
        closest_point(
            A,
            B,
            PointF::new(5.0, 0.0),
            PointLinePosition::BeforeSegment,
            2
        ),
        PointF::new(-2.0, 0.0)
    );
    assert_eq!(
        closest_point(
            A,
            B,
            PointF::new(5.0, 0.0),
            PointLinePosition::AfterSegment,
            2
        ),
        PointF::new(12.0, 0.0)
    );
}

#[test]
fn intersection_hits_the_border_facing_the_end_point() {
    let rect = RectangleF::new(0.0, 0.0, 100.0, 50.0);
    assert_eq!(
        intersection_rectangle_center(rect, PointF::new(200.0, 25.0)),
        PointF::new(100.0, 25.0)
    );
    assert_eq!(
        intersection_rectangle_center(rect, PointF::new(-100.0, 25.0)),
        PointF::new(0.0, 25.0)
    );
    assert_eq!(
        intersection_rectangle_center(rect, PointF::new(50.0, -100.0)),
        PointF::new(50.0, 0.0)
    );
    assert_eq!(
        intersection_rectangle_center(rect, PointF::new(50.0, 100.0)),
        PointF::new(50.0, 50.0)
    );
    let diag = intersection_rectangle_center(rect, PointF::new(150.0, 60.0));
    assert_eq!(diag, PointF::new(100.0, 42.5));
    let corner = intersection_rectangle_center(rect, PointF::new(150.0, 75.0));
    assert_eq!(corner, PointF::new(150.0, 75.0));
}

#[test]
fn intersection_returns_end_when_nothing_matches() {
    let rect = RectangleF::new(0.0, 0.0, 100.0, 50.0);
    assert_eq!(
        intersection_rectangle_center(rect, PointF::new(50.0, 25.0)),
        PointF::new(50.0, 25.0)
    );
}
