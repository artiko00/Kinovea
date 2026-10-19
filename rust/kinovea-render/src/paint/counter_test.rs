use crate::backend::recording::Call;
use crate::paint::counter::paint_counter;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::renderer::{Brush, Font, FontFamily, Transform};
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Counter;
use kinovea_metadata::measure_label::MeasureLabelType;
use kinovea_metadata::{Color, StyleElements, StyleValue};

#[test]
fn counter_painters_are_declared() {
    assert!(module_path!().contains("counter"));
}

/// Beats at 1000, 1200 and 1500: at 1300 two beats have passed (index 1, `GetBeatIndexInterval`).
fn counter(kind: MeasureLabelType) -> Counter {
    let mut d = Counter::new(PointF::new(10.0, 20.0));
    d.beats = vec![1000, 1200, 1500];
    d.measure_label_type = kind;
    d.header.style = StyleElements::new(vec![
        (
            "back color",
            StyleValue::Color(Color::argb(255, 10, 20, 30)),
        ),
        ("font size", StyleValue::FontSize(12)),
    ]);
    d.header.fading.average_timestamps_per_frame = 10.0;
    d
}

fn text_of(d: &Counter, timestamp: i64) -> Option<String> {
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_counter(d, &scene.ctx(timestamp), &mut r);
    match r.calls.get(1) {
        Some(Call::DrawText(text, ..)) => Some(text.clone()),
        _ => None,
    }
}

#[test]
fn count_is_the_number_of_beats_passed() {
    let d = counter(MeasureLabelType::Count);
    assert_eq!(text_of(&d, 1300).as_deref(), Some("2"));
    // On a beat counts it: the third beat is at 1500.
    assert_eq!(text_of(&d, 1500).as_deref(), Some("3"));
}

#[test]
fn zero_based_count_starts_at_zero_on_the_first_beat() {
    let mut d = counter(MeasureLabelType::Count);
    d.zero_based = true;
    assert_eq!(text_of(&d, 1300).as_deref(), Some("1"));
}

#[test]
fn half_and_double_cadence_scale_the_count() {
    let mut half = counter(MeasureLabelType::Count);
    half.half_cadence = true;
    assert_eq!(text_of(&half, 1300).as_deref(), Some("1"));
    let mut double = counter(MeasureLabelType::Count);
    double.double_cadence = true;
    assert_eq!(text_of(&double, 1300).as_deref(), Some("4"));
}

#[test]
fn reverse_count_ends_at_zero_on_the_last_beat() {
    // 3 beats, index 1 at 1300: 3 - 1 - 1 = 1.
    let d = counter(MeasureLabelType::CountReverse);
    assert_eq!(text_of(&d, 1300).as_deref(), Some("1"));
}

#[test]
fn before_the_first_beat_or_unsupported_kinds_show_no_value() {
    let d = counter(MeasureLabelType::Count);
    assert_eq!(text_of(&d, 900).as_deref(), Some("*"));
    let other = counter(MeasureLabelType::Speed);
    assert_eq!(text_of(&other, 1300).as_deref(), Some("*"));
}

#[test]
fn text_is_drawn_at_the_position_in_16_px_font() {
    // "2" at 16 px: width 1 * 0.55 * 16 = 8.8 -> 8; the box and text are at (10, 20).
    let d = counter(MeasureLabelType::Count);
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_counter(&d, &scene.ctx(1300), &mut r);
    let font = Font {
        family: FontFamily::Arial,
        em_px: 16.0,
        bold: false,
    };
    assert_eq!(
        r.calls[1],
        Call::DrawText(
            "2".into(),
            font,
            Brush::solid(Color::argb(255, 255, 255, 255)),
            PointF::new(10.0, 20.0),
            Transform::identity()
        )
    );
}

#[test]
fn locked_counter_is_dimmed_to_three_quarters() {
    // Master 0.75: background alpha int(0.75 * 225) = 168.
    let mut d = counter(MeasureLabelType::Count);
    d.locked = true;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_counter(&d, &scene.ctx(1300), &mut r);
    assert!(
        matches!(&r.calls[0], Call::FillPath(_, b, _) if b.color == Color::argb(168, 10, 20, 30))
    );
}

// DrawingCounter.cs:988-1012 (average) with Metadata.GetCadence (Metadata.cs:1549): 2 cycles over
// 500 ticks at 25 ticks per second is 0.1 Hz; the period is 500 / 2 = 250 ticks.
#[test]
fn average_cadence_and_period_over_all_beats() {
    assert_eq!(
        text_of(&counter(MeasureLabelType::CadenceAverage), 1300).as_deref(),
        Some("0.1 Hz")
    );
    assert_eq!(
        text_of(&counter(MeasureLabelType::PeriodAverage), 1300).as_deref(),
        Some("250")
    );
}

// DrawingCounter.cs:960-985 (instant): inside the 1200-1500 interval, 300 ticks is 1 / 12 Hz, which
// MinimalDigits rounds to 0.08 Hz; the period is the 300 ticks themselves.
#[test]
fn instant_cadence_and_period_use_the_interval_around_the_time() {
    assert_eq!(
        text_of(&counter(MeasureLabelType::CadenceInstant), 1300).as_deref(),
        Some("0.08 Hz")
    );
    assert_eq!(
        text_of(&counter(MeasureLabelType::PeriodInstant), 1300).as_deref(),
        Some("300")
    );
}

// DrawingCounter.cs:967-974: exactly on a beat the interval is the one leading to it, and the first
// beat has none.
#[test]
fn instant_on_a_beat_uses_the_interval_leading_to_it() {
    assert_eq!(
        text_of(&counter(MeasureLabelType::PeriodInstant), 1200).as_deref(),
        Some("200")
    );
    assert_eq!(
        text_of(&counter(MeasureLabelType::CadenceInstant), 1000).as_deref(),
        Some("*")
    );
}

// DrawingCounter.cs:964-967 and 1000-1004: the half cadence doubles the period and halves the cycles.
#[test]
fn half_cadence_doubles_the_period() {
    let mut d = counter(MeasureLabelType::PeriodInstant);
    d.half_cadence = true;
    assert_eq!(text_of(&d, 1300).as_deref(), Some("600"));
}
