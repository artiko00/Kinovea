use crate::template::capture::capture;
use crate::template_test::{reference, square, tracker_with};
use crate::tracker::Tracker;
use kinovea_geometry::Size;
use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;
use kinovea_video::frame::FrameBuffer;

#[test]
fn a_confident_match_keeps_the_reference_template() {
    let scene = square(10);
    let mut parameters = TrackingParameters::default();
    parameters.template_update_threshold = 0.0;
    let mut tracker = tracker_with(parameters);
    let start = reference(&mut tracker, &scene, (160, 160), 0);
    tracker.track_step(&[start], 1, Some(&scene.render((172, 168))));
    let expected = capture(&scene.render((160, 160)), (40.0, 40.0), Size::new(20, 20));
    assert_eq!(tracker.templates().last().unwrap().patch, expected);
}

#[test]
fn a_fair_match_replaces_the_template_with_the_patch_at_the_new_point() {
    let scene = square(10);
    let mut parameters = TrackingParameters::default();
    parameters.template_update_threshold = 1.0;
    let mut tracker = tracker_with(parameters);
    let start = reference(&mut tracker, &scene, (160, 160), 0);
    let moved = scene.render((172, 168));
    let result = tracker.track_step(&[start], 1, Some(&moved));
    let stored = &tracker.templates().last().unwrap().patch;
    let expected = capture(&moved, (result.point.x, result.point.y), Size::new(20, 20));
    assert_eq!(stored, &expected);
    assert_ne!(
        stored,
        &capture(&scene.render((160, 160)), (40.0, 40.0), Size::new(20, 20))
    );
}

#[test]
fn trim_and_clear_drop_the_templates() {
    let scene = square(10);
    let mut tracker = tracker_with(TrackingParameters::default());
    let frame: FrameBuffer = scene.render((160, 160));
    let mut timeline = vec![reference(&mut tracker, &scene, (160, 160), 0)];
    for t in [10, 20] {
        let step = tracker.track_step(&timeline, t, Some(&frame));
        timeline.push(step.point);
    }
    assert_eq!(tracker.templates().len(), 3);
    tracker.trim(10);
    assert_eq!(tracker.templates().len(), 2);
    tracker.clear();
    assert!(tracker.templates().is_empty());
    assert!(!tracker.is_ready(&timeline[0]));
}
