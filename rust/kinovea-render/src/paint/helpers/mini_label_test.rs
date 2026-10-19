use crate::backend::recording::Call;
use crate::paint::helpers::mini_label::{anchor_to, draw_mini_label};
use crate::paint::helpers::support_test::{surface, Scene};
use crate::renderer::{
    Brush, Font, FontFamily, LineCap, LineJoin, Path, PathSegment, Pen, Transform,
};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::measure_label::MeasureLabel;
use kinovea_metadata::Color;

fn pt(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

/// Label at (80, 50) attached to (100, 100): `MiniLabel` sits 20 px left and 50 px above its attach.
fn label() -> MeasureLabel {
    MeasureLabel::new(pt(100.0, 100.0))
}

/// Connector pen of `MiniLabel.Draw`: `GetBackgroundPen(alpha)`, width 1, round caps and joins.
fn connector_pen(alpha: u8) -> Pen {
    Pen {
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Pen::new(Color::argb(alpha, 200, 100, 0), 1.0)
    }
}

/// Font of `StyleData.GetFont(scale)`: 8 pt (the floor), bold Arial, converted at 96 DPI.
fn font(points: f32) -> Font {
    Font {
        family: FontFamily::Arial,
        em_px: points * (96.0 / 72.0),
        bold: true,
    }
}

/// `RoundedRectangle.Draw` of the box at (80, 50), 23 x 12 px, radius 3 (drop shape off).
fn box_path() -> Path {
    let mut p = Path::default();
    p.move_to(pt(77.0, 50.0));
    p.arc(RectangleF::new(77.0, 47.0, 6.0, 6.0), 180.0, 90.0);
    p.line_to(pt(103.0, 47.0));
    p.arc(RectangleF::new(100.0, 47.0, 6.0, 6.0), 270.0, 90.0);
    p.line_to(pt(106.0, 61.0));
    p.arc(RectangleF::new(100.0, 58.0, 6.0, 6.0), 0.0, 90.0);
    p.line_to(pt(80.0, 64.0));
    p.arc(RectangleF::new(77.0, 58.0, 6.0, 6.0), 90.0, 90.0);
    p.close();
    p
}

#[test]
fn connector_dot_line_box_and_text_at_scale_one() {
    // "12.5" at 10.667 px: width 4 * 0.55 * 10.667 = 23.47 -> 23, height 1.2 * 10.667 = 12.8 -> 12.
    // Connector centre: label + half of the untransformed size = (91.73, 56.4) -> (91, 56).
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    let back = Color::rgb(200, 100, 0);
    draw_mini_label(&mut r, &label(), "12.5", back, 8, 1.0, &scene.ctx(0));
    let (fill, fore) = (Brush::solid(back), Brush::solid(Color::rgb(255, 255, 255)));
    assert_eq!(
        r.calls,
        vec![
            Call::FillEllipse(
                RectangleF::new(98.0, 98.0, 4.0, 4.0),
                fill,
                Transform::identity()
            ),
            Call::StrokeLine(
                pt(100.0, 100.0),
                pt(91.0, 56.0),
                connector_pen(64),
                Transform::identity()
            ),
            Call::FillPath(box_path(), fill, Transform::identity()),
            Call::DrawText(
                "12.5".into(),
                font(8.0),
                fore,
                pt(80.0, 50.0),
                Transform::identity()
            ),
        ]
    );
}

#[test]
fn box_and_connector_follow_the_viewport_scale_and_origin() {
    // Origin (10, 10), scale 2: attach (210, 210), box at (170, 110). Font 16 pt -> 21.33 px:
    // width 46.93 -> 46, height 25.6 -> 25, radius 6. Centre: image (91.73, 56.4) -> (193, 122).
    let scene = Scene::new((10, 10), 2.0);
    let mut r = surface();
    draw_mini_label(
        &mut r,
        &label(),
        "12.5",
        Color::rgb(200, 100, 0),
        8,
        1.0,
        &scene.ctx(0),
    );
    assert_eq!(r.calls.len(), 4);
    assert!(
        matches!(&r.calls[0], Call::FillEllipse(rect, _, _) if *rect == RectangleF::new(208.0, 208.0, 4.0, 4.0))
    );
    assert!(
        matches!(&r.calls[1], Call::StrokeLine(a, b, _, _) if *a == pt(210.0, 210.0) && *b == pt(193.0, 122.0))
    );
    let Call::FillPath(path, _, _) = &r.calls[2] else {
        panic!("box is not a path")
    };
    assert_eq!(path.segments[0], PathSegment::MoveTo(pt(164.0, 110.0)));
    assert!(matches!(&r.calls[3], Call::DrawText(text, f, _, origin, _)
        if text == "12.5" && *f == font(16.0) && *origin == pt(170.0, 110.0)));
}

#[test]
fn opacity_scales_the_alpha_of_each_colour() {
    // 0.5: brush alpha int(127.5) = 127, connector alpha int(32) = 32.
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    draw_mini_label(
        &mut r,
        &label(),
        "12.5",
        Color::rgb(200, 100, 0),
        8,
        0.5,
        &scene.ctx(0),
    );
    let Call::StrokeLine(_, _, pen, _) = &r.calls[1] else {
        panic!("no connector")
    };
    assert_eq!(pen.color, Color::argb(32, 200, 100, 0));
    let Call::FillPath(_, brush, _) = &r.calls[2] else {
        panic!("no box")
    };
    assert_eq!(brush.color, Color::argb(127, 200, 100, 0));
}

#[test]
fn zero_opacity_paints_nothing() {
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    draw_mini_label(
        &mut r,
        &label(),
        "12.5",
        Color::rgb(200, 100, 0),
        8,
        0.0,
        &scene.ctx(0),
    );
    assert!(r.calls.is_empty());
}

#[test]
fn anchor_to_moves_the_label_with_its_attach_point() {
    // Relative offset (-20, -50) is kept when the attach point moves to (120, 100).
    let moved = anchor_to(&label(), pt(120.0, 100.0));
    assert_eq!(
        (moved.attach, moved.position),
        (pt(120.0, 100.0), pt(100.0, 50.0))
    );
}
