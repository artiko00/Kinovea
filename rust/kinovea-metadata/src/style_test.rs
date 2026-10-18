use crate::color::Color;
use crate::style::{StyleElements, StyleValue};
use kinovea_core::preferences::dom::Element;

const BLOCK: &str = "<DrawingStyle>\n  <Color Key=\"color\">\n    <Value>255;144;238;144</Value>\n  </Color>\n  <LineSize Key=\"line size\">\n    <Value>2</Value>\n  </LineSize>\n  <Mystery Key=\"m\">\n    <Thing>1</Thing>\n  </Mystery>\n</DrawingStyle>";

#[test]
fn block_round_trips_and_keeps_unknown_elements() {
    let e = Element::parse(BLOCK).unwrap();
    let style = StyleElements::read_kva(&e);
    assert_eq!(style.elements.len(), 3);
    assert!(matches!(style.get("m"), Some(StyleValue::Unknown(_))));
    let mut out = Element::new("DrawingStyle");
    out.children = style.write_kva();
    let mut doc = out.to_document();
    doc.drain(..doc.find('\n').unwrap() + 1);
    assert_eq!(doc, BLOCK);
}

#[test]
fn import_values_only_touches_known_keys() {
    let mut style = StyleElements::new(vec![("color", StyleValue::Color(Color::BLACK)), ("line size", StyleValue::LineSize(3))]);
    let other = StyleElements::read_kva(&Element::parse(BLOCK).unwrap());
    style.import_values(&other);
    assert_eq!(style.get("color"), Some(&StyleValue::Color(Color::rgb(144, 238, 144))));
    assert_eq!(style.get("line size"), Some(&StyleValue::LineSize(2)));
    assert!(!style.contains("m"));
    style.import_kva(&Element::parse("<DrawingStyle><LineSize Key=\"line size\"><Value>8</Value></LineSize></DrawingStyle>").unwrap());
    assert_eq!(style.get("line size"), Some(&StyleValue::LineSize(8)));
}

#[test]
fn insert_remove_complete_memorize_restore() {
    let mut style = StyleElements::new(vec![("color", StyleValue::Color(Color::BLACK))]);
    style.insert("size", StyleValue::int(4));
    style.insert("color", StyleValue::Color(Color::rgb(1, 1, 1)));
    assert_eq!(style.elements.len(), 2);
    let preset = StyleElements::new(vec![("color", StyleValue::Color(Color::BLACK)), ("extra", StyleValue::FontSize(10))]);
    style.complete_from(&preset);
    assert_eq!(style.elements.iter().map(|e| e.key.as_str()).collect::<Vec<_>>(), ["color", "size", "extra"]);
    let hash = style.content_hash();
    style.memorize();
    style.insert("size", StyleValue::int(9));
    style.insert("color", StyleValue::Color(Color::rgb(9, 9, 9)));
    assert_eq!(style.remove("extra"), Some(StyleValue::FontSize(10)));
    assert_ne!(style.content_hash(), hash);
    style.restore();
    assert_eq!(style.content_hash(), hash);
    assert_eq!(style.get("size"), Some(&StyleValue::int(4)));
    assert_eq!(style.remove("nope"), None);
}
