use super::fading::*;
use super::helpers::XmlField;

#[test]
fn defaults_match_the_original_file() {
    let t = TrackingParameters::default()
        .write("TrackingParameters")
        .unwrap();
    assert_eq!(t.child_text("SearchWindow"), Some("100;100"));
    assert_eq!(t.child_text("SimilarityThreshold"), Some("0.5"));
    assert_eq!(t.child_text("HSVRange"), Some("0;360;0;255;0;255"));
    assert_eq!(
        InfosFading::default()
            .write("InfoFading")
            .unwrap()
            .child_text("MasterFactor"),
        Some("1")
    );
}

#[test]
fn hsv_range_round_trip() {
    let mut r = HsvRange::default();
    r.read(&super::dom::Element::leaf("HSVRange", "10;20;30;40;50;60"));
    assert_eq!(r.hue, (10.0, 20.0));
    assert_eq!(r.write("HSVRange").unwrap().text, "10;20;30;40;50;60");
}
