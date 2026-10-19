//! Attribute helpers and the element readers for segments, circles, angles, polylines,
//! read-outs and computed points.

use super::super::model::*;
use super::{boolean, color_or_transparent, point_ref};
use crate::color::Color;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::parse_int_list;

pub(crate) fn attr<'a>(e: &'a Element, name: &str) -> Option<&'a str> {
    e.attribute(name)
}

pub(crate) fn attr_text(e: &Element, name: &str) -> String {
    attr(e, name).unwrap_or("").to_owned()
}

pub(crate) fn attr_int(e: &Element, name: &str, fallback: i32) -> i32 {
    attr(e, name).and_then(|v| v.trim().parse().ok()).unwrap_or(fallback)
}

pub(crate) fn attr_bool(e: &Element, name: &str, fallback: bool) -> bool {
    boolean(attr(e, name), fallback)
}

pub(crate) fn attr_color(e: &Element, name: &str) -> Color {
    color_or_transparent(attr(e, name))
}

fn attr_f32(e: &Element, name: &str) -> f32 {
    attr(e, name).and_then(|w| w.trim().parse().ok()).unwrap_or(0.0)
}

fn stroke(e: &Element) -> Stroke {
    let arrow_begin = attr_bool(e, "arrowBegin", false) || attr_bool(e, "arrowStart", false);
    Stroke {
        style: attr(e, "style").and_then(|s| s.parse().ok()).unwrap_or(SegmentLineStyle::Solid),
        width: attr_int(e, "width", 2),
        arrow_begin,
        arrow_end: attr_bool(e, "arrowEnd", false),
        color: attr_color(e, "color"),
    }
}

pub(super) fn segment(e: &Element) -> Segment {
    Segment { name: attr_text(e, "name"), start: attr_int(e, "point1", 0), end: attr_int(e, "point2", 0), stroke: stroke(e), option_group: attr_text(e, "optionGroup") }
}

pub(super) fn circle(e: &Element) -> PostureCircle {
    PostureCircle { name: attr_text(e, "name"), center: point_ref(e, "center"), radius: attr_int(e, "radius", 0), stroke: stroke(e), option_group: attr_text(e, "optionGroup") }
}

pub(super) fn polyline(e: &Element) -> PosturePolyline {
    PosturePolyline { name: attr_text(e, "name"), points: parse_int_list(attr(e, "points").unwrap_or("")), stroke: stroke(e), option_group: attr_text(e, "optionGroup") }
}

/// The original stores the `symbol` attribute into the name (and keeps `Symbol` empty).
pub(super) fn angle(e: &Element) -> PostureAngle {
    let name = attr(e, "symbol").or_else(|| attr(e, "name")).unwrap_or("").to_owned();
    PostureAngle {
        name,
        origin: attr_int(e, "origin", 0),
        leg1: attr_int(e, "leg1", 0),
        leg2: attr_int(e, "leg2", 0),
        text_distance: attr_int(e, "textDistance", 40),
        radius: attr_int(e, "radius", 40),
        tenth: attr_bool(e, "tenth", false),
        color: attr_color(e, "color"),
        symbol: String::new(),
        signed: attr_bool(e, "signed", true),
        ccw: attr_bool(e, "ccw", true),
        supplementary: attr_bool(e, "supplementary", false),
        option_group: attr_text(e, "optionGroup"),
    }
}

pub(super) fn measure(e: &Element, first: &str, second: &str) -> Measure {
    Measure { name: attr_text(e, "name"), point1: point_ref(e, first), point2: point_ref(e, second), symbol: attr_text(e, "symbol"), color: attr_color(e, "color"), option_group: attr_text(e, "optionGroup") }
}

pub(super) fn computed_point(e: &Element) -> ComputedPoint {
    let children = e
        .children
        .iter()
        .filter_map(|c| match c.name.as_str() {
            "ComputedPoint" => Some(WeightedPoint::Computed(computed_point(c))),
            "ReferencePoint" => Some(WeightedPoint::Reference { reference: attr_int(c, "ref", 0), weight: attr_f32(c, "weight") }),
            _ => None,
        })
        .collect();
    ComputedPoint {
        name: attr_text(e, "name"),
        weight: attr_f32(e, "weight").clamp(0.0, 1.0),
        color: attr_color(e, "color"),
        symbol: attr_text(e, "symbol"),
        option_group: attr_text(e, "optionGroup"),
        display_coordinates: attr_bool(e, "displayCoordinates", false),
        children,
    }
}
