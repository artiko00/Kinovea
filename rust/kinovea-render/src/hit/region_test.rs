//! Distance to a path and even-odd containment over its figures.
use super::path::GraphicsPath;
use kinovea_geometry::{PointF, RectangleF};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

#[test]
fn distance_is_to_the_nearest_segment() {
    let mut path = GraphicsPath::new();
    path.add_lines(&[p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0)]);
    assert!((path.distance_to(p(12.0, 5.0)) - 2.0).abs() < 1e-5);
    assert!((path.distance_to(p(5.0, -3.0)) - 3.0).abs() < 1e-5);
}

// A closed figure includes its closing edge, the left side (0,10)-(0,0).
#[test]
fn closed_figure_includes_its_closing_edge() {
    let corners = [p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)];
    let mut closed = GraphicsPath::new();
    closed.add_polygon(&corners);
    let mut open = GraphicsPath::new();
    open.add_lines(&corners);
    assert!((closed.distance_to(p(-2.0, 5.0)) - 2.0).abs() < 1e-5);
    assert!(open.distance_to(p(-2.0, 5.0)) > 5.0);
}

#[test]
fn single_point_figure_measures_to_the_point() {
    let mut path = GraphicsPath::new();
    path.add_lines(&[p(3.0, 4.0)]);
    assert!((path.distance_to(p(0.0, 0.0)) - 5.0).abs() < 1e-5);
}

#[test]
fn contains_uses_even_odd_crossings() {
    let mut path = GraphicsPath::new();
    path.add_rectangle(RectangleF::new(0.0, 0.0, 10.0, 10.0));
    assert!(path.contains_even_odd(p(5.0, 5.0)));
    assert!(!path.contains_even_odd(p(15.0, 5.0)));
}
