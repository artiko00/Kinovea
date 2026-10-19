use crate::camera_test::ShiftCamera;
use crate::drawing_tracker_test::{bound_drawing, track, FakeTrackable};
use crate::manager::{assign, TrackabilityRuntime};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use kinovea_metadata::trackability::TrackabilityManager;
use kinovea_metadata::Uuid;

/// A tracker read from a file: `a` bound to `track`, `b` unbound. Object tracking is not initialized.
fn loaded(drawing: Uuid, track: Uuid) -> TrackabilityManager {
    let xml = format!(
        "<Trackability><TrackableDrawing id=\"{drawing}\" name=\"Line\">\
         <TrackablePoint key=\"a\"><TrackId>{track}</TrackId><ReferenceTimestamp>0</ReferenceTimestamp><ReferenceValue>1;1</ReferenceValue></TrackablePoint>\
         <TrackablePoint key=\"b\"><ReferenceTimestamp>0</ReferenceTimestamp><ReferenceValue>2;2</ReferenceValue></TrackablePoint>\
         </TrackableDrawing></Trackability>"
    );
    let mut data = TrackabilityManager::default();
    data.read_kva(&Element::parse(&xml).unwrap());
    data
}

fn two_points() -> [(String, PointF); 2] {
    [
        ("a".to_owned(), PointF::new(1.0, 1.0)),
        ("b".to_owned(), PointF::new(2.0, 2.0)),
    ]
}

#[test]
fn assign_after_load_keeps_the_binding_and_resolves_through_the_track() {
    // DrawingTracker.cs:167-222 (Assign), :684-689 (GetPointAtTime through the track).
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let mut data = loaded(id, bound);
    let tracks = [track(bound, &[(0, 0.0, 0.0), (40, 7.0, 8.0)])];
    assert!(assign(&mut data, &tracks, id, "Line", 0, &two_points()));
    assert!(data.is_object_tracking_initialized(id));
    let runtime = TrackabilityRuntime::default();
    assert_eq!(
        runtime.point_at_time(&data, &tracks, id, "a", 40),
        PointF::new(7.0, 8.0)
    );
}

#[test]
fn assign_drops_the_missing_tracks_and_stays_initialized() {
    // DrawingTracker.cs:186-222: the missing tracks are dropped, isObjectTrackingInitialized stays true
    // (set at :222 from the count at :186). Then GetPointAtTime is EMPTY (:684-692) and ToggleTracking
    // still works (:330-336).
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let mut data = loaded(id, bound);
    assert!(assign(&mut data, &[], id, "Line", 0, &two_points()));
    assert_eq!(data.tracker(id).unwrap().points[0].1.track_id, None);
    assert!(data.is_object_tracking_initialized(id));
    let mut runtime = TrackabilityRuntime::default();
    assert_eq!(
        runtime.point_at_time(&data, &[], id, "a", 40),
        PointF::EMPTY
    );
    runtime.toggle_tracking(&data, id);
    assert!(runtime.is_tracking(id));
}

#[test]
fn assign_keeps_the_binding_while_the_track_exists() {
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let mut data = loaded(id, bound);
    let tracks = [track(bound, &[(0, 0.0, 0.0)])];
    assign(&mut data, &tracks, id, "Line", 0, &two_points());
    assert_eq!(data.tracker(id).unwrap().points[0].1.track_id, Some(bound));
}

#[test]
fn assign_without_bindings_leaves_object_tracking_off() {
    // DrawingTracker.cs:186: the flag is only set when the file had a binding.
    let id = Uuid::new_v4();
    let mut data = TrackabilityManager::default();
    data.add(id, "Line", 0, &two_points());
    assert!(assign(&mut data, &[], id, "Line", 0, &two_points()));
    assert!(!data.is_object_tracking_initialized(id));
}

#[test]
fn forgetting_the_last_track_stops_object_tracking_and_changes_the_hash() {
    // DrawingTracker.cs:374-376 and :385-402 (DeinitializeTracking), TrackabilityManager.cs:606-615.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let mut data = bound_drawing(id, bound);
    let mut runtime = TrackabilityRuntime::default();
    runtime.toggle_tracking(&data, id);
    assert!(runtime.is_tracking(id));
    assert!(runtime.any_tracking());
    let bound_hash = data.content_hash();
    runtime.forget_track(&mut data, bound);
    assert_ne!(data.content_hash(), bound_hash);
    assert!(!data.is_object_tracking_initialized(id));
    assert!(!runtime.is_tracking(id));
    assert!(!runtime.any_tracking());
}

#[test]
fn forgetting_is_ignored_before_object_tracking_starts() {
    // DrawingTracker.cs:352-357: ForgetTrack returns while object tracking is not initialized.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let mut data = loaded(id, bound);
    let mut runtime = TrackabilityRuntime::default();
    runtime.forget_track(&mut data, bound);
    assert_eq!(data.tracker(id).unwrap().points[0].1.track_id, Some(bound));
}

#[test]
fn toggling_needs_object_tracking() {
    // DrawingTracker.cs:330-334.
    let id = Uuid::new_v4();
    let mut unbound = TrackabilityManager::default();
    unbound.add(id, "Line", 0, &[("a".into(), PointF::new(1.0, 1.0))]);
    let mut runtime = TrackabilityRuntime::default();
    runtime.toggle_tracking(&unbound, id);
    assert!(!runtime.is_tracking(id));
    let bound = bound_drawing(id, Uuid::new_v4());
    runtime.toggle_tracking(&bound, id);
    assert!(runtime.is_tracking(id));
    runtime.toggle_tracking(&bound, id);
    assert!(!runtime.is_tracking(id));
}

#[test]
fn removing_a_drawing_forgets_its_tracker_and_changes_the_hash() {
    // TrackabilityManager.cs:197-205.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let mut data = bound_drawing(id, bound);
    let mut runtime = TrackabilityRuntime::default();
    runtime.toggle_tracking(&data, id);
    let before = data.content_hash();
    runtime.remove(&mut data, id);
    assert!(data.tracker(id).is_none());
    assert_ne!(data.content_hash(), before);
    assert!(!runtime.any_tracking());
}

#[test]
fn unknown_drawings_have_no_position_and_receive_nothing() {
    // TrackabilityManager.cs:385-398 (SanityCheck fails: EMPTY) and :337 (only registered trackers).
    let mut runtime = TrackabilityRuntime::default();
    let data = TrackabilityManager::default();
    assert_eq!(
        runtime.point_at_time(&data, &[], Uuid::nil(), "a", 0),
        PointF::EMPTY
    );
    let mut drawing = FakeTrackable::default();
    runtime.apply_at(&data, &[], Uuid::nil(), 0, &ShiftCamera::on(), &mut drawing);
    assert!(drawing.values.is_empty());
    assert_eq!(drawing.finished, 0);
}
