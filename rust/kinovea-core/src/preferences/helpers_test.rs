use super::dom::Element;
use super::enums::ImageFormat;
use super::helpers::*;

fn roundtrip<T: XmlField + Default>(text: &str) -> (T, String) {
    let mut v = T::default();
    v.read(&Element::leaf("X", text));
    let out = v.write("X").unwrap().text;
    (v, out)
}

#[test]
fn boolean_rule_matches_xml_helper() {
    assert!(parse_bool("true") && parse_bool("True") && parse_bool("1") && parse_bool("yes"));
    assert!(!parse_bool("false") && !parse_bool("False") && !parse_bool("0"));
    assert_eq!(true.write("B").unwrap().text, "true");
}

#[test]
fn encodings_round_trip() {
    assert_eq!(roundtrip::<Color>("44;44;44").1, "44;44;44");
    assert_eq!(roundtrip::<Color>("255;10;20;30").0, Color::rgb(10, 20, 30));
    assert_eq!(roundtrip::<Size>("100;100").1, "100;100");
    assert_eq!(roundtrip::<PointF>("1.5;-2").1, "1.5;-2");
    assert_eq!(roundtrip::<Fixed3>("25").1, "25.000");
    assert_eq!(roundtrip::<f32>("0.5").1, "0.5");
    assert_eq!(roundtrip::<i32>("12").0, 12);
    let mut fmt = ImageFormat::JPG;
    fmt.read(&Element::leaf("X", "PNG"));
    assert_eq!(fmt, ImageFormat::PNG);
}

#[test]
fn invalid_text_keeps_current_value() {
    let mut v = ImageFormat::JPG;
    v.read(&Element::leaf("X", "WEBP"));
    assert_eq!(v, ImageFormat::JPG);
    let mut n = 7i32;
    n.read(&Element::leaf("X", "abc"));
    assert_eq!(n, 7);
}
