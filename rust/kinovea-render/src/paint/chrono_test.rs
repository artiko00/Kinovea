use crate::backend::recording::Call;
use crate::paint::chrono::paint_chrono;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::renderer::{Brush, Font, FontFamily, Transform};
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Chrono;
use kinovea_metadata::style::{StyleValue, ToggleVariant};
use kinovea_metadata::{Color, StyleElements};

#[test]
fn chrono_helpers_are_declared() {
    assert!(module_path!().contains("chrono"));
}

const DARK: Color = Color::argb(255, 10, 20, 30);
const WHITE: Color = Color::argb(255, 255, 255, 255);

/// Style of the drawings below: dark background (white text), 12 pt, no clock toggle.
fn style() -> StyleElements {
    StyleElements::new(vec![
        ("back color", StyleValue::Color(DARK)),
        ("font size", StyleValue::FontSize(12)),
    ])
}

fn chrono(start: i64) -> Chrono {
    let mut d = Chrono::new(PointF::new(10.0, 20.0));
    d.start_counting = start;
    d.header.style = style();
    d.header.fading.average_timestamps_per_frame = 10.0;
    d
}

/// 12 pt is 16 px at 96 DPI.
fn font(em_px: f32) -> Font {
    Font {
        family: FontFamily::Arial,
        em_px,
        bold: false,
    }
}

#[test]
fn stopwatch_shows_the_elapsed_time_on_a_background_at_225_alpha() {
    // Started at 1000, now 1500: "500" is 3 * 0.55 * 16 = 26.4 px wide and the text is drawn at the corner.
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_chrono(&chrono(1000), &scene.ctx(1500), &mut r);
    assert!(
        matches!(&r.calls[0], Call::FillPath(_, b, _) if b.color == Color::argb(225, 10, 20, 30))
    );
    assert_eq!(
        r.calls[1],
        Call::DrawText(
            "500".into(),
            font(16.0),
            Brush::solid(WHITE),
            PointF::new(10.0, 20.0),
            Transform::identity()
        )
    );
    assert_eq!(r.calls.len(), 2);
}

#[test]
fn stopwatch_before_start_shows_zero_and_after_stop_keeps_the_last_value() {
    let scene = Scene::new((0, 0), 1.0);
    let mut before = surface();
    paint_chrono(&chrono(1000), &scene.ctx(900), &mut before);
    assert!(matches!(&before.calls[1], Call::DrawText(t, ..) if t == "0"));

    let mut stopped = chrono(1000);
    stopped.stop_counting = 2000;
    let mut after = surface();
    paint_chrono(&stopped, &scene.ctx(2500), &mut after);
    assert!(matches!(&after.calls[1], Call::DrawText(t, ..) if t == "1000"));
}

#[test]
fn clock_mode_shows_the_time_since_the_origin() {
    let mut d = chrono(1000);
    d.header
        .style
        .elements
        .push(kinovea_metadata::StyleElement {
            key: "clock".into(),
            value: StyleValue::Toggle {
                value: true,
                variant: ToggleVariant::Clock,
            },
        });
    d.clock_origin = 300;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_chrono(&d, &scene.ctx(1500), &mut r);
    assert!(matches!(&r.calls[1], Call::DrawText(t, ..) if t == "1200"));
}

#[test]
fn nothing_is_painted_before_visible() {
    let mut d = chrono(1000);
    d.visible = 2000;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_chrono(&d, &scene.ctx(1500), &mut r);
    assert!(r.calls.is_empty());
}

#[test]
fn after_invisible_the_opacity_fades_out() {
    // Invisible from 100; at 140 the fade is 1 - 140 / 200 = 0.3, alpha int(67.5) = 67 for the
    // background and int(76.5) = 76 for the text.
    let mut d = chrono(0);
    d.invisible = 100;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_chrono(&d, &scene.ctx(140), &mut r);
    assert!(
        matches!(&r.calls[0], Call::FillPath(_, b, _) if b.color == Color::argb(67, 10, 20, 30))
    );
    assert!(
        matches!(&r.calls[1], Call::DrawText(_, _, b, _, _) if b.color == Color::argb(76, 255, 255, 255))
    );
}

#[test]
fn zero_opacity_paints_nothing() {
    let mut d = chrono(0);
    d.invisible = 100;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_chrono(&d, &scene.ctx(200), &mut r);
    assert!(r.calls.is_empty());
}

#[test]
fn scale_two_doubles_the_font_and_moves_the_origin() {
    // Origin (10, 10), scale 2: text at (30, 50); 12 pt * 2 = 24 pt = 32 px.
    let scene = Scene::new((10, 10), 2.0);
    let mut r = surface();
    paint_chrono(&chrono(1000), &scene.ctx(1500), &mut r);
    assert!(
        matches!(&r.calls[1], Call::DrawText(t, f, _, o, _) if t == "500" && *f == font(32.0) && *o == PointF::new(30.0, 50.0))
    );
}

#[test]
fn name_label_sits_above_the_box_with_a_drop_shape() {
    // Name "Split" in the label font (6 pt floored to 8 pt = 10.67 px): 5 * 0.55 * 10.67 = 29.3 -> 29,
    // height 12. Box radius 19 / 4 = 4, label radius 12 / 3 = 4, so top = 20 - 12 - 4 - 4 = 0.
    let mut d = chrono(1000);
    d.show_label = true;
    d.header.name = "Split".into();
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_chrono(&d, &scene.ctx(1500), &mut r);
    assert!(matches!(&r.calls[2], Call::FillPath(..)));
    assert_eq!(
        r.calls[3],
        Call::DrawText(
            "Split".into(),
            font(8.0 * (96.0 / 72.0)),
            Brush::solid(WHITE),
            PointF::new(10.0, 0.0),
            Transform::identity()
        )
    );
}
