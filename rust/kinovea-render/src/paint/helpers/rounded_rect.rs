//! Owner: P2. Port of `RoundedRectangle.Draw`, plus the text box built on it (`MeasureString`, the
//! rounded background with radius `Font.Height / 4`, then `DrawString`) shared by the labels.

use crate::renderer::{Brush, Font, Path, Pen, Renderer};
use crate::viewport::{Point, Rectangle};
use kinovea_geometry::{PointF, RectangleF};

/// `Font.Height` as an integer: the line height `measure_text` reports for the font.
pub fn line_height(r: &dyn Renderer, font: &Font) -> i32 {
    r.measure_text("", font).height as i32
}

/// Port of `RoundedRectangle.Draw(canvas, rect, brush, radius, dropShape, contour, pen)`: the
/// background in viewport coordinates, with the contour stroked only when `contour` is set.
pub fn draw_rounded_rectangle(
    r: &mut dyn Renderer,
    rect: RectangleF,
    brush: &Brush,
    radius: i32,
    drop_shape: bool,
    contour: bool,
    pen: Option<&Pen>,
) {
    let path = if drop_shape {
        drop_shape_path(rect, radius)
    } else {
        rounded_path(rect, radius)
    };
    r.fill_path(&path, brush);
    if let Some(pen) = pen.filter(|_| contour) {
        r.stroke_path(&path, pen);
    }
}

/// Box of `text` drawn at `origin`: the measured size, truncated to whole pixels (`(int)SizeF`).
pub fn text_box_rect(r: &dyn Renderer, text: &str, font: &Font, origin: Point) -> Rectangle {
    let size = r.measure_text(text, font);
    Rectangle::new(origin.x, origin.y, size.width as i32, size.height as i32)
}

/// Text on a rounded background whose top-left corner is `origin`
/// (`MiniLabel.Draw`, `DrawingText.Draw`, `DrawingChrono.Draw`). Returns the box in viewport pixels.
pub fn draw_text_box(
    r: &mut dyn Renderer,
    text: &str,
    font: &Font,
    back: &Brush,
    fore: &Brush,
    origin: Point,
    contour: Option<&Pen>,
) -> Rectangle {
    let rect = text_box_rect(r, text, font, origin);
    let radius = line_height(r, font) / 4;
    draw_rounded_rectangle(
        r,
        to_rect_f(rect),
        back,
        radius,
        false,
        contour.is_some(),
        contour,
    );
    r.draw_text(text, font, fore, origin.to_point_f());
    rect
}

/// Viewport rectangle as `RectangleF`, for the path helpers.
pub fn to_rect_f(rect: Rectangle) -> RectangleF {
    RectangleF::new(
        rect.x as f32,
        rect.y as f32,
        rect.width as f32,
        rect.height as f32,
    )
}

/// Margins of the original: `radius` on the sides and top, `(int)(radius * 0.7)` at the bottom.
fn margins(radius: i32) -> (f32, f32) {
    (radius as f32, (radius as f32 * 0.7) as i32 as f32)
}

/// Point of the ellipse inscribed in `rect` at `degrees` (clockwise from +x, y down). Quadrant
/// angles are snapped so that the trig rounding does not leak into the path.
fn point_on(rect: RectangleF, degrees: f32) -> PointF {
    let (sin, cos) = degrees.to_radians().sin_cos();
    let center = PointF::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0);
    PointF::new(
        center.x + rect.width / 2.0 * exact(cos),
        center.y + rect.height / 2.0 * exact(sin),
    )
}

fn exact(v: f32) -> f32 {
    (v * 1e6).round() / 1e6
}

/// `DrawRoundedRectangle`: four quarter arcs, clockwise from the top-left corner.
fn rounded_path(rect: RectangleF, radius: i32) -> Path {
    let (m, mb) = margins(radius);
    let d = radius as f32 * 2.0;
    let (l, t, r, b) = (rect.left(), rect.top(), rect.right(), rect.bottom());
    let corners = [
        (RectangleF::new(l - m, t - m, d, d), 180.0),
        (RectangleF::new(r - d + m, t - m, d, d), 270.0),
        (RectangleF::new(r - d + m, b - d + mb, d, d), 0.0),
        (RectangleF::new(l - m, b - d + mb, d, d), 90.0),
    ];
    let mut path = Path::default();
    for (i, (arc, start)) in corners.into_iter().enumerate() {
        let p = point_on(arc, start);
        if i == 0 {
            path.move_to(p);
        } else {
            path.line_to(p);
        }
        path.arc(arc, start, 90.0);
    }
    path.close();
    path
}

/// `DrawDropShape`: top-left and bottom-right corners are pointed, the other two are arcs.
fn drop_shape_path(rect: RectangleF, radius: i32) -> Path {
    let (m, mb) = margins(radius);
    let d = radius as f32 * 2.0;
    let (l, t, r, b) = (rect.left(), rect.top(), rect.right(), rect.bottom());
    let top_right = RectangleF::new(r - d + m, t - m, d, d);
    let bottom_left = RectangleF::new(l - m, b - d + mb, d, d);
    let mut path = Path::default();
    path.move_to(PointF::new(l - m, t - m));
    path.line_to(PointF::new(l - m + d, t - m));
    path.line_to(point_on(top_right, 270.0));
    path.arc(top_right, 270.0, 90.0);
    path.line_to(PointF::new(r + m, b + mb));
    path.line_to(PointF::new(r - d + m, b + mb));
    path.line_to(point_on(bottom_left, 90.0));
    path.arc(bottom_left, 90.0, 90.0);
    path.close();
    path
}
