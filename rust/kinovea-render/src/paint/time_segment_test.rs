use crate::backend::recording::Call;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::paint::time_segment::paint_time_segment;
use crate::renderer::{Font, FontFamily, LineJoin, Pen, Transform};
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::TimeSegment;
use kinovea_metadata::measure_label::MeasureLabel;
use kinovea_metadata::{Color, StyleElements, StyleValue};

/// Segment from (10, 20) to (110, 20) at fraction 0.5, reference at 80, 40 timestamps per frame.
fn segment() -> TimeSegment {
    let mut d = TimeSegment::new(PointF::new(10.0, 20.0), PointF::new(110.0, 20.0));
    d.header.style = StyleElements::new(vec![
        ("color", StyleValue::Color(Color::rgb(10, 20, 30))),
        ("font size", StyleValue::FontSize(12)),
        ("line size", StyleValue::LineSize(3)),
    ]);
    d.header.fading.reference_timestamp = 80;
    d.header.fading.average_timestamps_per_frame = 40.0;
    // `MiniLabel(attach)` places the label 20 px left and 50 px above the attach point. The model's
    // constructor only sets the attach point, so the label is placed here as the original does.
    d.label = MeasureLabel::new(d.time_point());
    d
}

fn scene(origin: (i32, i32), scale: f64) -> Scene {
    let mut scene = Scene::new(origin, scale);
    scene.metadata.video.average_timestamps_per_frame = 40.0;
    scene
}

#[test]
fn segment_and_tick_at_a_are_one_pixel_wide_then_the_label() {
    // Tick: a + (0, -20) = (10, 0) and a - (0, -20) = (10, 40), from ab = (100, 0) * 0.2.
    let scene = scene((0, 0), 1.0);
    let mut r = surface();
    paint_time_segment(&segment(), &scene.ctx(80), &mut r);
    let pen = Pen {
        color: Color::rgb(10, 20, 30),
        line_join: LineJoin::Round,
        ..Pen::new(Color::rgb(10, 20, 30), 1.0)
    };
    assert_eq!(
        r.calls[0],
        Call::StrokeLine(
            PointF::new(10.0, 20.0),
            PointF::new(110.0, 20.0),
            pen.clone(),
            Transform::identity()
        )
    );
    assert_eq!(
        r.calls[1],
        Call::StrokeLine(
            PointF::new(10.0, 0.0),
            PointF::new(10.0, 40.0),
            pen,
            Transform::identity()
        )
    );
}

#[test]
fn label_text_is_the_time_at_the_fraction_of_the_reference_frame() {
    // Reference 80 is frame round(80 / 40) = 2; fraction 0.5 gives (2 + 0.5) * 40 = 100.
    let scene = scene((0, 0), 1.0);
    let mut r = surface();
    paint_time_segment(&segment(), &scene.ctx(80), &mut r);
    let font = Font {
        family: FontFamily::Arial,
        em_px: 16.0,
        bold: true,
    };
    let Call::DrawText(text, f, _, origin, _) = r.calls.last().unwrap() else {
        panic!("no label text")
    };
    assert_eq!((text.as_str(), *f), ("100", font));
    // The label sits 20 px left and 50 px above the point at the fraction, (60, 20): (40, -30).
    assert_eq!(*origin, PointF::new(40.0, -30.0));
}

#[test]
fn the_connector_joins_the_fraction_point_to_the_label() {
    let scene = scene((0, 0), 1.0);
    let mut r = surface();
    paint_time_segment(&segment(), &scene.ctx(80), &mut r);
    // Label "100" is 26.4 x 19.2 px: its centre is (40 + 13.2, -30 + 9.6) = (53.2, -20.4) -> (53, -20).
    assert!(
        matches!(&r.calls[3], Call::StrokeLine(a, b, _, _) if *a == PointF::new(60.0, 20.0) && *b == PointF::new(53.0, -20.0))
    );
}

#[test]
fn the_pen_stays_one_pixel_at_scale_two() {
    // Origin (10, 10), scale 2: segment (30, 50) to (230, 50); the tick is (30, 10) to (30, 90).
    let scene = scene((10, 10), 2.0);
    let mut r = surface();
    paint_time_segment(&segment(), &scene.ctx(80), &mut r);
    let Call::StrokeLine(a, b, pen, _) = &r.calls[0] else {
        panic!("no segment")
    };
    assert_eq!(
        (*a, *b, pen.width),
        (PointF::new(30.0, 50.0), PointF::new(230.0, 50.0), 1.0)
    );
    assert!(
        matches!(&r.calls[1], Call::StrokeLine(p, q, _, _) if *p == PointF::new(30.0, 10.0) && *q == PointF::new(30.0, 90.0))
    );
}

#[test]
fn nothing_is_painted_at_zero_fading() {
    // 20 fading frames of 40 timestamps span 800: at 880, 800 after the reference, opacity is 0.
    let scene = scene((0, 0), 1.0);
    let mut r = surface();
    paint_time_segment(&segment(), &scene.ctx(880), &mut r);
    assert!(r.calls.is_empty());
}
