//! Shared children of the KVA elements: the drawing style, the fading block and the measure
//! label pair.

use super::gen::{leaf, node, setting, Rng, FONT_SIZES, MEASURES, SIZES};
use kinovea_core::preferences::dom::Element;

pub fn style(r: &mut Rng) -> Element {
    let mut items = Vec::new();
    if r.chance() {
        items.push(setting("Color", "color", r.color()));
    }
    if r.chance() {
        items.push(setting("Color", "back color", r.color()));
    }
    if r.chance() {
        items.push(setting("LineSize", "line size", r.pick(&SIZES)));
    }
    if r.chance() {
        items.push(setting("FontSize", "font size", r.pick(&FONT_SIZES)));
    }
    if r.chance() {
        items.push(setting("PenSize", "pen size", r.pick(&SIZES)));
    }
    if r.chance() {
        items.push(setting("PenShape", "pen shape", r.pick(&["Solid", "Dash"])));
    }
    node("DrawingStyle", items)
}

pub fn fading(r: &mut Rng) -> Element {
    node(
        "InfosFading",
        vec![
            leaf("Enabled", r.flag()),
            leaf("UseDefault", r.flag()),
            leaf("AlwaysVisible", r.flag()),
            leaf("OpaqueFrames", r.int(0, 9)),
            leaf("Frames", r.int(0, 30)),
        ],
    )
}

/// The measure label pair of the line-like drawings.
pub fn measure(r: &mut Rng) -> Vec<Element> {
    let label = node(
        "MeasureLabel",
        vec![
            leaf("SpacePosition", r.point()),
            leaf("TimePosition", r.int(0, 9000)),
        ],
    );
    vec![leaf("ExtraData", r.pick(&MEASURES)), label]
}

/// The `DrawingStyle` and `InfosFading` children every attached drawing shares.
pub fn attached(r: &mut Rng) -> Vec<Element> {
    vec![style(r), fading(r)]
}
