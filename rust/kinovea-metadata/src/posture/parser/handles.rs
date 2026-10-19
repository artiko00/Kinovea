//! `Handle` elements: type, reference, optional `Constraint` and `Impacts`.

use super::super::model::*;
use super::{attr, attr_bool, attr_color, attr_int, attr_text};
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::parse_int_list;
use kinovea_geometry::PointF;

pub(super) fn handle(e: &Element) -> Handle {
    let mut h = Handle {
        name: attr_text(e, "name"),
        kind: attr(e, "type").and_then(|t| t.parse().ok()).unwrap_or(HandleType::Point),
        color: attr_color(e, "color"),
        reference: attr_int(e, "reference", 0),
        trackable: attr_bool(e, "trackable", false),
        option_group: attr_text(e, "optionGroup"),
        grab_point: PointF::EMPTY,
        constraint: None,
        impacts: Vec::new(),
    };
    for c in &e.children {
        match c.name.as_str() {
            "Constraint" => h.constraint = constraint(c),
            "Impact" | "Impacts" => h.impacts.extend(c.children.iter().filter_map(impact)),
            _ => {}
        }
    }
    h
}

/// The constraint's parameters live in a child element named after its type.
fn constraint(e: &Element) -> Option<Constraint> {
    let kind_name = attr(e, "type").unwrap_or("None");
    let p = e.children.first();
    let int = |name: &str, fallback: i32| p.map_or(fallback, |c| attr_int(c, name, fallback));
    let kind = match kind_name {
        "LineSlide" => ConstraintKind::LineSlide {
            start: int("point1", 0),
            end: int("point2", 0),
            allowed: parse_line_position(p.and_then(|c| attr(c, "position")).unwrap_or("")),
            margin: 10,
        },
        "VerticalSlide" => ConstraintKind::VerticalSlide,
        "HorizontalSlide" => ConstraintKind::HorizontalSlide,
        "DistanceToPoint" => ConstraintKind::DistanceToPoint { point: int("point", 0) },
        "RotationSteps" => ConstraintKind::RotationSteps {
            origin: int("origin", 0),
            leg1: int("leg1", 0),
            step: int("step", 0),
            keep_distance: p.is_some_and(|c| attr_bool(c, "keepDistance", false)),
        },
        "PerpendicularSlide" => ConstraintKind::PerpendicularSlide { origin: int("origin", 0), leg1: int("leg1", 0) },
        "ParallelSlide" => ConstraintKind::ParallelSlide { a: int("a", 0), b: int("b", 0), c: int("c", 0) },
        "LockedInPlace" => ConstraintKind::LockedInPlace,
        _ => return None,
    };
    Some(Constraint { kind, option_group: attr_text(e, "optionGroup") })
}

fn impact(e: &Element) -> Option<Impact> {
    let int = |name: &str| attr_int(e, name, 0);
    Some(match e.name.as_str() {
        "Align" => Impact::LineAlign { start: int("point1"), end: int("point2"), point_to_align: int("pointToAlign") },
        "VerticalAlign" => Impact::VerticalAlign { point: int("point") },
        "HorizontalAlign" => Impact::HorizontalAlign { point: int("point") },
        "Pivot" => Impact::Pivot { pivot: int("pivot"), impacted: parse_int_list(attr(e, "impacted").unwrap_or("")) },
        "KeepAngle" => Impact::KeepAngle { origin: int("origin"), leg1: int("leg1"), leg2: int("leg2"), old_angle: 0.0, old_distance: 0.0 },
        "HorizontalSymmetry" => Impact::HorizontalSymmetry { impacting: int("impacting"), impacted: int("impacted"), axis: int("axis") },
        "SegmentCenter" => Impact::SegmentCenter { point_to_move: int("pointToMove"), point1: int("point1"), point2: int("point2") },
        "PerdpendicularAlign" => Impact::PerpendicularAlign { point_to_move: int("pointToMove"), origin: int("origin"), leg1: int("leg1") },
        "ParallelAlign" => Impact::ParallelAlign { point_to_move: int("pointToMove"), a: int("a"), b: int("b"), c: int("c") },
        _ => return None,
    })
}
