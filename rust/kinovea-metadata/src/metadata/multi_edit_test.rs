use crate::kva_io::Filter;
use crate::metadata::edit::MultiItem;
use crate::metadata::Metadata;
use crate::multi::{AutoNumber, Spotlight};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;

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
fn delete_multi_item_forgets_its_tracker() {
    let mut m = Metadata::default();
    m.add_multi_item(MultiItem::Spotlight(Spotlight::new(5, PointF::EMPTY, 3)));
    m.register_spotlights();
    let spot = m.spotlights.items[0].id;
    assert!(m.trackability.tracker(spot).is_some());

    m.delete_multi_item(spot);

    assert!(m.trackability.tracker(spot).is_none());
}

#[test]
fn multi_item_lookup_finds_items_without_removing_them() {
    let mut m = Metadata::default();
    m.add_multi_item(MultiItem::AutoNumber(AutoNumber::new(6, PointF::EMPTY, 1)));
    let number = m.number_sequence.items[0].id;
    assert_eq!(m.multi_item(number).map(|i| i.id()), Some(number));
    assert_eq!(m.number_sequence.items.len(), 1);
}

#[test]
fn multi_item_element_round_trips() {
    let item = MultiItem::AutoNumber(AutoNumber::new(6, PointF::new(3.0, 4.0), 7));
    let e = item.to_element(Filter::KVA);
    let read = MultiItem::read_kva(&e);
    assert_eq!(read.map(|i| i.to_element(Filter::KVA)), Some(e));
    assert_eq!(MultiItem::read_kva(&Element::new("Unknown")), None);
}
