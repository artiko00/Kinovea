use crate::camera_test::ShiftCamera;
use crate::drawing_tracker::DrawingState;
use crate::drawing_tracker_test::{bound_drawing, moving, track, FakeTrackable};
use kinovea_geometry::PointF;
use kinovea_metadata::trackability::TrackabilityManager;
use kinovea_metadata::Uuid;

#[test]
fn tracking_sends_the_sample_with_age_zero() {
    // DrawingTracker.cs:459-497: while tracking the step's point is sent with age 0 (:497).
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let data = bound_drawing(id, bound);
    let mut state = DrawingState::default();
    state.currently_tracking = true;
    let mut drawing = FakeTrackable::default();
    state.apply_at(
        data.tracker(id).unwrap(),
        &moving(bound),
        45,
        &ShiftCamera::off(),
        &mut drawing,
    );
    assert_eq!(
        drawing.values,
        vec![("a".to_owned(), PointF::new(7.0, 8.0), 0)]
    );
}

#[test]
fn viewing_sends_the_age_of_the_sample() {
    // DrawingTracker.cs:476-484: not tracking, the clamped sample at 40 is sent with age 40 - 45 (:484).
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let data = bound_drawing(id, bound);
    let mut state = DrawingState::default();
    let mut drawing = FakeTrackable::default();
    state.apply_at(
        data.tracker(id).unwrap(),
        &moving(bound),
        45,
        &ShiftCamera::off(),
        &mut drawing,
    );
    assert_eq!(
        drawing.values,
        vec![("a".to_owned(), PointF::new(7.0, 8.0), -5)]
    );
}

#[test]
fn points_without_a_sample_keep_their_value() {
    // DrawingTracker.cs:476-481: no sample at the time, nothing is sent. Unbound points are not
    // reached by the track steps at all.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let mut data = bound_drawing(id, bound);
    data.add(
        id,
        "Line",
        0,
        &[
            ("a".into(), PointF::new(1.0, 1.0)),
            ("b".into(), PointF::new(3.0, 3.0)),
        ],
    );
    let gap = [track(bound, &[(0, 0.0, 0.0), (100, 7.0, 8.0)])];
    let mut state = DrawingState::default();
    let mut drawing = FakeTrackable::default();
    state.apply_at(
        data.tracker(id).unwrap(),
        &gap,
        50,
        &ShiftCamera::on(),
        &mut drawing,
    );
    assert!(drawing.values.is_empty());
    assert_eq!(drawing.finished, 1);
}

#[test]
fn camera_moves_points_only_without_object_tracking() {
    // DrawingTracker.cs:517-546 (set with age -1 at :545), reached only when the camera is initialized
    // (TrackabilityManager.cs:337-341).
    let id = Uuid::new_v4();
    let mut data = TrackabilityManager::default();
    data.add(id, "Line", 0, &[("a".into(), PointF::new(1.0, 1.0))]);
    let camera = ShiftCamera::on();
    let mut state = DrawingState::default();
    let mut drawing = FakeTrackable::default();
    state.apply_at(data.tracker(id).unwrap(), &[], 50, &camera, &mut drawing);
    assert_eq!(
        drawing.values,
        vec![("a".to_owned(), PointF::new(51.0, 1.0), -1)]
    );
    assert_eq!(camera.calls.get(), 1);
}

#[test]
fn camera_does_not_move_points_while_object_tracking() {
    // DrawingTracker.cs:519-520: CameraTrackingStep returns when object tracking is initialized.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let data = bound_drawing(id, bound);
    let gap = [track(bound, &[(0, 0.0, 0.0), (100, 7.0, 8.0)])];
    let camera = ShiftCamera::on();
    let mut state = DrawingState::default();
    let mut drawing = FakeTrackable::default();
    state.apply_at(data.tracker(id).unwrap(), &gap, 50, &camera, &mut drawing);
    assert!(drawing.values.is_empty());
    assert_eq!(camera.calls.get(), 0);
}

#[test]
fn without_camera_motion_nothing_is_sent_and_the_drawing_still_finishes() {
    // TrackabilityManager.cs:339-340 returns before any tracker; AfterAllTracksTrackingStep (:325-330)
    // still runs for every tracker.
    let id = Uuid::new_v4();
    let mut data = TrackabilityManager::default();
    data.add(id, "Line", 0, &[("a".into(), PointF::new(1.0, 1.0))]);
    let mut state = DrawingState::default();
    let mut drawing = FakeTrackable::default();
    state.apply_at(
        data.tracker(id).unwrap(),
        &[],
        50,
        &ShiftCamera::off(),
        &mut drawing,
    );
    assert!(drawing.values.is_empty());
    assert_eq!(drawing.finished, 1);
}
