//! Graphics path figures and their flattening (`GraphicsPath.Add*`).
use super::path::GraphicsPath;
use kinovea_geometry::{PointF, RectangleF};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// GraphicsPath.AddRectangle: closed figure with the four corners in order.
#[test]
fn rectangle_is_a_closed_figure_of_four_corners() {
    let mut path = GraphicsPath::new();
    path.add_rectangle(RectangleF::new(1.0, 2.0, 3.0, 4.0));
    let figure = &path.figures[0];
    assert!(figure.closed);
    assert_eq!(
        figure.points,
        vec![p(1.0, 2.0), p(4.0, 2.0), p(4.0, 6.0), p(1.0, 6.0)]
    );
}

// GraphicsPath.AddLines: open figure, no closing edge.
#[test]
fn lines_form_an_open_figure() {
    let mut path = GraphicsPath::new();
    path.add_lines(&[p(0.0, 0.0), p(1.0, 0.0), p(1.0, 1.0)]);
    assert!(!path.figures[0].closed);
    assert_eq!(path.figures[0].points.len(), 3);
}

// GraphicsPath.AddPolygon: closed figure.
#[test]
fn polygon_is_closed() {
    let mut path = GraphicsPath::new();
    path.add_polygon(&[p(0.0, 0.0), p(4.0, 0.0), p(0.0, 4.0)]);
    assert!(path.figures[0].closed);
}

// GraphicsPath.AddEllipse: flattened points lie on the ellipse (Bezier error well below 1e-3).
#[test]
fn ellipse_points_lie_on_the_ellipse() {
    let mut path = GraphicsPath::new();
    path.add_ellipse(RectangleF::new(0.0, 0.0, 20.0, 10.0));
    assert!(path.figures[0].closed);
    for q in &path.figures[0].points {
        let value = ((q.x - 10.0) / 10.0).powi(2) + ((q.y - 5.0) / 5.0).powi(2);
        assert!((value - 1.0).abs() < 1e-3, "{q:?} gives {value}");
    }
}

// GraphicsPath.AddArc: starts at angle 0 and ends at 90 degrees on the circle of radius 10.
#[test]
fn quarter_arc_runs_between_its_angles() {
    let mut path = GraphicsPath::new();
    path.add_arc(RectangleF::new(0.0, 0.0, 20.0, 20.0), 0.0, 90.0);
    let points = &path.figures[0].points;
    assert!((points[0].x - 20.0).abs() < 1e-4 && (points[0].y - 10.0).abs() < 1e-4);
    let last = points[points.len() - 1];
    assert!((last.x - 10.0).abs() < 1e-4 && (last.y - 20.0).abs() < 1e-4);
    for q in points {
        let radius = ((q.x - 10.0).powi(2) + (q.y - 10.0).powi(2)).sqrt();
        assert!((radius - 10.0).abs() < 0.01, "{q:?} at radius {radius}");
    }
}

// GraphicsPath.AddArc with zero sweep is a single point.
#[test]
fn zero_sweep_arc_is_a_single_point() {
    let mut path = GraphicsPath::new();
    path.add_arc(RectangleF::new(0.0, 0.0, 20.0, 20.0), 0.0, 0.0);
    assert_eq!(path.figures[0].points.len(), 1);
}

// GraphicsPath.AddCurve: the cardinal spline interpolates its points.
#[test]
fn curve_passes_through_its_points() {
    let mut path = GraphicsPath::new();
    path.add_curve(&[p(0.0, 0.0), p(10.0, 10.0), p(20.0, 0.0)], 0.5);
    let points = &path.figures[0].points;
    assert_eq!(points[0], p(0.0, 0.0));
    assert!(points.contains(&p(10.0, 10.0)));
    assert_eq!(points[points.len() - 1], p(20.0, 0.0));
}

// Degeneracy follows GDI+ bounds: all points coincide, or the path is empty (HitTester.cs:45).
#[test]
fn coincident_points_or_empty_path_are_degenerate() {
    let mut path = GraphicsPath::new();
    assert!(path.is_degenerate());
    path.add_line(p(5.0, 5.0), p(5.0, 5.0));
    assert!(path.is_degenerate());
    assert_eq!(path.first_point(), Some(p(5.0, 5.0)));
    path.add_line(p(0.0, 0.0), p(1.0, 0.0));
    assert!(!path.is_degenerate());
}
