use super::CircleTracker;
use crate::synthetic::Scene;
use crate::tracker::Tracker;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::Size;
use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;
use kinovea_video::frame::FrameBuffer;
use std::collections::BTreeSet;

fn tracker() -> CircleTracker {
    let mut parameters = TrackingParameters::default();
    parameters.set_search_window(Size::new(80, 80));
    parameters.set_block_window(Size::new(20, 20));
    CircleTracker::new(parameters)
}

fn scene(size: i64) -> Scene {
    Scene {
        width: 160,
        height: 120,
        background: [40, 60, 80],
        gx: 8,
        gy: 4,
        color: [230, 230, 230],
        disc: true,
        size,
    }
}

/// Disc of radius 10 centered at `center`, in quarter pixels as the oracle cases give them.
fn disc(center: (i64, i64)) -> FrameBuffer {
    scene(10).render(center)
}

/// Uniform background: a scene whose object has no size.
fn blank() -> FrameBuffer {
    scene(0).render((0, 0))
}

fn line(radii: &[(i64, f32)]) -> Vec<TimedPoint> {
    radii
        .iter()
        .map(|&(t, r)| TimedPoint::with_radius(0.0, 0.0, t, r))
        .collect()
}

#[test]
fn reference_radius_averages_the_last_four_samples() {
    let mut t = tracker();
    t.reference_times.insert(0);
    let timeline = line(&[
        (0, 10.0),
        (1, 10.0),
        (2, 10.0),
        (3, 10.0),
        (4, 12.0),
        (5, 12.0),
    ]);
    assert_eq!(t.reference_radius(&timeline), Some(11.0));
}

#[test]
fn reference_radius_does_not_reach_before_the_last_reference() {
    let mut t = tracker();
    t.reference_times.extend([0, 3]);
    let timeline = line(&[
        (0, 20.0),
        (1, 20.0),
        (2, 20.0),
        (3, 12.0),
        (4, 12.0),
        (5, 12.0),
    ]);
    assert_eq!(t.reference_radius(&timeline), Some(12.0));
}

#[test]
fn step_without_image_keeps_the_point_and_its_radius() {
    let mut t = tracker();
    t.reference_times.insert(0);
    let timeline = line(&[(0, 10.0)]);
    let step = t.track_step(&timeline, 1, None);
    assert_eq!(step.point, TimedPoint::with_radius(0.0, 0.0, 1, 10.0));
    assert!(!step.matched);
    assert_eq!(step.score, 0.0);
}

#[test]
fn step_without_circle_keeps_the_point_and_its_radius() {
    let mut t = tracker();
    t.reference_times.insert(0);
    let timeline = [TimedPoint::with_radius(12.0, 8.0, 0, 10.0)];
    let frame = blank();
    let step = t.track_step(&timeline, 1, Some(&frame));
    assert_eq!(step.point, TimedPoint::with_radius(12.0, 8.0, 1, 10.0));
    assert!(!step.matched);
    assert_eq!(step.score, 0.0);
}

#[test]
fn disc_is_found_and_its_radius_is_stored_on_the_point() {
    let mut t = tracker();
    let mut reference = TimedPoint::new(50.0, 50.0, 0);
    t.create_reference_point(&mut reference, &disc((200, 200)));
    let timeline = [reference];
    let step = t.track_step(&timeline, 1, Some(&disc((206, 204))));
    assert!(step.matched);
    assert!(step.score > 0.0);
    assert_eq!(step.point.t, 1);
    assert!((step.point.x - 51.5).abs() <= 1.0);
    assert!((step.point.y - 51.0).abs() <= 1.0);
    assert!((step.point.r - 10.0).abs() <= 1.0);
}

#[test]
fn reference_point_takes_half_the_larger_block_side() {
    let mut parameters = TrackingParameters::default();
    parameters.set_search_window(Size::new(80, 80));
    parameters.set_block_window(Size::new(20, 30));
    let mut t = CircleTracker::new(parameters);
    let mut point = TimedPoint::new(1.0, 2.0, 7);
    t.create_reference_point(&mut point, &blank());
    assert_eq!(point.r, 15.0);
    assert!(t.is_ready(&point));
}

#[test]
fn not_ready_without_radius_or_reference() {
    let t = tracker();
    assert!(!t.is_ready(&TimedPoint::with_radius(0.0, 0.0, 0, 10.0)));
    let mut t = tracker();
    t.reference_times.insert(0);
    assert!(!t.is_ready(&TimedPoint::new(0.0, 0.0, 0)));
}

#[test]
fn trim_forgets_the_references_after_the_time_and_clear_forgets_all() {
    let mut t = tracker();
    t.reference_times.extend([0, 5]);
    t.trim(4);
    assert_eq!(t.reference_times, BTreeSet::from([0]));
    t.clear();
    assert!(t.reference_times.is_empty());
}
