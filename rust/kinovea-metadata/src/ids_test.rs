use crate::ids::{id_text, new_id, normalize_ids, parse_id};
use uuid::Uuid;

#[test]
fn text_form_matches_guid_to_string() {
    let id = parse_id("11111111-2222-3333-4444-555555555555").unwrap();
    assert_eq!(id_text(id), "11111111-2222-3333-4444-555555555555");
    assert_eq!(id_text(Uuid::nil()), "00000000-0000-0000-0000-000000000000");
    assert_eq!(parse_id(" 11111111-2222-3333-4444-555555555555 "), Some(id));
    assert_eq!(parse_id("nope"), None);
    assert_ne!(new_id(), new_id());
}

#[test]
fn normalization_follows_order_of_first_appearance() {
    let text = "<A id=\"d3b07384-d9a0-4c1e-9a8b-000000000001\"/><B ref=\"D3B07384-D9A0-4C1E-9A8B-000000000001\"/><C id=\"7c9e6679-7425-40de-944b-e07fc1f90ae7\"/><Z>00000000-0000-0000-0000-000000000000</Z>";
    let want = "<A id=\"00000000-0000-4000-8000-000000000001\"/><B ref=\"00000000-0000-4000-8000-000000000001\"/><C id=\"00000000-0000-4000-8000-000000000002\"/><Z>00000000-0000-0000-0000-000000000000</Z>";
    assert_eq!(normalize_ids(text), want);
    assert_eq!(normalize_ids("no ids here é"), "no ids here é");
}
