//! Legacy `Drawing Type="…"` elements rewritten as 2.0 drawing elements.

use super::{copy_child, path_text, style, text};
use kinovea_core::preferences::dom::Element;

pub(super) fn convert(drawings: &Element) -> Element {
    let mut out = Element::new("Drawings");
    for d in drawings.children.iter().filter(|c| c.name == "Drawing") {
        out.push(match d.attribute("Type").unwrap_or("") {
            "DrawingAngle2D" => angle(d),
            "DrawingCircle" => circle(d),
            "DrawingCross2D" => cross(d),
            "DrawingLine2D" => line(d),
            "DrawingPencil" => pencil(d),
            "DrawingText" => label(d),
            _ => d.clone(),
        });
    }
    out
}

fn with_style(e: &mut Element, elements: Vec<Element>) {
    let mut block = Element::new("DrawingStyle");
    block.children = elements;
    e.push(block);
}

fn measure_label(e: &mut Element, position: &str) {
    let mut label = Element::new("MeasureLabel");
    label.push_leaf("SpacePosition", position);
    label.push_leaf("TimePosition", "0");
    e.push(label);
}

fn angle(d: &Element) -> Element {
    let mut e = Element::new("Angle");
    for name in ["PointO", "PointA", "PointB"] {
        copy_child(&mut e, d, name);
    }
    e.push_leaf("Signed", "false");
    e.push_leaf("CCW", "true");
    e.push_leaf("Supplementary", "false");
    with_style(&mut e, vec![style("Color", "line color", path_text(d, &["TextDecoration", "BackColor"]))]);
    copy_child(&mut e, d, "InfosFading");
    copy_child(&mut e, d, "Measure");
    e
}

fn circle(d: &Element) -> Element {
    let mut e = Element::new("Circle");
    copy_child(&mut e, d, "Origin");
    copy_child(&mut e, d, "Radius");
    e.push_leaf("ExtraData", "None");
    measure_label(&mut e, text(d, "Origin"));
    with_style(&mut e, vec![style("Color", "color", path_text(d, &["LineStyle", "ColorRGB"])), style("PenSize", "pen size", path_text(d, &["LineStyle", "Size"])), style("PenShape", "pen shape", "Solid")]);
    copy_child(&mut e, d, "InfosFading");
    e
}

fn cross(d: &Element) -> Element {
    let mut e = Element::new("CrossMark");
    copy_child(&mut e, d, "CenterPoint");
    e.push_leaf("ExtraData", if text(d, "CoordinatesVisible") == "True" { "Position" } else { "None" });
    measure_label(&mut e, text(d, "CenterPoint"));
    with_style(&mut e, vec![style("Color", "back color", path_text(d, &["LineStyle", "ColorRGB"]))]);
    copy_child(&mut e, d, "InfosFading");
    copy_child(&mut e, d, "Coordinates");
    e
}

fn line(d: &Element) -> Element {
    let mut e = Element::new("Line");
    e.push_leaf("Start", text(d, "m_StartPoint"));
    e.push_leaf("End", text(d, "m_EndPoint"));
    e.push_leaf("ExtraData", if text(d, "MeasureIsVisible") == "True" { "TotalDistance" } else { "None" });
    measure_label(&mut e, text(d, "m_StartPoint"));
    let arrows = match path_text(d, &["LineStyle", "LineShape"]) {
        "EndArrow" => "EndArrow",
        "DoubleArrow" => "DoubleArrow",
        _ => "None",
    };
    with_style(&mut e, vec![style("Color", "color", path_text(d, &["LineStyle", "ColorRGB"])), style("LineSize", "line size", path_text(d, &["LineStyle", "Size"])), style("LineShape", "line shape", "Solid"), style("Arrows", "arrows", arrows)]);
    copy_child(&mut e, d, "InfosFading");
    copy_child(&mut e, d, "Measure");
    e
}

fn pencil(d: &Element) -> Element {
    let mut e = Element::new("Pencil");
    copy_child(&mut e, d, "PointList");
    with_style(&mut e, vec![style("Color", "color", path_text(d, &["LineStyle", "ColorRGB"])), style("PenSize", "pen size", path_text(d, &["LineStyle", "Size"])), style("PenShape", "pen shape", "Solid")]);
    copy_child(&mut e, d, "InfosFading");
    e
}

fn label(d: &Element) -> Element {
    let mut e = Element::new("Label");
    e.attributes.push(("name".into(), text(d, "Text").to_owned()));
    copy_child(&mut e, d, "Text");
    copy_child(&mut e, d, "Position");
    e.push_leaf("ArrowVisible", "false");
    e.push_leaf("ArrowEnd", text(d, "Position"));
    with_style(&mut e, vec![style("Color", "back color", path_text(d, &["TextDecoration", "BackColor"])), style("FontSize", "font size", path_text(d, &["TextDecoration", "FontSize"]))]);
    copy_child(&mut e, d, "InfosFading");
    e
}
