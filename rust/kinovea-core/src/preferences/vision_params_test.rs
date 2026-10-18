use super::dom::Element;
use super::helpers::{Color, XmlField};
use super::vision_params::*;

#[test]
fn defaults_match_the_original_file() {
    let presets = KeyframePresets::default();
    assert_eq!(presets.0.len(), 10);
    assert_eq!(presets.0[9].color, Color::rgb(152, 152, 152));
    let cm = CameraMotionParameters::default()
        .write("CameraMotion")
        .unwrap();
    assert_eq!(cm.child_text("RansacReprojThreshold"), Some("1.25"));
    assert_eq!(
        LensCalibrationParameters::default()
            .write("L")
            .unwrap()
            .child_text("PatternSize"),
        Some("10;7")
    );
}

#[test]
fn presets_replace_defaults_in_order() {
    let xml = "<KeyframePresets><KeyframePreset><Name>Start</Name><Color>1;2;3</Color></KeyframePreset></KeyframePresets>";
    let mut p = KeyframePresets::default();
    p.read(&Element::parse(xml).unwrap());
    assert_eq!(p.0.len(), 10);
    assert_eq!(p.0[0].name, "Start");
    assert_eq!(p.0[1].color, Color::rgb(51, 255, 255));
}
