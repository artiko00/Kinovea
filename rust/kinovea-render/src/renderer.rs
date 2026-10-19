//! Backend-neutral drawing primitives (design D1). Coordinates are viewport pixels, y down; angles
//! are degrees clockwise from +x, as GDI+.
//!
//! GDI+ inventory of the original drawings (`Kinovea.ScreenManager/Metadata/Drawings`, `MiniLabel`,
//! `Tracking/DrawingTrack`) mapped to this trait:
//! - `DrawLine`, `DrawLines`, `DrawCurve` (tension 0.5, squiggles 0.9) → `stroke_line`, `stroke_lines`, `stroke_curve`
//! - `DrawEllipse`, `FillEllipse`, `DrawRectangle`, `FillRectangle` → `stroke_ellipse`, `fill_ellipse`, `stroke_rectangle`, `fill_rectangle`
//! - `DrawArc`, `FillPie`, `FillPolygon` → `stroke_arc`, `fill_pie`, `fill_polygon`
//! - `DrawPath`, `FillPath` over `GraphicsPath` (`AddLine/AddArc/AddEllipse/AddCurve/AddRectangle/AddPath`) → `stroke_path`, `fill_path`, `Path`
//! - `DrawString`, `MeasureString` → `draw_text`, `measure_text`; `DrawImage` with `ColorMatrix` alpha → `draw_image`
//! - `TranslateTransform`, `RotateTransform`, `ResetTransform` → `push_transform`, `pop_transform`, `reset_transform`
//! - `ClipBounds`, `SetClip` → `clip_bounds`, `push_clip`, `pop_clip`
//! - `SolidBrush`, `Pen.DashStyle`, `StartCap`/`EndCap` (Round, Square, NoAnchor), `LineJoin` → `Brush`, `Pen`
//! - Not in the trait: arrows (`ArrowHelper` fills triangles), squiggles and rounded rectangles (painter
//!   helpers over `stroke_curve`, `stroke_path`), hit testing (`Region`, `Widen`), and unused GDI+ calls
//!   (`HatchBrush`, gradients, `ArrowAnchor`, `DrawBezier`, closed-curve calls).

pub mod brush;
#[cfg(test)]
mod brush_test;
pub mod font;
#[cfg(test)]
mod font_test;
pub mod image;
#[cfg(test)]
mod image_test;
pub mod path;
#[cfg(test)]
mod path_test;
pub mod pen;
#[cfg(test)]
mod pen_test;
pub mod transform;
#[cfg(test)]
mod transform_test;

pub use brush::Brush;
pub use font::{Font, FontFamily};
pub use image::ImageRef;
pub use path::{FillMode, Path, PathSegment};
pub use pen::{dash_pattern, LineCap, LineJoin, Pen};
pub use transform::Transform;

use kinovea_geometry::{PointF, RectangleF, SizeF};

/// Immediate-mode drawing surface. Transforms and clips apply to every primitive issued after them.
pub trait Renderer {
    fn stroke_line(&mut self, a: PointF, b: PointF, pen: &Pen);
    /// Open polyline.
    fn stroke_lines(&mut self, points: &[PointF], pen: &Pen);
    /// Cardinal spline through `points`.
    fn stroke_curve(&mut self, points: &[PointF], tension: f32, pen: &Pen);
    fn stroke_rectangle(&mut self, rect: RectangleF, pen: &Pen);
    fn fill_rectangle(&mut self, rect: RectangleF, brush: &Brush);
    /// `rect` is the bounding box of the ellipse.
    fn stroke_ellipse(&mut self, rect: RectangleF, pen: &Pen);
    fn fill_ellipse(&mut self, rect: RectangleF, brush: &Brush);
    /// Arc of the ellipse inscribed in `rect`.
    fn stroke_arc(&mut self, rect: RectangleF, start: f32, sweep: f32, pen: &Pen);
    /// Pie slice of the ellipse inscribed in `rect`.
    fn fill_pie(&mut self, rect: RectangleF, start: f32, sweep: f32, brush: &Brush);
    fn fill_polygon(&mut self, points: &[PointF], brush: &Brush);
    fn stroke_path(&mut self, path: &Path, pen: &Pen);
    fn fill_path(&mut self, path: &Path, brush: &Brush);
    /// `origin` is the top-left corner of the text box.
    fn draw_text(&mut self, text: &str, font: &Font, brush: &Brush, origin: PointF);
    fn measure_text(&self, text: &str, font: &Font) -> SizeF;
    /// Blits `src` of `image` onto `dst`, with `opacity` in 0..=1 and an optional horizontal mirror.
    fn draw_image(
        &mut self,
        image: &ImageRef,
        src: RectangleF,
        dst: RectangleF,
        opacity: f32,
        mirrored: bool,
    );
    /// Applies `transform` to the points drawn from now on, before the transform in force.
    fn push_transform(&mut self, transform: Transform);
    fn pop_transform(&mut self);
    fn reset_transform(&mut self);
    /// Intersects the clip with `rect` until the matching `pop_clip`.
    fn push_clip(&mut self, rect: RectangleF);
    fn pop_clip(&mut self);
    /// Current clip in viewport pixels (`Graphics.ClipBounds`).
    fn clip_bounds(&self) -> RectangleF;
}
