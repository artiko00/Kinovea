//! Owner: P1. Port of `GraphicsExtensions.DrawSquigglyLine(s)`.

use crate::renderer::{Pen, Renderer};
use kinovea_geometry::PointF;

/// Port of `GraphicsExtensions.DrawSquigglyLine`.
pub fn draw_squiggly_line(r: &mut dyn Renderer, pen: &Pen, a: PointF, b: PointF) {
    let _ = (r, pen, a, b);
}

/// Port of `GraphicsExtensions.DrawSquigglyLines`.
pub fn draw_squiggly_lines(r: &mut dyn Renderer, pen: &Pen, points: &[PointF]) {
    let _ = (r, pen, points);
}
