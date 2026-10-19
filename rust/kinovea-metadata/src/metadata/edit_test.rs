use crate::drawing::{Chrono, Drawing, Line};
use crate::keyframe::Keyframe;
use crate::kva_io::Filter;
use crate::metadata::edit::MultiItem;
use crate::metadata::{Manager, Metadata};
use crate::multi::{AutoNumber, Spotlight};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;

fn line(name: &str) -> Drawing {
    let mut d = Drawing::Line(Line::new(PointF::EMPTY, PointF::new(1.0, 2.0)));
    d.header_mut().name = name.to_owned();
    d
}

fn keyframe_with_line(m: &mut Metadata) -> (uuid::Uuid, uuid::Uuid) {
    let kf = Keyframe::new(250, "k");
    let kid = kf.id;
    m.merge_insert_keyframe(kf);
    m.add_drawing(Manager::Keyframe(kid), line("a"));
    m.register_trackables();
    (kid, m.keyframes[0].drawings[0].id())
}

#[test]
fn keyframe_drawing_takes_the_keyframe_time_as_reference() {
    let mut m = Metadata::default();
    let (kid, id) = keyframe_with_line(&mut m);
    let d = m.drawing(Manager::Keyframe(kid), id).unwrap();
    assert_eq!(d.header().reference_timestamp, 250);
    assert_eq!(d.header().fading.reference_timestamp, 250);
}

#[test]
fn delete_drawing_forgets_its_tracker_and_returns_the_drawing() {
    let mut m = Metadata::default();
    let (kid, id) = keyframe_with_line(&mut m);
    assert!(m.trackability.tracker(id).is_some());

    let removed = m.delete_drawing(Manager::Keyframe(kid), id);

    assert_eq!(removed.map(|d| d.id()), Some(id));
    assert!(m.keyframes[0].drawings.is_empty());
    assert!(m.trackability.tracker(id).is_none());
    assert!(m.delete_drawing(Manager::Keyframe(kid), id).is_none());
}

#[test]
fn chronos_are_found_and_edited_in_place_but_singletons_are_not_drawing_lists() {
    let mut m = Metadata::default();
    m.add_drawing(Manager::Chronos, Drawing::Chrono(Chrono::new(PointF::EMPTY)));
    let id = m.chronos[0].id();

    if let Some(d) = m.drawing_mut(Manager::Chronos, id) {
        d.header_mut().name = "renamed".to_owned();
    }

    assert_eq!(m.drawing(Manager::Chronos, id).map(|d| d.header().name.as_str()), Some("renamed"));
    assert!(m.drawing(Manager::Singletons, id).is_none());
    assert!(m.drawing(Manager::Tracks, id).is_none());
}

#[test]
fn delete_keyframe_forgets_the_trackers_of_its_drawings() {
    let mut m = Metadata::default();
    let (kid, id) = keyframe_with_line(&mut m);

    assert!(m.delete_keyframe(kid).is_some());

    assert!(m.keyframes.is_empty());
    assert!(m.trackability.tracker(id).is_none());
    assert!(m.delete_keyframe(kid).is_none());
}

#[test]
fn multi_items_are_removed_from_whichever_container_holds_them() {
    let mut m = Metadata::default();
    m.add_multi_item(MultiItem::Spotlight(Spotlight::new(5, PointF::EMPTY, 3)));
    m.add_multi_item(MultiItem::AutoNumber(AutoNumber::new(6, PointF::EMPTY, 1)));
    let spot = m.spotlights.items[0].id;
    let number = m.number_sequence.items[0].id;

    assert_eq!(m.delete_multi_item(number).map(|i| i.id()), Some(number));
    assert!(m.number_sequence.items.is_empty());
    assert!(matches!(m.delete_multi_item(spot), Some(MultiItem::Spotlight(_))));
    assert!(m.delete_multi_item(spot).is_none());
}

#[test]
fn multi_item_element_round_trips() {
    let item = MultiItem::AutoNumber(AutoNumber::new(6, PointF::new(3.0, 4.0), 7));
    let e = item.to_element(Filter::KVA);
    let read = MultiItem::read_kva(&e);
    assert_eq!(read.map(|i| i.to_element(Filter::KVA)), Some(e));
    assert_eq!(MultiItem::read_kva(&Element::new("Unknown")), None);
}
