use super::dom::Element;
use super::helpers::XmlField;
use super::kinogram::*;

#[test]
fn defaults_match_the_original_file() {
    let k = KinogramParameters::default().write("Kinogram").unwrap();
    assert_eq!(k.child_text("BorderColor"), Some("44;44;44"));
    assert_eq!(k.child("CropPositions").unwrap().children.len(), 0);
    assert_eq!(k.child_text("MeasureLabelType"), Some("None"));
    assert_eq!(
        ExportProfile::default()
            .write("ExportProfile")
            .unwrap()
            .child_text("GOPSize"),
        Some("12")
    );
}

#[test]
fn crop_positions_round_trip() {
    let xml = "<Kinogram><CropPositions><CropPosition anchor=\"true\" offset=\"3\">1.5;2</CropPosition></CropPositions></Kinogram>";
    let mut k = KinogramParameters::default();
    k.read(&Element::parse(xml).unwrap());
    assert_eq!(k.crop_positions.0.len(), 1);
    assert!(k.crop_positions.0[0].anchor);
    assert_eq!(k.crop_positions.0[0].offset, 3);
    let out = k.write("Kinogram").unwrap();
    let cp = &out.child("CropPositions").unwrap().children[0];
    assert_eq!(cp.attribute("offset"), Some("3"));
    assert_eq!(cp.text, "1.5;2");
}
