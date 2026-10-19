//! Tests of the shared children: the style, fading and measure label shapes.

use super::gen::Rng;
use super::parts::{attached, fading, measure, style};
use kinovea_core::preferences::dom::Element;

fn names(items: &[Element]) -> Vec<&str> {
    items.iter().map(|e| e.name.as_str()).collect()
}

#[test]
fn fading_has_the_five_settings() {
    let f = fading(&mut Rng::new(1));
    assert_eq!(f.name, "InfosFading");
    assert_eq!(
        names(&f.children),
        [
            "Enabled",
            "UseDefault",
            "AlwaysVisible",
            "OpaqueFrames",
            "Frames"
        ]
    );
}

#[test]
fn measure_is_extra_data_then_label() {
    assert_eq!(
        names(&measure(&mut Rng::new(2))),
        ["ExtraData", "MeasureLabel"]
    );
}

#[test]
fn attached_is_style_then_fading() {
    assert_eq!(
        names(&attached(&mut Rng::new(3))),
        ["DrawingStyle", "InfosFading"]
    );
}

#[test]
fn style_settings_carry_a_key() {
    for seed in 0..50 {
        for item in &style(&mut Rng::new(seed)).children {
            assert!(
                item.attribute("Key").is_some(),
                "seed {seed}: {}",
                item.name
            );
        }
    }
}
