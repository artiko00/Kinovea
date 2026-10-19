//! Parser of `KinoveaPostureTool` descriptions (`GenericPosture.ReadXml`). Unknown elements
//! are skipped; element lists are read in file order.

use super::model::*;
use crate::color::Color;
use crate::drawing::tracking_parameters::TrackingParameters;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, parse_int_list, parse_point_f, parse_point_reference};
use kinovea_geometry::PointF;
use uuid::Uuid;

mod handles;
#[cfg(test)]
mod handles_test;
mod shapes;
#[cfg(test)]
mod shapes_test;
pub(crate) use shapes::{attr, attr_bool, attr_color, attr_int, attr_text};

/// Pushes a parsed child into the list matching its element name.
macro_rules! lists {
    ($p:ident, $s:ident; $($($name:literal)|+ : $field:ident => $value:expr),+ $(,)?) => {
        match $s.name.as_str() {
            $($($name)|+ => $p.$field.push($value),)+
            _ => {}
        }
    };
}

/// Parses a tool file; `None` when the root is not a `KinoveaPostureTool`.
pub fn parse_posture(xml: &str, from_kva: bool) -> Option<GenericPosture> {
    let root = Element::parse(xml).ok()?;
    if root.name != "KinoveaPostureTool" {
        return None;
    }
    let mut p = GenericPosture { from_kva, ..GenericPosture::default() };
    for c in &root.children {
        match c.name.as_str() {
            "Id" => p.id = Uuid::parse_str(c.text.trim()).unwrap_or_default(),
            "Name" => p.name = c.text.trim().to_owned(),
            "DisplayName" => p.display_name = c.text.trim().to_owned(),
            "Icon" => p.icon = c.text.trim().to_owned(),
            "InitialConfiguration" => {
                for pt in c.children.iter().filter(|pt| pt.name == "Point") {
                    let (x, y) = parse_point_f(pt.text.trim());
                    p.points.push(PosturePoint { name: String::new(), value: PointF::new(x, y), color: TRANSPARENT });
                }
                p.point_list = p.points.iter().map(|pt| pt.value).collect();
            }
            "Points" => {
                for pt in c.children.iter().filter(|pt| pt.name == "Point") {
                    let (x, y) = parse_point_f(attr(pt, "value").unwrap_or("0;0"));
                    p.points.push(PosturePoint { name: attr_text(pt, "name"), value: PointF::new(x, y), color: attr_color(pt, "color") });
                }
                p.point_list = p.points.iter().map(|pt| pt.value).collect();
            }
            "Segments" | "Ellipses" | "Circles" | "Angles" | "Polylines" | "Handles" | "HitZone" | "Distances" | "Positions" | "ComputedPoints" => {
                for s in &c.children {
                    lists!(p, s;
                        "Segment": segments => shapes::segment(s),
                        "Ellipse" | "Circle": circles => shapes::circle(s),
                        "Angle": angles => shapes::angle(s),
                        "Polyline": polylines => shapes::polyline(s),
                        "Handle": handles => handles::handle(s),
                        "Polygon": hit_zones => HitZone { points: parse_int_list(attr(s, "points").unwrap_or("")) },
                        "Distance": distances => shapes::measure(s, "point1", "point2"),
                        "Position": positions => shapes::measure(s, "point", "point"),
                        "ComputedPoint": computed_points => shapes::computed_point(s),
                    );
                }
            }
            "Options" => {
                for o in c.children.iter().filter(|o| o.name == "Option") {
                    let key = attr_text(o, "key");
                    if !p.options.iter().any(|x| x.key == key) {
                        let default_value = attr_bool(o, "default", false);
                        p.options.push(PostureOption { key, label: attr_text(o, "label"), default_value, value: default_value, hidden: attr_bool(o, "hidden", false) });
                    }
                }
            }
            "Capabilities" => {
                p.flip_horizontal = attr_bool(c, "flipHorizontal", false);
                p.flip_vertical = attr_bool(c, "flipVertical", false);
            }
            "TrackingParameters" => p.tracking_parameters = TrackingParameters::read_kva(c),
            _ => {}
        }
    }
    consolidate_options(&mut p);
    p.trackable_indices = p.handles.iter().filter(|h| h.trackable).map(|h| h.reference).collect();
    Some(p)
}

/// `ConsolidateOptions`: every option group referenced by an element exists as an option.
fn consolidate_options(p: &mut GenericPosture) {
    let mut groups: Vec<String> = Vec::new();
    groups.extend(p.segments.iter().map(|s| s.option_group.clone()));
    for h in &p.handles {
        groups.push(h.option_group.clone());
        if let Some(c) = &h.constraint {
            groups.push(c.option_group.clone());
        }
    }
    groups.extend(p.circles.iter().map(|s| s.option_group.clone()));
    groups.extend(p.angles.iter().map(|s| s.option_group.clone()));
    groups.extend(p.distances.iter().map(|s| s.option_group.clone()));
    groups.extend(p.positions.iter().map(|s| s.option_group.clone()));
    groups.extend(p.computed_points.iter().map(|s| s.option_group.clone()));
    for group in groups.iter().filter(|g| !g.is_empty()) {
        for key in group.split('|') {
            if !p.options.iter().any(|o| o.label == key || o.key == key) {
                p.options.push(PostureOption { key: key.to_owned(), label: key.to_owned(), default_value: false, value: false, hidden: false });
            }
        }
    }
    p.has_non_hidden_options = p.options.iter().any(|o| !o.hidden);
}

/// Index of a point, accepting the `$n` variable form.
pub(crate) fn point_ref(e: &Element, name: &str) -> i32 {
    parse_point_reference(attr(e, name).unwrap_or("0"))
}

pub(crate) fn color_or_transparent(text: Option<&str>) -> Color {
    text.map_or(TRANSPARENT, |t| Color::parse(t, TRANSPARENT))
}

pub(crate) fn boolean(text: Option<&str>, fallback: bool) -> bool {
    text.map_or(fallback, parse_boolean)
}
