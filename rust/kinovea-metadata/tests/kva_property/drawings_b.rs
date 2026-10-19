//! Seeded generators for the shape and point-list drawings: rectangles, planes, pencil strokes,
//! polylines, trajectory curves and distortion grids.

use super::gen::{drawing, leaf, node, Rng};
use super::parts::attached;
use kinovea_core::preferences::dom::Element;

const CORNERS: [&str; 4] = [
    "PointUpperLeft",
    "PointUpperRight",
    "PointLowerRight",
    "PointLowerLeft",
];

fn corners(r: &mut Rng) -> Vec<Element> {
    CORNERS.iter().map(|name| leaf(name, r.point())).collect()
}

/// A list of `count` points; keyed points carry a `key` attribute, as polylines do.
fn point_list(r: &mut Rng, name: &str, keyed: bool, count: i64) -> Element {
    let items: Vec<Element> = (0..count)
        .map(|i| {
            let mut p = leaf("Point", r.point());
            if keyed {
                p.attributes.push(("key".into(), i.to_string()));
            }
            p
        })
        .collect();
    let mut list = node(name, items);
    list.attributes.push(("Count".into(), count.to_string()));
    list
}

pub fn rectangle(r: &mut Rng) -> Element {
    let mut c = corners(r);
    c.push(leaf("Filled", r.flag()));
    c.extend(attached(r));
    drawing("Rectangle", r, c)
}

pub fn plane(r: &mut Rng) -> Element {
    let mut c = corners(r);
    c.extend([
        leaf("ShowGrid", r.flag()),
        leaf("ShowXLine", r.flag()),
        leaf("ShowYLine", r.flag()),
        leaf("XLineCoord", r.int(0, 4) as f32 / 4.0),
        leaf("YLineCoord", r.int(0, 4) as f32 / 4.0),
    ]);
    c.extend(attached(r));
    drawing("Plane", r, c)
}

pub fn pencil(r: &mut Rng) -> Element {
    let count = r.int(2, 5);
    let mut c = vec![point_list(r, "PointList", false, count)];
    c.extend(attached(r));
    drawing("Pencil", r, c)
}

pub fn polyline(r: &mut Rng) -> Element {
    let count = r.int(2, 5);
    let mut c = vec![point_list(r, "PointList", true, count)];
    c.extend(attached(r));
    drawing("Polyline", r, c)
}

pub fn time_polyline(r: &mut Rng) -> Element {
    let count = r.int(2, 5);
    let mut c = vec![point_list(r, "PointList", true, count)];
    c.extend(attached(r));
    drawing("TimePolyline", r, c)
}

pub fn distortion_grid(r: &mut Rng) -> Element {
    let mut c = vec![point_list(r, "Points", false, 9)];
    c.extend(attached(r));
    drawing("DistortionGrid", r, c)
}
