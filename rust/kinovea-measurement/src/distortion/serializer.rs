//! `CameraCalibration` XML fragment and the Kinovea parameter file (`DistortionSerializer`,
//! `DistortionImporterKinovea`).

use super::parameters::DEFAULT_SENSOR_WIDTH_MM;
use super::DistortionParameters;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{format_f64_invariant, parse_size};
use kinovea_geometry::Size;
use std::fmt;
use std::path::Path;

/// A parameter file could not be imported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportError {
    NotFound(String),
    Malformed { path: String, reason: String },
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::NotFound(p) => write!(f, "lens calibration file not found: {p}"),
            ImportError::Malformed { path, reason } => write!(f, "malformed lens calibration {path}: {reason}"),
        }
    }
}

impl std::error::Error for ImportError {}

/// `<CameraCalibration>` with the optional `ImageSize` first.
pub fn to_element(p: &DistortionParameters, image_size: Option<Size>) -> Element {
    let mut e = Element::new("CameraCalibration");
    if let Some(s) = image_size {
        e.push_leaf("ImageSize", &format!("{};{}", s.width, s.height));
    }
    let values = [("Fx", p.fx), ("Fy", p.fy), ("Cx", p.cx), ("Cy", p.cy), ("K1", p.k1), ("K2", p.k2), ("K3", p.k3), ("P1", p.p1), ("P2", p.p2), ("PixelsPerMM", p.pixels_per_millimeter)];
    for (name, v) in values {
        e.push_leaf(name, &format_f64_invariant(v));
    }
    e.push_leaf("Path", &p.path);
    e
}

/// Reads a `<CameraCalibration>` element, rescaling the intrinsics to `input_size` when the
/// stored image size differs and defaulting pixels-per-millimetre from the sensor width.
pub fn from_element(e: &Element, input_size: Size) -> DistortionParameters {
    let mut size = input_size;
    let mut p = DistortionParameters::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0);
    for c in &e.children {
        let v = || c.text.trim().parse::<f64>().unwrap_or(0.0);
        match c.name.as_str() {
            "ImageSize" => {
                let (w, h) = parse_size(&c.text);
                size = Size::new(w, h);
            }
            "PixelsPerMM" => p.pixels_per_millimeter = c.text.trim().parse().unwrap_or(0.0),
            "Fx" => p.fx = v(),
            "Fy" => p.fy = v(),
            "Cx" => p.cx = v(),
            "Cy" => p.cy = v(),
            "K1" => p.k1 = v(),
            "K2" => p.k2 = v(),
            "K3" => p.k3 = v(),
            "P1" => p.p1 = v(),
            "P2" => p.p2 = v(),
            "Path" => p.path = c.text.clone(),
            _ => {}
        }
    }
    if p.pixels_per_millimeter == 0.0 {
        p.pixels_per_millimeter = f64::from(size.width) / DEFAULT_SENSOR_WIDTH_MM;
    }
    if input_size != size {
        let xf = f64::from(input_size.width) / f64::from(size.width);
        let yf = f64::from(input_size.height) / f64::from(size.height);
        p.fx *= xf;
        p.fy *= yf;
        p.cx *= xf;
        p.cy *= yf;
    }
    p
}

/// Imports a Kinovea lens calibration file; `path` is recorded in the parameters.
pub fn import_kinovea(path: &Path, image_size: Size) -> Result<DistortionParameters, ImportError> {
    let text = std::fs::read_to_string(path).map_err(|_| ImportError::NotFound(path.display().to_string()))?;
    let root = Element::parse(&text).map_err(|e| ImportError::Malformed {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    let mut p = from_element(&root, image_size);
    p.path = path.display().to_string();
    Ok(p)
}

/// Writes the parameters with the image size, in the `.NET` `XmlWriter` document style.
pub fn export_kinovea(path: &Path, p: &DistortionParameters, image_size: Size) -> std::io::Result<()> {
    std::fs::write(path, to_element(p, Some(image_size)).to_document())
}
