use crate::trackability::TrackabilityManager;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const BLOCK: &str = "<Trackability>\n  <TrackableDrawing id=\"00000000-0000-4000-8000-000000000001\" name=\"Line 1\">\n    <TrackablePoint key=\"a\">\n      <TrackId>00000000-0000-4000-8000-000000000009</TrackId>\n      <ReferenceTimestamp>10</ReferenceTimestamp>\n      <ReferenceValue>1;2</ReferenceValue>\n    </TrackablePoint>\n    <TrackablePoint key=\"b\">\n      <ReferenceTimestamp>10</ReferenceTimestamp>\n      <ReferenceValue>3;4</ReferenceValue>\n    </TrackablePoint>\n  </TrackableDrawing>\n</Trackability>";

#[test]
fn block_round_trips_and_reference_values_resolve() {
    let mut m = TrackabilityManager::default();
    m.read_kva(&Element::parse(BLOCK).unwrap());
    let id = Uuid::parse_str("00000000-0000-4000-8000-000000000001").unwrap();
    assert_eq!(m.reference_value(id, "b"), PointF::new(3.0, 4.0));
    assert_eq!(m.reference_value(id, "zz"), PointF::EMPTY);
    assert_eq!(m.reference_value(Uuid::nil(), "a"), PointF::EMPTY);
    assert!(m.tracker(id).unwrap().points[0].1.track_id.is_some());
    let mut e = Element::new("Trackability");
    e.children = m.write_kva();
    let doc = e.to_document();
    assert_eq!(&doc[doc.find('\n').unwrap() + 1..], BLOCK);
}

#[test]
fn add_replaces_in_place_and_keeps_track_bindings() {
    let mut m = TrackabilityManager::default();
    m.read_kva(&Element::parse(BLOCK).unwrap());
    let id = Uuid::parse_str("00000000-0000-4000-8000-000000000001").unwrap();
    let other = Uuid::new_v4();
    m.add(other, "Other", 0, &[("0".into(), PointF::new(9.0, 9.0))]);
    m.add(id, "Line 1", 20, &[("a".into(), PointF::new(5.0, 6.0)), ("b".into(), PointF::new(7.0, 8.0))]);
    // Re-registration keeps the position.
    assert_eq!(order(&m), [id, other]);
    assert_eq!(m.reference_value(id, "a"), PointF::new(5.0, 6.0));
    let first = m.trackers.iter().next().unwrap();
    assert!(first.points[0].1.track_id.is_some());
    assert_eq!(first.points[0].1.reference_timestamp, 20);
    m.remove(id);
    assert_eq!(order(&m), vec![other]);
    assert_eq!(m.reference_value(other, "0"), PointF::new(9.0, 9.0));
}

fn order(m: &TrackabilityManager) -> Vec<Uuid> {
    m.trackers.keys().collect()
}

fn written_names(m: &TrackabilityManager) -> Vec<String> {
    let written = m.write_kva();
    written
        .iter()
        .map(|e| e.attribute("name").unwrap().to_owned())
        .collect::<Vec<_>>()
}

fn ids<const N: usize>() -> [Uuid; N] {
    std::array::from_fn(|_| Uuid::new_v4())
}

fn point(v: f32) -> Vec<(String, PointF)> {
    vec![("0".into(), PointF::new(v, v))]
}

#[test]
fn update_id_to_a_known_id_drops_the_old_entry_and_frees_its_slot() {
    // `UpdateId` with the new id already registered: `Remove(oldId)` only. The next `Add` takes that slot.
    let [old, angle, new, late] = ids();
    let mut m = TrackabilityManager::default();
    m.add(old, "Coordinate system 1", 0, &point(1.0));
    m.add(angle, "Angle 1", 0, &point(2.0));
    m.add(new, "Coordinate system 1", 0, &point(3.0));
    m.update_id(old, new);
    assert_eq!(order(&m), vec![angle, new]);
    m.add(late, "Late", 0, &point(4.0));
    assert_eq!(order(&m), vec![late, angle, new]);
}

#[test]
fn update_id_to_an_unknown_id_adds_the_new_entry_before_removing_the_old_one() {
    // `UpdateId` with a new id: `Add(newId, trackers[oldId])` then `Remove(oldId)` (TrackabilityManager.cs:130-134).
    // The new entry takes the free slot left by an earlier removal, and the old slot is freed afterwards.
    // The copy keeps its `drawingId` (DrawingTracker.cs:130 only sets it in the constructors and `Reassign`),
    // so `WriteXml` (TrackabilityManager.cs:403-420) finds no tracker for that id and does not write it.
    let [a, old, b, new, late] = ids();
    let track = Uuid::new_v4();
    let mut m = TrackabilityManager::default();
    m.add(a, "A", 0, &point(1.0));
    m.add(old, "Coordinate system 1", 0, &point(2.0));
    m.add(b, "B", 0, &point(3.0));
    m.trackers.get_mut(old).unwrap().points[0].1.track_id = Some(track);
    m.remove(a);
    m.update_id(old, new);
    assert_eq!(order(&m), vec![new, b]);
    assert!(m.tracker(old).is_none());
    let bound = m.tracker(new).unwrap().points[0].1.track_id;
    assert_eq!(bound, Some(track), "the bindings move with the tracker");
    let filed = m.tracker(new).unwrap().drawing_id;
    assert_eq!(filed, old, "the drawing id is not changed");
    assert_eq!(written_names(&m), ["B"], "the stale copy is not written");
    m.add(late, "Late", 0, &point(4.0));
    assert_eq!(order(&m), vec![new, late, b]);
}

#[test]
fn update_id_ignores_the_same_id_and_unknown_ids() {
    let (known, other) = (Uuid::new_v4(), Uuid::new_v4());
    let mut m = TrackabilityManager::default();
    m.add(known, "Known", 0, &point(1.0));
    m.update_id(known, known);
    m.update_id(other, Uuid::new_v4());
    assert_eq!(order(&m), vec![known]);
}
