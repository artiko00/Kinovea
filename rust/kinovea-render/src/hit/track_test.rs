use super::common::testing::Scene;
use super::track::*;
use super::Hit;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Track;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn tp(x: f32, y: f32, t: i64) -> TimedPoint {
    TimedPoint { x, y, t, r: 0.0 }
}

fn visible_track(see_future: bool) -> Track {
    let mut track = Track::new(vec![
        tp(10.0, 10.0, 0),
        tp(60.0, 40.0, 100),
        tp(110.0, 10.0, 200),
    ]);
    track.is_visible = true;
    track.visible = 0;
    track.invisible = i64::MAX;
    track.see_future = see_future;
    track
}

// DrawingTrack.cs:645-650: nothing is hit outside the visibility window.
#[test]
fn hidden_track_is_never_hit() {
    let scene = Scene::at_scale(1.0);
    let mut track = visible_track(false);
    track.is_visible = false;
    assert_eq!(hit_track(&track, p(10.0, 10.0), &scene.ctx(0)), Hit::Miss);
}

// DrawingTrack.cs:720-727: the point at the probe time is handle 1.
#[test]
fn current_point_is_handle_one() {
    let scene = Scene::at_scale(1.0);
    let track = visible_track(false);
    assert_eq!(
        hit_track(&track, p(110.0, 10.0), &scene.ctx(200)),
        Hit::Handle(1)
    );
}

// DrawingTrack.cs:742-770: the curve runs from the first point to the current one, end excluded, so
// at t = 100 only one point is visible and nothing on the curve hits.
#[test]
fn trajectory_is_body_over_the_visible_points() {
    let scene = Scene::at_scale(1.0);
    let track = visible_track(false);
    assert_eq!(hit_track(&track, p(35.0, 25.0), &scene.ctx(200)), Hit::Body);
    assert_eq!(hit_track(&track, p(35.0, 25.0), &scene.ctx(100)), Hit::Miss);
}

// DrawingTrack.cs:746-747: with see-future the curve runs to the last point.
#[test]
fn see_future_extends_the_trajectory() {
    let scene = Scene::at_scale(1.0);
    let track = visible_track(true);
    assert_eq!(hit_track(&track, p(35.0, 25.0), &scene.ctx(0)), Hit::Body);
}

// DrawingTrack.cs:757: a curve with no extent (here flat) has empty bounds and is not hit.
#[test]
fn flat_trajectory_is_not_hit() {
    let scene = Scene::at_scale(1.0);
    let mut track = visible_track(false);
    track.positions = vec![tp(10.0, 10.0, 0), tp(60.0, 10.0, 100), tp(110.0, 10.0, 200)];
    assert_eq!(hit_track(&track, p(35.0, 10.0), &scene.ctx(200)), Hit::Miss);
}
