use super::dom::Element;
use super::general::GeneralPreferences;
use super::helpers::XmlField;

#[test]
fn defaults_match_the_original() {
    let g = GeneralPreferences::default();
    assert_eq!(g.culture, "");
    assert!(!g.enable_debug_log && !g.enable_all_languages);
    assert_eq!(g.preference_page, 0);
    assert_eq!(g.pointer_key, "::default");
}

#[test]
fn unknown_elements_are_ignored() {
    let xml = "<General><Culture>es</Culture><FutureSetting>x</FutureSetting><PreferencesPage>2</PreferencesPage></General>";
    let mut g = GeneralPreferences::default();
    g.read(&Element::parse(xml).unwrap());
    assert_eq!(g.culture, "es");
    assert_eq!(g.preference_page, 2);
    let names: Vec<_> = g
        .write("General")
        .unwrap()
        .children
        .iter()
        .map(|c| c.name.clone())
        .collect();
    assert_eq!(
        names,
        [
            "Culture",
            "EnableDebugLog",
            "EnableAllLanguages",
            "PreferencesPage",
            "Pointer"
        ]
    );
}
