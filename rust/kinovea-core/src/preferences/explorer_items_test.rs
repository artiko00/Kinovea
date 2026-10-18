use super::dom::Element;
use super::enums::FileProperty;
use super::explorer_items::*;
use super::helpers::XmlField;

#[test]
fn visibility_defaults_and_updates() {
    let mut v = FilePropertyVisibility::default();
    assert_eq!(v.visible.len(), 5);
    assert_eq!(v.visible[3], (FileProperty::CreationTime, false));
    v.read(
        &Element::parse("<V><CreationTime>true</CreationTime><IsImage>true</IsImage></V>").unwrap(),
    );
    assert!(v.visible[3].1);
    assert_eq!(v.visible.len(), 5);
}

#[test]
fn favorite_folder_fields() {
    let mut f = FavoriteFolder::default();
    f.read_fields(
        &Element::parse("<S><FriendlyName>Vids</FriendlyName><Location>/v</Location></S>").unwrap(),
    );
    assert_eq!(
        (f.friendly_name.as_str(), f.location.as_str()),
        ("Vids", "/v")
    );
}
