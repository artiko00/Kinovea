//! Seeded generators for the line-like drawings: lines, angles, markers, circles, labels, segments.

use super::gen::{drawing, leaf, Rng, TEXTS};
use super::parts::{attached, measure};
use kinovea_core::preferences::dom::Element;

pub fn line(r: &mut Rng) -> Element {
    let mut c = vec![
        leaf("Start", r.point()),
        leaf("End", r.point()),
        leaf("ReferenceTimestamp", r.int(0, 9000)),
    ];
    c.extend(measure(r));
    c.extend(attached(r));
    drawing("Line", r, c)
}

pub fn angle(r: &mut Rng) -> Element {
    let mut c = vec![
        leaf("PointO", r.point()),
        leaf("PointA", r.point()),
        leaf("PointB", r.point()),
        leaf("Signed", r.flag()),
        leaf("CCW", r.flag()),
        leaf("Supplementary", r.flag()),
        leaf("ShowCircle", r.flag()),
    ];
    c.extend(attached(r));
    drawing("Angle", r, c)
}

pub fn cross_mark(r: &mut Rng) -> Element {
    let mut c = vec![leaf("CenterPoint", r.point()), leaf("ShowAsDot", r.flag())];
    c.extend(measure(r));
    c.extend(attached(r));
    drawing("CrossMark", r, c)
}

pub fn circle(r: &mut Rng) -> Element {
    let mut c = vec![
        leaf("Origin", r.point()),
        leaf("Radius", r.int(1, 200)),
        leaf("Filled", r.flag()),
        leaf("ShowCenter", r.flag()),
    ];
    c.extend(measure(r));
    c.extend(attached(r));
    drawing("Circle", r, c)
}

pub fn label(r: &mut Rng) -> Element {
    let mut c = vec![
        leaf("Text", r.pick(&TEXTS)),
        leaf("Position", r.point()),
        leaf("BackgroundVisible", r.flag()),
        leaf("ArrowVisible", r.flag()),
        leaf("CircleVisible", r.flag()),
        leaf("ArrowEnd", r.point()),
    ];
    c.extend(attached(r));
    drawing("Label", r, c)
}

pub fn time_segment(r: &mut Rng) -> Element {
    let mut c = vec![
        leaf("Start", r.point()),
        leaf("End", r.point()),
        leaf("Fraction", r.int(0, 4) as f32 / 4.0),
    ];
    c.extend(measure(r));
    c.extend(attached(r));
    drawing("TimeSegment", r, c)
}
