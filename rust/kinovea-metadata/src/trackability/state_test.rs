use crate::trackability::TrackabilityManager;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

fn two_points(m: &mut TrackabilityManager, id: Uuid) {
    m.add(
        id,
        "Line",
        0,
        &[
            ("a".into(), PointF::new(1.0, 2.0)),
            ("b".into(), PointF::new(3.0, 4.0)),
        ],
    );
}

#[test]
fn second_binding_does_not_change_the_hash() {
    // DrawingTracker.cs:83-94: the hash covers the flag, not the track ids.
    let mut m = TrackabilityManager::default();
    let id = Uuid::new_v4();
    two_points(&mut m, id);
    m.bind_track(id, "a", Uuid::new_v4());
    let bound = m.content_hash();
    m.bind_track(id, "b", Uuid::new_v4());
    assert_eq!(m.content_hash(), bound);
}

#[test]
fn binding_and_forgetting_change_the_hash() {
    let mut m = TrackabilityManager::default();
    let id = Uuid::new_v4();
    two_points(&mut m, id);
    let track = Uuid::new_v4();
    let unbound = m.content_hash();
    m.bind_track(id, "a", track);
    assert_ne!(m.content_hash(), unbound);
    m.forget_track(track);
    assert_eq!(m.content_hash(), unbound);
}

#[test]
fn reference_values_change_the_hash() {
    let mut m = TrackabilityManager::default();
    let id = Uuid::new_v4();
    two_points(&mut m, id);
    let before = m.content_hash();
    m.add(id, "Line", 0, &[("a".into(), PointF::new(9.0, 2.0))]);
    assert_ne!(m.content_hash(), before);
}

#[test]
fn object_tracking_is_initialized_while_some_point_is_bound() {
    let mut m = TrackabilityManager::default();
    let id = Uuid::new_v4();
    two_points(&mut m, id);
    assert!(!m.is_object_tracking_initialized(id));
    let track = Uuid::new_v4();
    m.bind_track(id, "b", track);
    assert!(m.is_object_tracking_initialized(id));
    m.forget_track(track);
    assert!(!m.is_object_tracking_initialized(id));
    assert!(!m.is_object_tracking_initialized(Uuid::nil()));
}

#[test]
fn bind_ignores_unknown_drawings_and_keys() {
    let mut m = TrackabilityManager::default();
    let id = Uuid::new_v4();
    two_points(&mut m, id);
    let before = m.content_hash();
    m.bind_track(Uuid::nil(), "a", Uuid::new_v4());
    m.bind_track(id, "zz", Uuid::new_v4());
    assert_eq!(m.content_hash(), before);
    assert!(!m.is_object_tracking_initialized(id));
}

/// `Trackability` as the original writes it when no tracker started object tracking: the hash does
/// not cover `TrackId` alone (`DrawingTracker.cs:89` hashes the flag, which KVA does not store).
#[test]
fn hash_survives_a_write_and_read_back() {
    let text = "<Trackability><TrackableDrawing id=\"00000000-0000-4000-8000-000000000001\" name=\"Line\"><TrackablePoint key=\"a\"><TrackId>00000000-0000-4000-8000-000000000009</TrackId><ReferenceTimestamp>0</ReferenceTimestamp><ReferenceValue>1.5;2</ReferenceValue></TrackablePoint></TrackableDrawing><TrackableDrawing id=\"00000000-0000-4000-8000-000000000002\" name=\"Other\"><TrackablePoint key=\"o\"><ReferenceTimestamp>0</ReferenceTimestamp><ReferenceValue>3;4</ReferenceValue></TrackablePoint></TrackableDrawing></Trackability>";
    let mut m = TrackabilityManager::default();
    m.read_kva(&Element::parse(text).unwrap());
    let mut e = Element::new("Trackability");
    e.children = m.write_kva();
    let mut back = TrackabilityManager::default();
    back.read_kva(&Element::parse(&e.to_document()).unwrap());
    assert_eq!(back, m);
    assert_eq!(back.content_hash(), m.content_hash());
}

#[test]
fn forgetting_is_ignored_until_object_tracking_starts() {
    // DrawingTracker.cs:352-357: ForgetTrack returns while object tracking is not initialized.
    let text = "<Trackability><TrackableDrawing id=\"00000000-0000-4000-8000-000000000001\" name=\"Line\"><TrackablePoint key=\"a\"><TrackId>00000000-0000-4000-8000-000000000009</TrackId><ReferenceTimestamp>0</ReferenceTimestamp><ReferenceValue>1;2</ReferenceValue></TrackablePoint></TrackableDrawing></Trackability>";
    let mut m = TrackabilityManager::default();
    m.read_kva(&Element::parse(text).unwrap());
    m.forget_track(Uuid::parse_str("00000000-0000-4000-8000-000000000009").unwrap());
    assert!(m.tracker(m.trackers[0].drawing_id).unwrap().points[0]
        .1
        .track_id
        .is_some());
}

#[test]
fn unbinding_keeps_object_tracking_initialized() {
    // DrawingTracker.cs:186-222: Assign drops the missing tracks without deinitializing.
    let mut m = TrackabilityManager::default();
    let id = Uuid::new_v4();
    two_points(&mut m, id);
    let track = Uuid::new_v4();
    m.bind_track(id, "a", track);
    m.unbind_track(track);
    assert!(m.is_object_tracking_initialized(id));
    assert!(m
        .tracker(id)
        .unwrap()
        .points
        .iter()
        .all(|(_, p)| p.track_id.is_none()));
}
