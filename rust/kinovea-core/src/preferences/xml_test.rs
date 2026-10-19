use super::dom::Element;
use super::Preferences;
use kinovea_fixtures::fixtures_root;
use std::collections::HashMap;

fn fixture(case: &str) -> Element {
    let path = fixtures_root()
        .join("preferences")
        .join(case)
        .join("Preferences.xml");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    Element::parse(&text).unwrap()
}

/// Canonical form: children sorted, GUIDs numbered by first appearance, capture paths made
/// host independent (the oracle replaces the Documents folder by `{MyDocuments}`).
fn canonical(e: &Element, ids: &mut HashMap<String, String>) -> Element {
    let mut text = e.text.clone();
    if e.name == "Path" {
        text = text
            .replace('\\', "/")
            .replace(&super::capture::documents_dir(), "{MyDocuments}");
    }
    if e.name == "Id" && text != "00000000-0000-0000-0000-000000000000" {
        let n = ids.len() + 1;
        text = ids
            .entry(text)
            .or_insert_with(|| format!("00000000-0000-4000-8000-{n:012}"))
            .clone();
    }
    let mut children: Vec<Element> = e.children.iter().map(|c| canonical(c, ids)).collect();
    children.sort_by(|a, b| (&a.name, &a.text).cmp(&(&b.name, &b.text)));
    Element {
        name: e.name.clone(),
        attributes: e.attributes.clone(),
        text,
        children,
        end_tag: e.end_tag,
    }
}

fn assert_same_document(ours: &Element, oracle: &Element) {
    let a = canonical(ours, &mut HashMap::new()).to_document();
    let b = canonical(oracle, &mut HashMap::new()).to_document();
    let diff: Vec<String> = a
        .lines()
        .zip(b.lines())
        .filter(|(x, y)| x != y)
        .map(|(x, y)| format!("ours: {x}\n  oracle: {y}"))
        .collect();
    assert!(
        diff.is_empty() && a.lines().count() == b.lines().count(),
        "{}\n(lines {} vs {})",
        diff.join("\n"),
        a.lines().count(),
        b.lines().count()
    );
}

#[test]
fn defaults_match_the_oracle_file() {
    let mut prefs = Preferences::default();
    let oracle = fixture("defaults");
    // The oracle always writes the Keyboard section; it is opaque to this model.
    prefs.keyboard = oracle.child("Keyboard").cloned();
    assert_same_document(&prefs.to_element(), &oracle);
}

#[test]
fn modified_file_round_trips_and_drops_unknown_elements() {
    let input_path = fixtures_root().join("../cases/preferences/modified.xml");
    let input = std::fs::read_to_string(&input_path).unwrap();
    let prefs = Preferences::from_xml(&input, "modified.xml").unwrap();
    assert_eq!(prefs.general.culture, "es");
    assert_eq!(prefs.player.decimal_places, 3);
    assert_eq!(
        prefs.player.timecode_format,
        crate::timecode::TimecodeFormat::Frames
    );
    assert_eq!(prefs.file_explorer.recent_files.items.len(), 2);
    assert_eq!(prefs.capture.memory_buffer, 1024);

    let mut written = prefs.to_element();
    let oracle = fixture("modified");
    written.children.extend(oracle.child("Keyboard").cloned());
    assert!(written
        .child("General")
        .unwrap()
        .child("FutureSetting")
        .is_none());
    assert!(written.child("FutureSection").is_none());
    assert_same_document(&written, &oracle);
}

#[test]
fn missing_file_yields_defaults_and_set_does_not_persist() {
    let dir = std::env::temp_dir().join(format!("kinovea-prefs-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("Preferences.xml");
    let mut prefs = Preferences::load(&path).unwrap();
    assert_eq!(prefs.player.decimal_places, 2);
    prefs.player.decimal_places = 4;
    assert!(!path.exists());
    prefs.save(&path).unwrap();
    let again = Preferences::load(&path).unwrap();
    assert_eq!(again.player.decimal_places, 4);
    assert!(again.to_xml().starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<KinoveaPreferences>\n  <FormatVersion>3.0</FormatVersion>"));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn wrong_root_is_an_error() {
    let err = Preferences::from_xml("<Other />", "x.xml").unwrap_err();
    assert!(err.to_string().contains("expected KinoveaPreferences"));
}
