use crate::backend::recording::Call;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::paint::label::paint_label;
use crate::renderer::{Brush, Font, FontFamily, LineCap, LineJoin, Pen, Transform};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Label;
use kinovea_metadata::{Color, StyleElements, StyleValue};

#[test]
fn label_painters_are_declared() {
    assert!(module_path!().contains("label"));
}

/// Dark background, so the text is white (`GetForegroundColor`). Font 12 pt is 16 px at 96 DPI.
fn label(text: &str, background: Color) -> Label {
    let mut d = Label::new(text, PointF::new(10.0, 20.0));
    d.header.fading.average_timestamps_per_frame = 10.0;
    d.header.style = StyleElements::new(vec![
        ("back color", StyleValue::Color(background)),
        ("font size", StyleValue::FontSize(12)),
        ("line size", StyleValue::LineSize(1)),
    ]);
    d
}

const DARK: Color = Color::argb(255, 10, 20, 30);

fn font() -> Font {
    Font {
        family: FontFamily::Arial,
        em_px: 16.0,
        bold: false,
    }
}

#[test]
fn background_box_then_text_at_the_position() {
    // "Label" at 16 px: 44 x 19 box at (10, 20); the text is drawn at its corner.
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_label(&label("Label", DARK), &scene.ctx(0), &mut r);
    let fore = Brush::solid(Color::argb(255, 255, 255, 255));
    assert!(matches!(&r.calls[0], Call::FillPath(_, brush, _) if brush.color == DARK));
    assert_eq!(
        r.calls[1],
        Call::DrawText(
            "Label".into(),
            font(),
            fore,
            PointF::new(10.0, 20.0),
            Transform::identity()
        )
    );
    assert_eq!(r.calls.len(), 2);
}

#[test]
fn fading_scales_the_background_and_text_alpha() {
    // 40 timestamps after the reference, 10 per frame, 20 fading frames: 1 - 40 / 200 = 0.8,
    // so the alpha is int(0.8 * 255) = 204 for both the box and the text.
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_label(&label("Label", DARK), &scene.ctx(40), &mut r);
    assert!(
        matches!(&r.calls[0], Call::FillPath(_, brush, _) if brush.color == Color::argb(204, 10, 20, 30))
    );
    assert!(
        matches!(&r.calls[1], Call::DrawText(_, _, brush, _, _) if brush.color == Color::argb(204, 255, 255, 255))
    );
}

#[test]
fn zero_fading_paints_nothing() {
    // 200 timestamps after the reference is the end of the fading: opacity 0.
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_label(&label("Label", DARK), &scene.ctx(200), &mut r);
    assert!(r.calls.is_empty());
}

#[test]
fn without_background_the_text_takes_the_background_colour() {
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    let mut d = label("Label", DARK);
    d.background_visible = false;
    paint_label(&d, &scene.ctx(0), &mut r);
    let back = Brush::solid(DARK);
    assert_eq!(
        r.calls,
        vec![Call::DrawText(
            "Label".into(),
            font(),
            back,
            PointF::new(10.0, 20.0),
            Transform::identity()
        )]
    );
}

#[test]
fn arrow_runs_from_the_box_border_to_the_arrow_end_before_the_box() {
    // Arrow end straight above the box centre (32, 29.5): the border crossing is (32, 20).
    // `ArrowHelper.UpdateStartEnd` (ArrowHelper.cs:62-77): width 1 -> reference 4, offset
    // = v * 12 / 70 = (0, -12), so the leader stops at the truncated (32, -38) and the head is
    // drawn there pointing back to the box: its tip (ArrowHelper.cs:22-24) lands on (32, -50).
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    let mut d = label("Label", DARK);
    d.arrow_visible = true;
    d.arrow_end = PointF::new(32.0, -50.0);
    paint_label(&d, &scene.ctx(0), &mut r);
    let pen = Pen {
        color: DARK,
        start_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Pen::new(DARK, 1.0)
    };
    assert_eq!(
        r.calls[0],
        Call::StrokeLine(
            PointF::new(32.0, 20.0),
            PointF::new(32.0, -38.0),
            pen,
            Transform::identity()
        )
    );
    assert!(
        matches!(&r.calls[1], Call::FillPolygon(points, _, _) if points[0] == PointF::new(32.0, -50.0))
    );
}

#[test]
fn circle_is_three_line_widths_around_the_arrow_end() {
    // Circle radius = 3 * pen width (1): rect (29, -53, 6, 6) around (32, -50).
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    let mut d = label("Label", DARK);
    d.circle_visible = true;
    d.arrow_end = PointF::new(32.0, -50.0);
    paint_label(&d, &scene.ctx(0), &mut r);
    assert!(
        matches!(&r.calls[1], Call::StrokeEllipse(rect, _, _) if *rect == RectangleF::new(29.0, -53.0, 6.0, 6.0))
    );
}

#[test]
fn scale_two_moves_the_box_and_the_arrow_end() {
    // Origin (10, 10), scale 2: box at (30, 50), arrow end (32, -50) -> (74, -90).
    let scene = Scene::new((10, 10), 2.0);
    let mut r = surface();
    let mut d = label("Label", DARK);
    d.arrow_visible = true;
    d.arrow_end = PointF::new(32.0, -50.0);
    paint_label(&d, &scene.ctx(0), &mut r);
    // The leader heads to (74, -90) and is shortened by the arrow offset along the vertical.
    assert!(
        matches!(&r.calls[0], Call::StrokeLine(a, b, _, _) if a.x == 74.0 && b.x == 74.0 && b.y > -90.0 && b.y < a.y)
    );
    let text = r.calls.iter().find(|c| matches!(c, Call::DrawText(..)));
    assert!(
        matches!(text, Some(Call::DrawText(_, f, _, o, _)) if *f == Font { em_px: 32.0, ..font() } && *o == PointF::new(30.0, 50.0))
    );
}
