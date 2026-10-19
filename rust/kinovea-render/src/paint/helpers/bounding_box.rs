//! Owner: P2. Port of `BoundingBox.Draw`.

use crate::renderer::{Brush, Pen, Renderer};
use kinovea_geometry::RectangleF;

/// Port of `BoundingBox.Draw(canvas, rect, pen, brush, widen)`.
pub fn draw_bounding_box(
    r: &mut dyn Renderer,
    rect: RectangleF,
    pen: &Pen,
    brush: &Brush,
    widen: i32,
) {
    let _ = (r, rect, pen, brush, widen);
}
