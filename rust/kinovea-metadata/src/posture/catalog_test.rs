use crate::posture::catalog::{custom_postures, posture_by_id, CUSTOM_XML};
use uuid::Uuid;

#[test]
fn all_shipped_tools_parse_with_a_unique_id() {
    assert_eq!(CUSTOM_XML.len(), 16);
    let postures = custom_postures();
    assert_eq!(postures.len(), 16);
    let mut ids: Vec<Uuid> = postures.iter().map(|p| p.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 16, "ids are unique");
    assert!(postures.iter().all(|p| !p.point_list.is_empty() && !p.name.is_empty()));
}

#[test]
fn lookup_by_id_returns_a_fresh_instance() {
    let id = Uuid::parse_str("02a52d37-4694-4b90-a6d4-58d2a399638c").unwrap();
    let bike = posture_by_id(id, true).unwrap();
    assert_eq!(bike.name, "Bikefit");
    assert!(bike.from_kva);
    assert!(posture_by_id(Uuid::nil(), false).is_none());
}
