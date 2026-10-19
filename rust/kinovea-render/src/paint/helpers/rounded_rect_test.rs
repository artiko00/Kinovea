use crate::backend::recording::Call;
use crate::paint::helpers::rounded_rect::{draw_rounded_rectangle, draw_text_box, line_height};
use crate::paint::helpers::support_test::surface;
use crate::renderer::{Brush, Font, FontFamily, Path, Pen, Transform};
use crate::viewport::{Point, Rectangle};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::Color;

#[test]
fn rounded_rect_helper_is_declared() {
    assert!(module_path!().contains("rounded_rect"));
}

fn pt(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn rect(x: f32, y: f32, w: f32, h: f32) -> RectangleF {
    RectangleF::new(x, y, w, h)
}

/// `DrawDropShape` for rect (10, 20, 100, 30) and radius 4: margins 4 and 2 (bottom), diameter 8.
fn drop_path() -> Path {
    let mut p = Path::default();
    p.move_to(pt(6.0, 16.0));
    p.line_to(pt(14.0, 16.0));
    p.line_to(pt(110.0, 16.0));
    p.arc(rect(106.0, 16.0, 8.0, 8.0), 270.0, 90.0);
    p.line_to(pt(114.0, 52.0));
    p.line_to(pt(106.0, 52.0));
    p.line_to(pt(10.0, 52.0));
    p.arc(rect(6.0, 44.0, 8.0, 8.0), 90.0, 90.0);
    p.close();
    p
}

/// `DrawRoundedRectangle` for rect (10, 20, 100, 30) and radius 4.
fn rounded_path() -> Path {
    let mut p = Path::default();
    p.move_to(pt(6.0, 20.0));
    p.arc(rect(6.0, 16.0, 8.0, 8.0), 180.0, 90.0);
    p.line_to(pt(110.0, 16.0));
    p.arc(rect(106.0, 16.0, 8.0, 8.0), 270.0, 90.0);
    p.line_to(pt(114.0, 48.0));
    p.arc(rect(106.0, 44.0, 8.0, 8.0), 0.0, 90.0);
    p.line_to(pt(10.0, 52.0));
    p.arc(rect(6.0, 44.0, 8.0, 8.0), 90.0, 90.0);
    p.close();
    p
}

#[test]
fn drop_shape_fills_the_pointed_outline() {
    let mut r = surface();
    let brush = Brush::solid(Color::BLACK);
    let bounds = rect(10.0, 20.0, 100.0, 30.0);
    draw_rounded_rectangle(&mut r, bounds, &brush, 4, true, false, None);
    assert_eq!(
        r.calls,
        vec![Call::FillPath(drop_path(), brush, Transform::identity())]
    );
}

#[test]
fn rounded_shape_fills_four_corner_arcs() {
    let mut r = surface();
    let brush = Brush::solid(Color::BLACK);
    let bounds = rect(10.0, 20.0, 100.0, 30.0);
    draw_rounded_rectangle(&mut r, bounds, &brush, 4, false, false, None);
    assert_eq!(
        r.calls,
        vec![Call::FillPath(rounded_path(), brush, Transform::identity())]
    );
}

#[test]
fn contour_strokes_the_same_outline_when_a_pen_is_given() {
    let mut r = surface();
    let brush = Brush::solid(Color::BLACK);
    let pen = Pen::new(Color::rgb(1, 2, 3), 2.0);
    let bounds = rect(10.0, 20.0, 100.0, 30.0);
    draw_rounded_rectangle(&mut r, bounds, &brush, 4, false, true, Some(&pen));
    assert_eq!(
        r.calls,
        vec![
            Call::FillPath(rounded_path(), brush, Transform::identity()),
            Call::StrokePath(rounded_path(), pen, Transform::identity()),
        ]
    );
}

#[test]
fn contour_without_pen_only_fills() {
    let mut r = surface();
    let brush = Brush::solid(Color::BLACK);
    draw_rounded_rectangle(
        &mut r,
        rect(10.0, 20.0, 100.0, 30.0),
        &brush,
        4,
        false,
        true,
        None,
    );
    assert_eq!(r.calls.len(), 1);
}

#[test]
fn line_height_is_the_truncated_font_height() {
    // 16 px em: 1.2 * 16 = 19.2, truncated to 19.
    let font = Font {
        family: FontFamily::Arial,
        em_px: 16.0,
        bold: false,
    };
    assert_eq!(line_height(&surface(), &font), 19);
}

#[test]
fn text_box_fills_the_measured_box_then_draws_the_text_at_its_corner() {
    // "Label" at 16 px: width 5 * 0.55 * 16 = 44, height 19; radius 19 / 4 = 4.
    let mut r = surface();
    let font = Font {
        family: FontFamily::Arial,
        em_px: 16.0,
        bold: false,
    };
    let back = Brush::solid(Color::rgb(9, 9, 9));
    let fore = Brush::solid(Color::rgb(1, 1, 1));
    let boxed = draw_text_box(
        &mut r,
        "Label",
        &font,
        &back,
        &fore,
        Point::new(30, 40),
        None,
    );
    assert_eq!(boxed, Rectangle::new(30, 40, 44, 19));
    let mut expected = Path::default();
    expected.move_to(pt(26.0, 40.0));
    expected.arc(rect(26.0, 36.0, 8.0, 8.0), 180.0, 90.0);
    expected.line_to(pt(74.0, 36.0));
    expected.arc(rect(70.0, 36.0, 8.0, 8.0), 270.0, 90.0);
    expected.line_to(pt(78.0, 57.0));
    expected.arc(rect(70.0, 53.0, 8.0, 8.0), 0.0, 90.0);
    expected.line_to(pt(30.0, 61.0));
    expected.arc(rect(26.0, 53.0, 8.0, 8.0), 90.0, 90.0);
    expected.close();
    assert_eq!(
        r.calls,
        vec![
            Call::FillPath(expected, back, Transform::identity()),
            Call::DrawText(
                "Label".into(),
                font,
                fore,
                pt(30.0, 40.0),
                Transform::identity()
            ),
        ]
    );
}
