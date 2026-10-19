//! Seeded generators for the media drawings (bitmaps, SVG images) and the generic posture.

use super::gen::{drawing, leaf, node, Rng};
use super::parts::{attached, fading};
use kinovea_core::preferences::dom::Element;

/// A file that exists, so the SVG survives the keyframe validity check.
const SVG_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");

/// Tool of four points, the posture the oracle uses for generic postures.
const POSTURE_TOOL: &str = "02ea93f9-ba00-424b-8c87-07e156e0252f";

pub fn bitmap(r: &mut Rng) -> Element {
    let bounding_box = format!(
        "{};{};{};{}",
        r.int(0, 600),
        r.int(0, 400),
        r.int(8, 128),
        r.int(8, 128)
    );
    let mut c = vec![
        leaf("File", ""),
        leaf("BoundingBox", bounding_box),
        leaf("Sticker", "true"),
        leaf("StickerReference", r.pick(&["_1f389", "_1f600"])),
    ];
    c.push(fading(r));
    drawing("Bitmap", r, c)
}

pub fn svg(r: &mut Rng) -> Element {
    let mut c = vec![leaf("File", SVG_FILE)];
    c.push(fading(r));
    drawing("SVG", r, c)
}

pub fn generic_posture(r: &mut Rng) -> Element {
    let points: Vec<Element> = (0..4).map(|_| leaf("Point", r.point())).collect();
    let mut c = vec![leaf("ToolId", POSTURE_TOOL), node("Positions", points)];
    c.extend(attached(r));
    drawing("GenericPosture", r, c)
}
