use crate::backend::recording::Call;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::paint::track::paint_track;
use crate::paint::track_test::track;
use crate::renderer::{Font, FontFamily};
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{KeyframeLabel, Track};
use kinovea_metadata::measure_label::MeasureLabel;
use kinovea_metadata::{Color, Keyframe};

/// Bold 16 px label font of `MiniLabel` (its font size is the track's, 12 pt).
fn label_font() -> Font {
    Font {
        family: FontFamily::Arial,
        em_px: 16.0,
        bold: true,
    }
}

/// Keyframe label at t = 20 (point 2, at (20, 0)) with the default offset (0, -50).
fn with_keyframe_label(d: &mut Track) {
    let mut label = MeasureLabel::new(PointF::new(20.0, 0.0));
    label.timestamp = 20;
    d.keyframe_labels.push(KeyframeLabel {
        label,
        attach_index: 2,
    });
}

fn scene_with_keyframe() -> Scene {
    let mut scene = Scene::new((0, 0), 1.0);
    let mut keyframe = Keyframe::new(20, "Jump");
    keyframe.color = Color::rgb(0, 0, 255);
    scene.metadata.keyframes.push(keyframe);
    scene
}

#[test]
fn a_future_keyframe_label_uses_the_future_opacity_and_the_keyframe_colour() {
    // At t = 10 the keyframe at 20 is in the future: alpha int(0.25 * 255) = 63 on its box, in the
    // keyframe's blue, with the keyframe name "Jump" at (0, -50).
    let mut d = track();
    with_keyframe_label(&mut d);
    let scene = scene_with_keyframe();
    let mut r = surface();
    paint_track(&d, &scene.ctx(10), &mut r);
    assert!(r
        .calls
        .iter()
        .any(|c| matches!(c, Call::FillPath(_, b, _) if b.color == Color::argb(63, 0, 0, 255))));
    assert!(r
        .calls
        .iter()
        .any(|c| matches!(c, Call::DrawText(t, f, _, o, _)
        if t == "Jump" && *f == label_font() && *o == PointF::new(0.0, -50.0))));
}

#[test]
fn keyframe_labels_take_the_track_colour_when_colours_are_off() {
    let mut d = track();
    with_keyframe_label(&mut d);
    d.use_keyframe_colors = false;
    let scene = scene_with_keyframe();
    let mut r = surface();
    paint_track(&d, &scene.ctx(10), &mut r);
    assert!(r
        .calls
        .iter()
        .any(|c| matches!(c, Call::FillPath(_, b, _) if b.color == Color::argb(63, 255, 0, 0))));
}

#[test]
fn keyframe_labels_are_left_out_when_the_future_is_hidden() {
    let mut d = track();
    with_keyframe_label(&mut d);
    d.see_future = false;
    let scene = scene_with_keyframe();
    let mut r = surface();
    paint_track(&d, &scene.ctx(10), &mut r);
    assert!(!r
        .calls
        .iter()
        .any(|c| matches!(c, Call::DrawText(t, ..) if t == "Jump")));
}

#[test]
fn main_label_shows_the_track_name_on_the_current_point() {
    // Current point 1 is (10, 0); the label, anchored there with its own offset (-10, -50).
    let mut d = track();
    d.show_track_label = true;
    d.header.name = "Track".into();
    d.main_label = MeasureLabel::new(PointF::new(10.0, 0.0));
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_track(&d, &scene.ctx(10), &mut r);
    assert!(r
        .calls
        .iter()
        .any(|c| matches!(c, Call::DrawText(t, f, _, o, _)
        if t == "Track" && *f == label_font() && *o == PointF::new(-10.0, -50.0))));
}

#[test]
fn main_label_is_attached_to_the_first_point_before_the_track() {
    // At -5 the attach is the first point (0, 0): the label moves by (-10, 0) with it, to (-20, -50).
    let mut d = track();
    d.visible = -10;
    d.show_track_label = true;
    d.header.name = "Track".into();
    d.main_label = MeasureLabel::new(PointF::new(10.0, 0.0));
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_track(&d, &scene.ctx(-5), &mut r);
    assert!(r
        .calls
        .iter()
        .any(|c| matches!(c, Call::DrawText(t, _, _, o, _)
        if t == "Track" && *o == PointF::new(-20.0, -50.0))));
}
