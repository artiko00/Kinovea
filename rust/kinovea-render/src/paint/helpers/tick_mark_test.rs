//! `TickMark.Draw` on the label box: the value is rounded, the box sits beside its anchor by the
//! margin, and the label is drawn at the box origin.

use super::tick_mark::{draw_tick_mark, TextAlignment, TickLabel, TickMark};
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::Scene;
use crate::renderer::{Brush, Font, FontFamily};
use crate::style::StyleData;
use crate::viewport::Point;
use kinovea_geometry::PointF;
use kinovea_metadata::Color;

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

/// Arial 8 points, bold, at 96 DPI: 10.6667 px.
fn label_font() -> Font {
    Font {
        family: FontFamily::Arial,
        em_px: 8.0 * 96.0 / 72.0,
        bold: true,
    }
}

/// Draws the label of 1.23456 at the anchor (100, 50) with a margin of 20 and returns its text and origin.
fn draw_label(alignment: TextAlignment, decimal_places: i32) -> (String, PointF) {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let tick = TickMark {
        value: 1.23456,
        image_location: PointF::new(100.0, 50.0),
        alignment,
    };
    let fill = Brush::solid(Color::rgb(0, 0, 0));
    let text = Brush::solid(Color::rgb(255, 255, 255));
    let font = label_font();
    let label = TickLabel {
        fill: &fill,
        text: &text,
        font: &font,
        decimal_places,
    };
    let calls = scene.record(0, |ctx, r| {
        draw_tick_mark(r, &tick, &label, &ctx, &identity, 20.0)
    });
    calls
        .iter()
        .find_map(|c| match c {
            Call::DrawText(text, _, _, origin, _) => Some((text.clone(), *origin)),
            _ => None,
        })
        .expect("one label")
}

#[test]
fn top_label_is_rounded_to_two_decimals_and_sits_above_its_tick() {
    // C#: TickMark.cs:26-27 (dp = 2, label = Math.Round), 44-61 (Top: y - height - margin).
    // "1.23" is 4 glyphs of 0.55 em at 10.6667 px: width 23.4667, height 12.8. Origin x = 100 - 11.7333,
    // y = 50 - 12.8 - 20 = 17.2.
    let (text, origin) = draw_label(TextAlignment::Top, 2);
    assert_eq!(text, "1.23");
    assert!(close(origin.x, 88.266_67) && close(origin.y, 17.2));
}

#[test]
fn left_label_sits_before_its_tick_and_is_centred_on_it() {
    // C#: TickMark.cs:44-61 (Left: x - width - margin, y - height / 2).
    // Origin x = 100 - 23.4667 - 20 = 56.5333, y = 50 - 6.4 = 43.6.
    let (_, origin) = draw_label(TextAlignment::Left, 2);
    assert!(close(origin.x, 56.533_33) && close(origin.y, 43.6));
}

#[test]
fn zero_decimal_places_round_the_value_to_an_integer() {
    // C#: TickMark.cs:26-27 with dp = 0: Math.Round(1.23456, 0) = 1.
    let (text, _) = draw_label(TextAlignment::Top, 0);
    assert_eq!(text, "1");
}

/// Ten-pixel font: "50" measures 11 by 12 (2 characters at 0.55 em, line 1.2 em).
fn small_font() -> Font {
    Font {
        em_px: 10.0,
        ..StyleData::default().font(1.0)
    }
}

/// Distortion that does nothing: the tick is where the image says.
fn identity(p: PointF) -> PointF {
    p
}

/// Draws "50" at `at` with a margin of 8 and returns the recorded calls.
fn draw_fifty(alignment: TextAlignment, at: PointF) -> Vec<Call> {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let tick = TickMark {
        value: 50.0,
        image_location: at,
        alignment,
    };
    let brush = Brush::solid(crate::paint::helpers::frame::scene::COLOR);
    let font = small_font();
    let label = TickLabel {
        fill: &brush,
        text: &brush,
        font: &font,
        decimal_places: 2,
    };
    scene.record(0, |ctx, r| {
        draw_tick_mark(r, &tick, &label, &ctx, &identity, 8.0)
    })
}

fn fifty_at(origin: PointF) -> Call {
    Call::DrawText(
        "50".into(),
        small_font(),
        Brush::solid(crate::paint::helpers::frame::scene::COLOR),
        origin,
        crate::renderer::Transform::identity(),
    )
}

#[test]
fn bottom_label_is_centred_below_its_point_on_a_rounded_background() {
    // x = 50 - 11 / 2 = 44.5; y = 40 + margin 8 = 48.
    let calls = draw_fifty(TextAlignment::Bottom, PointF::new(50.0, 40.0));
    assert_eq!(calls.len(), 2);
    assert!(matches!(calls[0], Call::FillPath(..)));
    assert_eq!(calls[1], fifty_at(PointF::new(44.5, 48.0)));
}

#[test]
fn left_label_is_left_of_its_point_and_centred_vertically() {
    // x = 50 - 11 - margin 8 = 31; y = 40 - 12 / 2 = 34.
    let calls = draw_fifty(TextAlignment::Left, PointF::new(50.0, 40.0));
    assert_eq!(calls[1], fifty_at(PointF::new(31.0, 34.0)));
}

#[test]
fn label_is_anchored_on_the_distorted_point() {
    // The distortion moves the tick from (50, 40) to (60, 40): x = 60 - 5.5 = 54.5, y = 40 + 8 = 48.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let tick = TickMark {
        value: 50.0,
        image_location: PointF::new(50.0, 40.0),
        alignment: TextAlignment::Bottom,
    };
    let brush = Brush::solid(crate::paint::helpers::frame::scene::COLOR);
    let font = small_font();
    let label = TickLabel {
        fill: &brush,
        text: &brush,
        font: &font,
        decimal_places: 2,
    };
    let shift = |p: PointF| PointF::new(p.x + 10.0, p.y);
    let calls = scene.record(0, |ctx, r| {
        draw_tick_mark(r, &tick, &label, &ctx, &shift, 8.0)
    });
    assert_eq!(calls[1], fifty_at(PointF::new(54.5, 48.0)));
}

#[test]
fn bottom_right_label_is_offset_by_the_margin_on_both_axes() {
    // x = 50 + 8 = 58; y = 40 + 8 = 48.
    let calls = draw_fifty(TextAlignment::BottomRight, PointF::new(50.0, 40.0));
    assert_eq!(calls[1], fifty_at(PointF::new(58.0, 48.0)));
}
