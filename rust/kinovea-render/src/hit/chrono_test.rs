use super::chrono::{hit_chrono, hit_chrono_multi, hit_counter};
use super::common::testing::Scene;
use super::Hit;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Chrono, ChronoMulti, Counter};
use kinovea_metadata::measure_label::MeasureLabelType;
use kinovea_metadata::StyleValue;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

/// A chrono at `position` with a 12 pt font, so the box sizes below are exact.
fn chrono(position: PointF) -> Chrono {
    let mut c = Chrono::new(position);
    c.header.style.insert("font size", StyleValue::FontSize(12));
    c
}

fn scene_showing_the_raw_time() -> Scene {
    let mut scene = Scene::at_scale(1.0);
    scene.metadata.timecode = |t| t.to_string();
    scene
}

// DrawingChrono.cs:280-290: nothing before the visible time, nothing after the hiding time plus
// allowedFramesOver (12 frames, DrawingChrono.cs:143).
#[test]
fn hit_only_inside_the_visibility_window() {
    let mut scene = scene_showing_the_raw_time();
    scene.metadata.video.average_timestamps_per_frame = 40.0;
    let mut chrono = chrono(p(100.0, 100.0));
    chrono.visible = 100;
    chrono.invisible = 200;
    // Stopwatch started at 0: the shown duration is the timestamp itself.
    chrono.start_counting = 0;
    assert_eq!(
        hit_chrono(&chrono, p(110.0, 105.0), &scene.ctx(99)),
        Hit::Miss
    );
    assert_eq!(
        hit_chrono(&chrono, p(110.0, 105.0), &scene.ctx(100)),
        Hit::Body
    );
    assert_eq!(
        hit_chrono(&chrono, p(110.0, 105.0), &scene.ctx(200 + 480)),
        Hit::Body
    );
    assert_eq!(
        hit_chrono(&chrono, p(110.0, 105.0), &scene.ctx(200 + 481)),
        Hit::Miss
    );
}

// DrawingChrono.cs:280-287: the timecode box's hidden resizer is handle 1 at its bottom-right corner.
#[test]
fn timecode_box_resizer_is_handle_one() {
    let scene = {
        let mut s = Scene::at_scale(1.0);
        s.metadata.timecode = |_| "00000".to_string();
        s
    };
    let chrono = chrono(p(100.0, 100.0));
    assert_eq!(
        hit_chrono(&chrono, p(143.0, 118.0), &scene.ctx(0)),
        Hit::Handle(1)
    );
    assert_eq!(
        hit_chrono(&chrono, p(110.0, 105.0), &scene.ctx(0)),
        Hit::Body
    );
}

// DrawingChrono.cs:698-721 (GetTimecode, stopwatch): before the start the duration is zero.
#[test]
fn stopwatch_shows_zero_before_it_starts() {
    let scene = scene_showing_the_raw_time();
    let mut chrono = chrono(p(100.0, 100.0));
    chrono.visible = 0;
    chrono.start_counting = 500;
    // The text "0" is one glyph wide, so the resizer sits at x 108.8 and the box is narrower than
    // the raw "100" would make it.
    assert_eq!(
        hit_chrono(&chrono, p(107.0, 118.0), &scene.ctx(100)),
        Hit::Handle(1)
    );
}

// DrawingChrono.cs:262-276: the name box sits above the timecode box, on its left edge.
#[test]
fn name_box_above_the_timecode_is_body() {
    let scene = {
        let mut s = Scene::at_scale(1.0);
        s.metadata.timecode = |_| "00000".to_string();
        s
    };
    let mut chrono = chrono(p(100.0, 100.0));
    chrono.show_label = true;
    chrono.header.name = "Lap".to_string();
    assert_eq!(
        hit_chrono(&chrono, p(105.0, 85.0), &scene.ctx(0)),
        Hit::Body
    );
    chrono.show_label = false;
    assert_eq!(
        hit_chrono(&chrono, p(105.0, 85.0), &scene.ctx(0)),
        Hit::Miss
    );
}

// DrawingCounter.cs:323-348 with BuildText (paint::counter): the count of beats up to the time.
#[test]
fn counter_count_is_the_text_of_its_box() {
    let scene = scene_showing_the_raw_time();
    let mut counter = Counter::new(p(100.0, 100.0));
    counter.beats = vec![0, 100, 200];
    counter.measure_label_type = MeasureLabelType::Count;
    counter.invisible = i64::MAX;
    // At 150 the count is 2: one glyph of 8.8 px, so the resizer is at x 108.8.
    assert_eq!(
        hit_counter(&counter, p(107.0, 118.0), &scene.ctx(150)),
        Hit::Handle(1)
    );
    // Before the visible time the counter is not hittable (DrawingCounter.cs:323-334).
    counter.visible = 120;
    assert_eq!(
        hit_counter(&counter, p(107.0, 118.0), &scene.ctx(50)),
        Hit::Miss
    );
}

// DrawingCounter.cs:1 and paint::counter: a locked counter prefixes its name with the square.
#[test]
fn locked_counter_name_tag_is_body() {
    let scene = {
        let mut s = Scene::at_scale(1.0);
        s.metadata.timecode = |_| "0".to_string();
        s
    };
    let mut counter = Counter::new(p(100.0, 100.0));
    counter.beats = vec![0];
    counter.measure_label_type = MeasureLabelType::Count;
    counter.invisible = i64::MAX;
    counter.show_label = true;
    counter.locked = true;
    counter.header.name = "Lap".to_string();
    assert_eq!(
        hit_counter(&counter, p(105.0, 85.0), &scene.ctx(0)),
        Hit::Body
    );
}

// DrawingChronoMulti.cs:340-362: before the first section the text is the zero timecode.
#[test]
fn chrono_multi_before_its_sections_shows_zero() {
    let scene = scene_showing_the_raw_time();
    let multi = ChronoMulti::new(p(100.0, 100.0));
    assert_eq!(
        hit_chrono_multi(&multi, p(107.0, 118.0), &scene.ctx(0)),
        Hit::Handle(1)
    );
}
