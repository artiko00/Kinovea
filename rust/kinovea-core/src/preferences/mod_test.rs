use super::Preferences;

#[test]
fn default_preferences_have_no_keyboard_section() {
    let p = Preferences::default();
    assert!(p.keyboard.is_none());
    assert_eq!(p.general.pointer_key, "::default");
    assert_eq!(p.to_element().children[0].name, "FormatVersion");
}
