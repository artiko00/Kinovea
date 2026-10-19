//! `Renderer` for the recording backend: each primitive becomes a `Call` carrying its arguments.

use super::{Call, RecordingRenderer};
use crate::renderer::{Brush, Font, ImageRef, Path, Pen, Renderer, Transform};
use kinovea_geometry::{PointF, RectangleF, SizeF};

/// Advance per character, in em, of the fixed text metric (no font is loaded).
const CHAR_ADVANCE_EM: f32 = 0.55;
/// Line height, in em, of the fixed text metric.
const LINE_HEIGHT_EM: f32 = 1.2;

impl Renderer for RecordingRenderer {
    fn stroke_line(&mut self, a: PointF, b: PointF, pen: &Pen) {
        self.draw(|t| Call::StrokeLine(a, b, pen.clone(), t));
    }

    fn stroke_lines(&mut self, points: &[PointF], pen: &Pen) {
        self.draw(|t| Call::StrokeLines(points.to_vec(), pen.clone(), t));
    }

    fn stroke_curve(&mut self, points: &[PointF], tension: f32, pen: &Pen) {
        self.draw(|t| Call::StrokeCurve(points.to_vec(), tension, pen.clone(), t));
    }

    fn stroke_rectangle(&mut self, rect: RectangleF, pen: &Pen) {
        self.draw(|t| Call::StrokeRectangle(rect, pen.clone(), t));
    }

    fn fill_rectangle(&mut self, rect: RectangleF, brush: &Brush) {
        self.draw(|t| Call::FillRectangle(rect, *brush, t));
    }

    fn stroke_ellipse(&mut self, rect: RectangleF, pen: &Pen) {
        self.draw(|t| Call::StrokeEllipse(rect, pen.clone(), t));
    }

    fn fill_ellipse(&mut self, rect: RectangleF, brush: &Brush) {
        self.draw(|t| Call::FillEllipse(rect, *brush, t));
    }

    fn stroke_arc(&mut self, rect: RectangleF, start: f32, sweep: f32, pen: &Pen) {
        self.draw(|t| Call::StrokeArc(rect, start, sweep, pen.clone(), t));
    }

    fn fill_pie(&mut self, rect: RectangleF, start: f32, sweep: f32, brush: &Brush) {
        self.draw(|t| Call::FillPie(rect, start, sweep, *brush, t));
    }

    fn fill_polygon(&mut self, points: &[PointF], brush: &Brush) {
        self.draw(|t| Call::FillPolygon(points.to_vec(), *brush, t));
    }

    fn stroke_path(&mut self, path: &Path, pen: &Pen) {
        self.draw(|t| Call::StrokePath(path.clone(), pen.clone(), t));
    }

    fn fill_path(&mut self, path: &Path, brush: &Brush) {
        self.draw(|t| Call::FillPath(path.clone(), *brush, t));
    }

    fn draw_text(&mut self, text: &str, font: &Font, brush: &Brush, origin: PointF) {
        self.draw(|t| Call::DrawText(text.to_owned(), *font, *brush, origin, t));
    }

    fn measure_text(&self, text: &str, font: &Font) -> SizeF {
        let width = text.chars().count() as f32 * CHAR_ADVANCE_EM * font.em_px;
        SizeF::new(width, LINE_HEIGHT_EM * font.em_px)
    }

    fn draw_image(
        &mut self,
        image: &ImageRef,
        src: RectangleF,
        dst: RectangleF,
        opacity: f32,
        mirrored: bool,
    ) {
        self.draw(|t| Call::DrawImage(image.width, image.height, src, dst, opacity, mirrored, t));
    }

    fn push_transform(&mut self, transform: Transform) {
        self.enter_transform(transform);
    }

    fn pop_transform(&mut self) {
        self.leave_transform();
    }

    fn reset_transform(&mut self) {
        self.reset_current_transform();
    }

    fn push_clip(&mut self, rect: RectangleF) {
        self.enter_clip(rect);
    }

    fn pop_clip(&mut self) {
        self.leave_clip();
    }

    fn clip_bounds(&self) -> RectangleF {
        self.current_clip()
    }
}
