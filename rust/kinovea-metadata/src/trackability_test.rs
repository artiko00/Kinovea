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
    assert_eq!(m.trackers.len(), 2);
    assert_eq!(m.trackers[0].drawing_id, id, "re-registration keeps the position");
    assert_eq!(m.reference_value(id, "a"), PointF::new(5.0, 6.0));
    assert!(m.trackers[0].points[0].1.track_id.is_some());
    assert_eq!(m.trackers[0].points[0].1.reference_timestamp, 20);
    m.remove(id);
    assert_eq!(m.trackers.len(), 1);
    assert_eq!(m.reference_value(other, "0"), PointF::new(9.0, 9.0));
}
