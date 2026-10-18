//! The standard drawing tool descriptions (`Tools/DrawingTools/Standard/*.xml`) embedded at
//! compile time; they provide the default style of each drawing kind.

use super::StyleElements;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::parse_boolean;
use uuid::Uuid;

macro_rules! standard {
    ($($file:literal),* $(,)?) => {
        &[$(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Tools/DrawingTools/Standard/", $file))),*]
    };
}

const STANDARD_XML: &[&str] = standard!(
    "angle.xml", "arrow.xml", "arrowcurve.xml", "arrowdash.xml", "arrowpolyline.xml",
    "arrowpolylinedash.xml", "arrowpolylinesquiggly.xml", "arrowsquiggly.xml", "chrono.xml",
    "chronomulti.xml", "circle.xml", "clock.xml", "counter.xml", "crossmark.xml", "curve.xml",
    "distortiongrid.xml", "grid.xml", "label.xml", "line.xml", "pencil.xml", "plane.xml",
    "polyline.xml", "rectangle.xml", "sticker.xml", "timepolyline.xml", "timesegment.xml",
);

/// A `KinoveaDrawingTool` description.
#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub icon: String,
    /// The C# class, e.g. `DrawingLine`.
    pub drawing_class: String,
    pub attached: bool,
    pub keep_tool_after_drawing_creation: bool,
    pub keep_tool_after_frame_change: bool,
    pub default_style: StyleElements,
}

impl Tool {
    pub fn parse(xml: &str) -> Option<Tool> {
        let root = Element::parse(xml).ok()?;
        if root.name != "KinoveaDrawingTool" {
            return None;
        }
        let text = |name: &str| root.child_text(name).unwrap_or("").trim().to_owned();
        Some(Tool {
            id: Uuid::parse_str(&text("Id")).ok()?,
            name: text("Name"),
            display_name: text("DisplayName"),
            icon: text("Icon"),
            drawing_class: text("DrawingClass"),
            attached: parse_boolean(&text("Attached")),
            keep_tool_after_drawing_creation: parse_boolean(&text("KeepToolAfterDrawingCreation")),
            keep_tool_after_frame_change: parse_boolean(&text("KeepToolAfterFrameChange")),
            default_style: root.child("DefaultStyle").map(StyleElements::read_kva).unwrap_or_default(),
        })
    }
}

/// The 26 shipped tools, in file name order.
pub fn standard_tools() -> Vec<Tool> {
    STANDARD_XML.iter().filter_map(|xml| Tool::parse(xml)).collect()
}

/// Default style of the tool named `name` (`ToolManager.GetDefaultStyleElements`); empty when unknown.
pub fn default_style(name: &str) -> StyleElements {
    standard_tools().into_iter().find(|t| t.name == name).map(|t| t.default_style).unwrap_or_default()
}
