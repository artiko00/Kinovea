use crate::synthetic::Scene;
use crate::template::TemplateTracker;
use crate::tracker::Tracker;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::Size;
use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;

/// A square of side `size` on the oracle background, in a 160x120 frame.
pub(crate) fn square(size: i64) -> Scene {
    Scene {
        width: 160,
        height: 120,
        background: [40, 60, 80],
        gx: 8,
        gy: 4,
        color: [230, 230, 230],
        disc: false,
        size,
    }
}

pub(crate) fn disc(size: i64) -> Scene {
    Scene {
        disc: true,
        ..square(size)
    }
}

/// Places a reference point at `center` (quarter pixels) and returns it.
pub(crate) fn reference(
    tracker: &mut TemplateTracker,
    scene: &Scene,
    center: (i64, i64),
    t: i64,
) -> TimedPoint {
    let mut point = TimedPoint::new((center.0 / 4) as f32, (center.1 / 4) as f32, t);
    tracker.create_reference_point(&mut point, &scene.render(center));
    point
}

pub(crate) fn tracker_with(parameters: TrackingParameters) -> TemplateTracker {
    TemplateTracker::new(parameters)
}

#[test]
fn a_tracker_is_ready_only_after_a_reference_point() {
    let scene = square(10);
    let mut tracker = tracker_with(TrackingParameters::default());
    let last = TimedPoint::new(40.0, 40.0, 0);
    assert!(!tracker.is_ready(&last));
    reference(&mut tracker, &scene, (160, 160), 0);
    assert!(tracker.is_ready(&last));
}

#[test]
fn a_changed_block_window_makes_the_tracker_not_ready() {
    let scene = square(10);
    let mut tracker = tracker_with(TrackingParameters::default());
    reference(&mut tracker, &scene, (160, 160), 0);
    tracker.parameters_mut().set_block_window(Size::new(30, 30));
    assert!(!tracker.is_ready(&TimedPoint::new(40.0, 40.0, 0)));
}

#[test]
fn a_step_follows_a_translated_target() {
    let scene = square(10);
    let mut tracker = tracker_with(TrackingParameters::default());
    let start = reference(&mut tracker, &scene, (160, 160), 0);
    let result = tracker.track_step(&[start], 1, Some(&scene.render((172, 168))));
    assert!(result.matched);
    assert!(
        (result.point.x - 43.0).abs() <= 0.5,
        "x = {}",
        result.point.x
    );
    assert!(
        (result.point.y - 42.0).abs() <= 0.5,
        "y = {}",
        result.point.y
    );
    assert_eq!(result.point.t, 1);
    assert!(result.score >= 0.5, "score = {}", result.score);
}

#[test]
fn a_lost_target_keeps_the_previous_position_and_scores_zero() {
    let scene = square(10);
    let mut tracker = tracker_with(TrackingParameters::default());
    let start = reference(&mut tracker, &scene, (160, 160), 0);
    let result = tracker.track_step(&[start], 1, Some(&scene.render((480, 360))));
    assert!(!result.matched);
    assert_eq!(result.point, TimedPoint::new(40.0, 40.0, 1));
    assert_eq!(result.score, 0.0);
}

#[test]
fn a_step_without_image_keeps_the_previous_position() {
    let scene = square(10);
    let mut tracker = tracker_with(TrackingParameters::default());
    let start = reference(&mut tracker, &scene, (160, 160), 0);
    let result = tracker.track_step(&[start], 1, None);
    assert!(!result.matched);
    assert_eq!(result.point, TimedPoint::new(40.0, 40.0, 1));
}

#[test]
fn a_half_pixel_motion_is_refined_within_0_15_pixel() {
    let scene = square(10);
    let mut tracker = tracker_with(TrackingParameters::default());
    let start = reference(&mut tracker, &scene, (160, 160), 0);
    let result = tracker.track_step(&[start], 1, Some(&scene.render((162, 160))));
    assert!(
        (result.point.x - 40.5).abs() <= 0.15,
        "x = {}",
        result.point.x
    );
    assert!(
        (result.point.y - 40.0).abs() <= 0.15,
        "y = {}",
        result.point.y
    );
}
