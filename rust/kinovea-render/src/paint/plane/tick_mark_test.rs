use super::tick_mark::{draw_tick_mark, Alignment, TickLabel, TickMark};
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::Scene;
use crate::renderer::{Brush, Font, FontFamily};
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

/// Draws the label of 1.23456 at the image point (100, 50) and returns its text and origin.
fn draw_label(alignment: Alignment, decimal_places: i32) -> (String, PointF) {
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
    let calls = scene.record(0, |ctx, r| draw_tick_mark(r, &tick, &label, &ctx));
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
    let (text, origin) = draw_label(Alignment::Top, 2);
    assert_eq!(text, "1.23");
    assert!(close(origin.x, 88.266_67) && close(origin.y, 17.2));
}

#[test]
fn left_label_sits_before_its_tick_and_is_centred_on_it() {
    // C#: TickMark.cs:44-61 (Left: x - width - margin, y - height / 2).
    // Origin x = 100 - 23.4667 - 20 = 56.5333, y = 50 - 6.4 = 43.6.
    let (_, origin) = draw_label(Alignment::Left, 2);
    assert!(close(origin.x, 56.533_33) && close(origin.y, 43.6));
}

#[test]
fn zero_decimal_places_round_the_value_to_an_integer() {
    // C#: TickMark.cs:26-27 with dp = 0: Math.Round(1.23456, 0) = 1.
    let (text, _) = draw_label(Alignment::Top, 0);
    assert_eq!(text, "1");
}
