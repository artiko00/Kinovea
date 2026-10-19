//! One recorded `Renderer` call. Fields follow the argument order of the trait; drawing calls end
//! with the transform that was in force when they were issued.

use crate::renderer::{Brush, Font, Path, Pen, Transform};
use kinovea_geometry::{PointF, RectangleF};

#[derive(Debug, Clone, PartialEq)]
pub enum Call {
    StrokeLine(PointF, PointF, Pen, Transform),
    StrokeLines(Vec<PointF>, Pen, Transform),
    StrokeCurve(Vec<PointF>, f32, Pen, Transform),
    StrokeRectangle(RectangleF, Pen, Transform),
    FillRectangle(RectangleF, Brush, Transform),
    StrokeEllipse(RectangleF, Pen, Transform),
    FillEllipse(RectangleF, Brush, Transform),
    StrokeArc(RectangleF, f32, f32, Pen, Transform),
    FillPie(RectangleF, f32, f32, Brush, Transform),
    FillPolygon(Vec<PointF>, Brush, Transform),
    StrokePath(Path, Pen, Transform),
    FillPath(Path, Brush, Transform),
    /// Text, font, brush, origin.
    DrawText(String, Font, Brush, PointF, Transform),
    /// Source width and height in pixels (not the pixels), source rectangle, destination rectangle,
    /// opacity, mirrored.
    DrawImage(u32, u32, RectangleF, RectangleF, f32, bool, Transform),
    PushTransform(Transform),
    PopTransform,
    ResetTransform,
    PushClip(RectangleF),
    PopClip,
}
