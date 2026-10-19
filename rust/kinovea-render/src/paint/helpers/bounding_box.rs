//! Owner: P2. Port of `BoundingBox.Draw`.

use crate::renderer::{Brush, Pen, Renderer};
use kinovea_geometry::RectangleF;

/// Port of `BoundingBox.Draw(canvas, rect, pen, brush, widen)`: the outline and a filled handle
/// centred on each corner, `widen` pixels in radius.
pub fn draw_bounding_box(
    r: &mut dyn Renderer,
    rect: RectangleF,
    pen: &Pen,
    brush: &Brush,
    widen: i32,
) {
    r.stroke_rectangle(rect, pen);
    let widen = widen as f32;
    let size = widen * 2.0;
    let corners = [
        (rect.left(), rect.top()),
        (rect.left(), rect.bottom()),
        (rect.right(), rect.top()),
        (rect.right(), rect.bottom()),
    ];
    for (x, y) in corners {
        r.fill_ellipse(RectangleF::new(x - widen, y - widen, size, size), brush);
    }
}
