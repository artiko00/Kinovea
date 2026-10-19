//! `GraphicsPath` as the drawings build it: lines, cardinal curves, arcs and closed shapes.

use kinovea_geometry::{PointF, RectangleF};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillMode {
    #[default]
    Alternate,
    Winding,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PathSegment {
    MoveTo(PointF),
    LineTo(PointF),
    /// `AddCurve`: cardinal spline through the points.
    Curve {
        points: Vec<PointF>,
        tension: f32,
    },
    /// `AddArc`: degrees, clockwise from +x.
    Arc {
        rect: RectangleF,
        start: f32,
        sweep: f32,
    },
    /// `AddEllipse`: closed figure.
    Ellipse(RectangleF),
    /// `AddRectangle`: closed figure.
    Rectangle(RectangleF),
    /// `AddPolygon`: closed figure.
    Polygon(Vec<PointF>),
    /// `CloseFigure`.
    Close,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub fill_mode: FillMode,
}

impl Path {
    pub fn move_to(&mut self, p: PointF) {
        self.segments.push(PathSegment::MoveTo(p));
    }

    pub fn line_to(&mut self, p: PointF) {
        self.segments.push(PathSegment::LineTo(p));
    }

    pub fn curve(&mut self, points: &[PointF], tension: f32) {
        self.segments.push(PathSegment::Curve {
            points: points.to_vec(),
            tension,
        });
    }

    pub fn arc(&mut self, rect: RectangleF, start: f32, sweep: f32) {
        self.segments.push(PathSegment::Arc { rect, start, sweep });
    }

    pub fn ellipse(&mut self, rect: RectangleF) {
        self.segments.push(PathSegment::Ellipse(rect));
    }

    pub fn rectangle(&mut self, rect: RectangleF) {
        self.segments.push(PathSegment::Rectangle(rect));
    }

    pub fn polygon(&mut self, points: &[PointF]) {
        self.segments.push(PathSegment::Polygon(points.to_vec()));
    }

    pub fn close(&mut self) {
        self.segments.push(PathSegment::Close);
    }

    /// `AddPath`: appends the segments of `other`.
    pub fn add_path(&mut self, other: &Path) {
        self.segments.extend(other.segments.iter().cloned());
    }
}
