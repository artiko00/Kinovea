use super::*;
use kinovea_geometry::{PointF, RectangleF};

#[test]
fn default_path_is_empty_with_alternate_fill() {
    let path = Path::default();
    assert!(path.segments.is_empty());
    assert_eq!(path.fill_mode, FillMode::Alternate);
}

#[test]
fn builder_records_segments_in_order() {
    let a = PointF::new(0.0, 0.0);
    let b = PointF::new(1.0, 0.0);
    let rect = RectangleF::new(0.0, 0.0, 4.0, 4.0);
    let mut path = Path::default();
    path.move_to(a);
    path.line_to(b);
    path.curve(&[a, b], 0.5);
    path.arc(rect, 0.0, 90.0);
    path.ellipse(rect);
    path.rectangle(rect);
    path.polygon(&[a, b]);
    path.close();
    assert_eq!(
        path.segments,
        vec![
            PathSegment::MoveTo(a),
            PathSegment::LineTo(b),
            PathSegment::Curve {
                points: vec![a, b],
                tension: 0.5
            },
            PathSegment::Arc {
                rect,
                start: 0.0,
                sweep: 90.0
            },
            PathSegment::Ellipse(rect),
            PathSegment::Rectangle(rect),
            PathSegment::Polygon(vec![a, b]),
            PathSegment::Close,
        ]
    );
}

#[test]
fn add_path_appends_the_other_segments() {
    let a = PointF::new(0.0, 0.0);
    let mut inner = Path::default();
    inner.line_to(a);
    let mut outer = Path::default();
    outer.move_to(a);
    outer.add_path(&inner);
    assert_eq!(
        outer.segments,
        vec![PathSegment::MoveTo(a), PathSegment::LineTo(a)]
    );
}
