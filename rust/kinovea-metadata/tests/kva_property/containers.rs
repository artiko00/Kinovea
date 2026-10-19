//! Seeded generators for the containers and singletons: keyframes, spotlights, the number sequence,
//! the coordinate system and the test grid.

use super::drawings_a::{angle, circle, label, line};
use super::drawings_b::{pencil, plane, polyline, rectangle};
use super::gen::{drawing, leaf, node, Rng, TEXTS};
use super::media_posture::{bitmap, generic_posture, svg};
use super::parts::style;
use kinovea_core::preferences::dom::Element;

const KINDS: [fn(&mut Rng) -> Element; 11] = [
    line,
    angle,
    circle,
    label,
    rectangle,
    plane,
    pencil,
    polyline,
    bitmap,
    svg,
    generic_posture,
];

pub fn keyframe(r: &mut Rng) -> Element {
    let id = r.uuid();
    let drawings: Vec<Element> = (0..r.int(1, 3))
        .map(|_| {
            let make = r.pick(&KINDS);
            make(r)
        })
        .collect();
    let mut e = node(
        "Keyframe",
        vec![
            leaf("Timestamp", r.int(0, 9000)),
            leaf("Name", r.pick(&TEXTS)),
            leaf("Color", r.color()),
            leaf("Comment", "note"),
            node("Drawings", drawings),
        ],
    );
    e.attributes = vec![("id".into(), id)];
    e
}

fn spotlight(r: &mut Rng) -> Element {
    let mut e = node(
        "Spotlight",
        vec![
            leaf("Time", r.int(0, 9000)),
            leaf("Center", r.point()),
            leaf("Radius", r.int(1, 80)),
        ],
    );
    e.attributes = vec![("id".into(), r.uuid()), ("name".into(), String::new())];
    e
}

pub fn spotlights(r: &mut Rng) -> Element {
    let items: Vec<Element> = (0..r.int(1, 3)).map(|_| spotlight(r)).collect();
    node("Spotlights", items)
}

fn auto_number(r: &mut Rng, value: i64) -> Element {
    let mut e = node(
        "AutoNumber",
        vec![
            leaf("Time", r.int(0, 9000)),
            leaf("Location", r.point()),
            leaf("Value", value),
        ],
    );
    e.attributes = vec![("id".into(), r.uuid()), ("name".into(), String::new())];
    e
}

pub fn number_sequence(r: &mut Rng) -> Element {
    let mut items: Vec<Element> = (1..=r.int(1, 3)).map(|v| auto_number(r, v)).collect();
    items.push(style(r));
    node("NumberSequence", items)
}

pub fn coordinate_system(r: &mut Rng) -> Element {
    let c = vec![
        leaf("Position", r.point()),
        leaf("ReferenceTimestamp", r.int(0, 9000)),
        leaf("Visible", r.flag()),
        leaf("ShowGrid", r.flag()),
        leaf("ShowGraduations", r.flag()),
        style(r),
    ];
    drawing("CoordinateSystem", r, c)
}

pub fn test_grid(r: &mut Rng) -> Element {
    let c = vec![
        leaf("Visible", r.flag()),
        leaf("ShowHorizontalAxis", r.flag()),
        leaf("ShowVerticalAxis", r.flag()),
        leaf("ShowFraming", r.flag()),
        leaf("ShowThirds", r.flag()),
        style(r),
    ];
    drawing("TestGrid", r, c)
}
