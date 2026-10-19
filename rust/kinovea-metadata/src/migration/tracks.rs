//! Legacy `Track` elements: positions become `TrackPointList`, the label becomes `MainLabel`.

use super::{copy_child, path_text, style};
use kinovea_core::preferences::dom::Element;

const POINT_ATTRIBUTES: [&str; 5] = ["UserX", "UserXInvariant", "UserY", "UserYInvariant", "UserTime"];

pub(super) fn convert(tracks: &Element) -> Element {
    let mut out = Element::new("Tracks");
    for t in tracks.children.iter().filter(|c| c.name == "Track") {
        out.push(track(t));
    }
    out
}

fn track(t: &Element) -> Element {
    let label = path_text(t, &["Label", "Text"]);
    let mut e = Element::new("Track");
    e.attributes.push(("name".into(), label.to_owned()));
    copy_child(&mut e, t, "TimePosition");
    copy_child(&mut e, t, "Mode");
    copy_child(&mut e, t, "ExtraData");
    let legacy_list = t.child("TrackPositionList");
    let mut list = Element::new("TrackPointList");
    for attr in ["Count", "UserUnitLength"] {
        list.attributes.push((attr.into(), legacy_list.and_then(|l| l.attribute(attr)).unwrap_or("").to_owned()));
    }
    for p in legacy_list.map(|l| l.children.iter().filter(|c| c.name == "TrackPosition").collect::<Vec<_>>()).unwrap_or_default() {
        let mut point = Element::leaf("TrackPoint", &p.text);
        for attr in POINT_ATTRIBUTES {
            point.attributes.push((attr.into(), p.attribute(attr).unwrap_or("").to_owned()));
        }
        list.push(point);
    }
    e.push(list);
    let mut main = Element::new("MainLabel");
    main.attributes.push(("Text".into(), label.to_owned()));
    main.push_leaf("SpacePosition", path_text(t, &["MainLabel", "KeyframeLabel", "SpacePosition"]));
    main.push_leaf("TimePosition", path_text(t, &["MainLabel", "KeyframeLabel", "TimePosition"]));
    e.push(main);
    let mut block = Element::new("DrawingStyle");
    block.children = vec![style("Color", "color", path_text(t, &["TrackLine", "LineStyle", "ColorRGB"])), style("LineSize", "line size", path_text(t, &["TrackLine", "LineStyle", "Size"])), style("TrackShape", "track shape", "Solid;false")];
    e.push(block);
    e
}
