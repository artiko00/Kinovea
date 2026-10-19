//! Legacy KVA formats (1.3 to 1.5) converted to 2.0, as `xslt/kva-1.5to2.0.xsl` does before the
//! reader runs. Track times in those files are relative to the track start and are fixed after
//! the import (`FixRelativeTrajectories`).

use kinovea_core::preferences::dom::Element;

mod drawings;
#[cfg(test)]
mod drawings_test;
mod tracks;
#[cfg(test)]
mod tracks_test;

pub const PRODUCER: &str = "Kinovea XSLT Converter (1.5 to 2.0)";

/// Formats 1.3 to 1.5 are converted; 2.x is read as is; older ones are rejected by the reader.
pub fn needs_migration(version: f64) -> bool {
    (1.3..2.0).contains(&version)
}

/// Port of the stylesheet: a 2.0 document built from the legacy root.
pub fn migrate_legacy(root: &Element) -> Element {
    let mut out = Element::new("KinoveaVideoAnalysis");
    out.push_leaf("FormatVersion", "2.0");
    out.push_leaf("Producer", PRODUCER);
    for name in ["OriginalFilename", "GlobalTitle", "ImageSize", "AverageTimeStampsPerFrame", "FirstTimeStamp", "SelectionStart"] {
        copy_child(&mut out, root, name);
    }
    for help in descendants(root, "CalibrationHelp") {
        out.push(calibration(help));
    }
    for keyframes in descendants(root, "Keyframes") {
        let mut list = Element::new("Keyframes");
        for kf in keyframes.children.iter().filter(|k| k.name == "Keyframe") {
            let mut e = Element::new("Keyframe");
            copy_child(&mut e, kf, "Position");
            if kf.child("Title").is_some() {
                copy_child(&mut e, kf, "Title");
                copy_child(&mut e, kf, "Comment");
            }
            for d in kf.children.iter().filter(|c| c.name == "Drawings") {
                e.push(drawings::convert(d));
            }
            list.push(e);
        }
        out.push(list);
    }
    for chronos in descendants(root, "Chronos") {
        out.push(chronos.clone());
    }
    for tracks in descendants(root, "Tracks") {
        out.push(tracks::convert(tracks));
    }
    out.push(Element::new("Trackability"));
    out
}

/// `CalibrationHelp` → line calibration from the pixel-to-unit scale and the unit code.
fn calibration(help: &Element) -> Element {
    let mut cal = Element::new("Calibration");
    let mut line = Element::new("CalibrationLine");
    let origin = text(help, "CoordinatesOrigin");
    if origin != "-1;-1" {
        line.push_leaf("Origin", origin);
    }
    line.push_leaf("Scale", text(help, "PixelToUnit"));
    cal.push(line);
    let unit_code = help.child("LengthUnit");
    let unit = match unit_code.map_or("", |u| u.text.trim()) {
        "0" => "Centimeters",
        "1" => "Meters",
        "2" => "Inches",
        "3" => "Feet",
        "4" => "Yards",
        _ => "Pixels",
    };
    let mut unit_element = Element::leaf("Unit", unit);
    unit_element.attributes.push(("Abbreviation".into(), unit_code.and_then(|u| u.attribute("UserUnitLength")).unwrap_or("").to_owned()));
    cal.push(unit_element);
    cal
}

/// `xsl:copy-of select="name"`: the child element with its content, when present.
pub(crate) fn copy_child(out: &mut Element, from: &Element, name: &str) {
    if let Some(c) = from.child(name) {
        out.push(c.clone());
    }
}

/// `xsl:value-of select="name"`: the child text or the empty string.
pub(crate) fn text<'a>(e: &'a Element, name: &str) -> &'a str {
    e.child(name).map_or("", |c| c.text.as_str())
}

/// `xsl:value-of` along a path of child names.
pub(crate) fn path_text<'a>(e: &'a Element, path: &[&str]) -> &'a str {
    let mut current = e;
    for name in path {
        match current.child(name) {
            Some(c) => current = c,
            None => return "",
        }
    }
    current.text.as_str()
}

/// `//name` in document order.
fn descendants<'a>(e: &'a Element, name: &str) -> Vec<&'a Element> {
    let mut out = Vec::new();
    for c in &e.children {
        if c.name == name {
            out.push(c);
        }
        out.extend(descendants(c, name));
    }
    out
}

/// A `<Tag Key="key"><Value>value</Value></Tag>` style element.
pub(crate) fn style(tag: &str, key: &str, value: &str) -> Element {
    let mut e = Element::new(tag);
    e.attributes.push(("Key".into(), key.to_owned()));
    e.push_leaf("Value", value);
    e
}
