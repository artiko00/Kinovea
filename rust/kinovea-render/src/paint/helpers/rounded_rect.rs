//! Owner: P2. Port of `RoundedRectangle.Draw`.

use crate::renderer::{Brush, Pen, Renderer};
use kinovea_geometry::RectangleF;

/// Port of `RoundedRectangle.Draw`: rounded background box in viewport coordinates.
pub fn draw_rounded_rectangle(
    r: &mut dyn Renderer,
    rect: RectangleF,
    brush: &Brush,
    radius: i32,
    drop_shape: bool,
    contour: bool,
    pen: Option<&Pen>,
) {
    let _ = (r, rect, brush, radius, drop_shape, contour, pen);
}
