//! Import of Agisoft Lens calibration files (`DistortionImporterAgisoft`).

use super::parameters::DEFAULT_SENSOR_WIDTH_MM;
use super::serializer::ImportError;
use super::DistortionParameters;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::Size;
use std::path::Path;

/// Reads `/calibration/{width,height,fx,fy,cx,cy,k1,k2,k3,p1,p2}` and rescales the intrinsics
/// to `image_size`; every element is required.
pub fn import_agisoft(path: &Path, image_size: Size) -> Result<DistortionParameters, ImportError> {
    let shown = path.display().to_string();
    let text = std::fs::read_to_string(path).map_err(|_| ImportError::NotFound(shown.clone()))?;
    let malformed = |reason: String| ImportError::Malformed {
        path: shown.clone(),
        reason,
    };
    let root = Element::parse(&text).map_err(|e| malformed(e.to_string()))?;
    if root.name != "calibration" {
        return Err(malformed(format!("root element is {}", root.name)));
    }
    let read = |name: &str| -> Result<f64, ImportError> {
        let t = root.child_text(name).ok_or_else(|| malformed(format!("missing {name}")))?;
        t.trim().parse().map_err(|_| malformed(format!("{name} is not a number: {t}")))
    };
    let (width, height) = (read("width")?, read("height")?);
    let (mut fx, mut fy, mut cx, mut cy) = (read("fx")?, read("fy")?, read("cx")?, read("cy")?);
    let (k1, k2, k3, p1, p2) = (read("k1")?, read("k2")?, read("k3")?, read("p1")?, read("p2")?);
    if f64::from(image_size.width) != width || f64::from(image_size.height) != height {
        let xf = f64::from(image_size.width) / width;
        let yf = f64::from(image_size.height) / height;
        fx *= xf;
        fy *= yf;
        cx *= xf;
        cy *= yf;
    }
    let ppmm = f64::from(image_size.width) / DEFAULT_SENSOR_WIDTH_MM;
    Ok(DistortionParameters::new(k1, k2, k3, p1, p2, fx, fy, cx, cy, ppmm))
}
