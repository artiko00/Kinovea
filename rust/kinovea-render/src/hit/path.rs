//! Graphics path figures, flattened to polylines at construction (`GraphicsPath.Add*`).
use super::bezier::{arc_to_bezier, cardinal_to_bezier, ellipse_point, flatten};
use kinovea_geometry::{PointF, RectangleF};

/// One subpath: its flattened points and whether it closes back on its first point.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Figure {
    pub(super) points: Vec<PointF>,
    pub(super) closed: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphicsPath {
    pub(super) figures: Vec<Figure>,
}

impl GraphicsPath {
    pub fn new() -> Self {
        GraphicsPath::default()
    }

    /// `AddLine`: an open figure with two points.
    pub fn add_line(&mut self, a: PointF, b: PointF) {
        self.push(vec![a, b], false);
    }

    /// `AddLines`: an open polyline.
    pub fn add_lines(&mut self, points: &[PointF]) {
        self.push(points.to_vec(), false);
    }

    /// `AddPolygon`: a closed polyline.
    pub fn add_polygon(&mut self, points: &[PointF]) {
        self.push(points.to_vec(), true);
    }

    /// `AddRectangle`: a closed figure through the four corners.
    pub fn add_rectangle(&mut self, r: RectangleF) {
        let corners = vec![
            PointF::new(r.left(), r.top()),
            PointF::new(r.right(), r.top()),
            PointF::new(r.right(), r.bottom()),
            PointF::new(r.left(), r.bottom()),
        ];
        self.push(corners, true);
    }

    /// `AddCurve` with a cardinal spline of the given tension, converted to cubic Beziers.
    pub fn add_curve(&mut self, points: &[PointF], tension: f32) {
        let Some(&start) = points.first() else {
            return;
        };
        self.push(flatten(start, &cardinal_to_bezier(points, tension)), false);
    }

    /// `AddArc`: an open figure sweeping `sweep` degrees from `start` degrees.
    pub fn add_arc(&mut self, r: RectangleF, start: f32, sweep: f32) {
        self.push(arc_figure(r, start, sweep), false);
    }

    /// `AddEllipse`: a closed figure around the rectangle.
    pub fn add_ellipse(&mut self, r: RectangleF) {
        self.push(arc_figure(r, 0.0, 360.0), true);
    }

    /// First recorded point, `None` for an empty path.
    pub fn first_point(&self) -> Option<PointF> {
        self.figures.first().and_then(|f| f.points.first().copied())
    }

    /// True when every point coincides, which GDI+ reports as zero bounds (`HitTester.cs:45`).
    pub fn is_degenerate(&self) -> bool {
        let Some(first) = self.first_point() else {
            return true;
        };
        self.figures
            .iter()
            .flat_map(|f| f.points.iter())
            .all(|q| *q == first)
    }

    fn push(&mut self, points: Vec<PointF>, closed: bool) {
        if !points.is_empty() {
            self.figures.push(Figure { points, closed });
        }
    }
}

fn arc_figure(r: RectangleF, start: f32, sweep: f32) -> Vec<PointF> {
    let start_point = ellipse_point(r, start);
    flatten(start_point, &arc_to_bezier(r, start, sweep))
}
