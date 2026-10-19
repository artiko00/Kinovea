//! Owner: P1. Port of `ArrowHelper.Draw`.

use crate::renderer::{Pen, Renderer};
use kinovea_geometry::PointF;

/// Port of `ArrowHelper.Draw(canvas, pen, tip, from)`: arrow head at `tip` pointing away from `from`.
pub fn draw_arrow(r: &mut dyn Renderer, pen: &Pen, tip: PointF, from: PointF) {
    let _ = (r, pen, tip, from);
}
