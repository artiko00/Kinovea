use crate::drawing_tracker_test::track;
use crate::manager::{assign, TrackabilityRuntime};
use kinovea_geometry::PointF;
use kinovea_metadata::trackability::TrackabilityManager;
use kinovea_metadata::Uuid;

#[test]
fn initialize_tracking_starts_object_tracking_and_the_tracking_flag() {
    // DrawingTracker.cs:300-305: InitializeTracking sets both isObjectTrackingInitialized and
    // isCurrentlyTracking.
    let id = Uuid::new_v4();
    let mut data = TrackabilityManager::default();
    data.add(id, "Line", 0, &[("a".into(), PointF::new(1.0, 1.0))]);
    let mut runtime = TrackabilityRuntime::default();
    runtime.initialize_tracking(&mut data, id);
    assert!(data.is_object_tracking_initialized(id));
    assert!(runtime.is_tracking(id));
    assert!(runtime.any_tracking());
}

#[test]
fn assign_starts_object_tracking_without_the_tracking_flag() {
    // DrawingTracker.cs:186-222: Assign of a loaded tracker leaves isCurrentlyTracking untouched.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let mut data = TrackabilityManager::default();
    data.add(id, "Line", 0, &[("a".into(), PointF::new(1.0, 1.0))]);
    data.bind_track(id, "a", bound);
    let tracks = [track(bound, &[(0, 0.0, 0.0)])];
    let runtime = TrackabilityRuntime::default();
    assert!(assign(
        &mut data,
        &tracks,
        id,
        "Line",
        0,
        &[("a".into(), PointF::new(1.0, 1.0))]
    ));
    assert!(data.is_object_tracking_initialized(id));
    assert!(!runtime.is_tracking(id));
}
