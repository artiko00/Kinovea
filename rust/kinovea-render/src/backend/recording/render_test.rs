use crate::backend::recording::{Call, RecordingRenderer};
use crate::renderer::{Brush, Font, FontFamily, ImageRef, Path, Pen, Renderer, Transform};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::Color;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn font(em_px: f32) -> Font {
    Font {
        family: FontFamily::Arial,
        em_px,
        bold: true,
    }
}

#[test]
fn stroke_line_records_its_arguments_with_the_identity_transform() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    let pen = Pen::new(Color::rgb(1, 2, 3), 2.0);
    r.stroke_line(p(0.0, 0.0), p(3.0, 4.0), &pen);
    assert_eq!(
        r.calls,
        vec![Call::StrokeLine(
            p(0.0, 0.0),
            p(3.0, 4.0),
            pen,
            Transform::identity()
        )]
    );
}

#[test]
fn drawing_calls_carry_the_transform_in_force() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    let shift = Transform::translate(5.0, 5.0);
    let brush = Brush::solid(Color::BLACK);
    let rect = RectangleF::new(0.0, 0.0, 1.0, 1.0);
    r.push_transform(shift);
    r.fill_rectangle(rect, &brush);
    r.pop_transform();
    r.fill_rectangle(rect, &brush);
    assert_eq!(
        r.calls,
        vec![
            Call::PushTransform(shift),
            Call::FillRectangle(rect, brush, shift),
            Call::PopTransform,
            Call::FillRectangle(rect, brush, Transform::identity()),
        ]
    );
}

#[test]
fn measure_text_is_deterministic_and_font_free() {
    let r = RecordingRenderer::new(100.0, 100.0);
    let size = r.measure_text("abcd", &font(10.0));
    assert!((size.width - 22.0).abs() < 1e-4 && (size.height - 12.0).abs() < 1e-4);
    let empty = r.measure_text("", &font(10.0));
    assert!(empty.width == 0.0 && (empty.height - 12.0).abs() < 1e-4);
}

#[test]
fn draw_text_records_text_font_brush_and_origin() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    let brush = Brush::solid(Color::BLACK);
    r.draw_text("12", &font(16.0), &brush, p(4.0, 5.0));
    assert_eq!(
        r.calls,
        vec![Call::DrawText(
            "12".to_owned(),
            font(16.0),
            brush,
            p(4.0, 5.0),
            Transform::identity()
        )]
    );
}

#[test]
fn draw_image_records_sizes_and_rectangles_not_pixels() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    let pixels = [0u8; 16];
    let image = ImageRef {
        width: 2,
        height: 2,
        rgba: &pixels,
    };
    let src = RectangleF::new(0.0, 0.0, 2.0, 2.0);
    let dst = RectangleF::new(10.0, 10.0, 4.0, 4.0);
    r.draw_image(&image, src, dst, 0.5, true);
    assert_eq!(
        r.calls,
        vec![Call::DrawImage(
            2,
            2,
            src,
            dst,
            0.5,
            true,
            Transform::identity()
        )]
    );
}

#[test]
fn every_drawing_primitive_records_exactly_one_call() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    let pen = Pen::new(Color::BLACK, 1.0);
    let brush = Brush::solid(Color::BLACK);
    let rect = RectangleF::new(0.0, 0.0, 9.0, 9.0);
    let pts = [p(0.0, 0.0), p(1.0, 1.0)];
    let path = Path::default();
    let pixels = [0u8; 4];
    let image = ImageRef {
        width: 1,
        height: 1,
        rgba: &pixels,
    };
    r.stroke_line(pts[0], pts[1], &pen);
    r.stroke_lines(&pts, &pen);
    r.stroke_curve(&pts, 0.5, &pen);
    r.stroke_rectangle(rect, &pen);
    r.fill_rectangle(rect, &brush);
    r.stroke_ellipse(rect, &pen);
    r.fill_ellipse(rect, &brush);
    r.stroke_arc(rect, 0.0, 90.0, &pen);
    r.fill_pie(rect, 0.0, 90.0, &brush);
    r.fill_polygon(&pts, &brush);
    r.stroke_path(&path, &pen);
    r.fill_path(&path, &brush);
    r.draw_text("x", &font(8.0), &brush, pts[0]);
    r.draw_image(&image, rect, rect, 1.0, false);
    assert_eq!(r.calls.len(), 14);
}

#[test]
fn a_recorder_can_be_driven_through_the_trait_object() {
    fn paint(target: &mut dyn Renderer) {
        target.fill_ellipse(RectangleF::EMPTY, &Brush::solid(Color::BLACK));
    }
    let mut r = RecordingRenderer::new(10.0, 10.0);
    paint(&mut r);
    assert_eq!(r.calls.len(), 1);
}
