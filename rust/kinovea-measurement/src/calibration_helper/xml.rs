//! KVA `Calibration` fragment (`CalibrationHelper.WriteXml`/`ReadXml`).

use super::CalibrationHelper;
use crate::calibrator_type::CalibratorType;
use crate::distortion::serializer::{from_element, to_element};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::{PointF, Size};
use uuid::Uuid;

impl CalibrationHelper {
    /// Children of the `Calibration` element: the calibrator fragment, `CalibrationDrawingId`,
    /// `Unit` (with `Abbreviation`) and the optional `CameraCalibration`.
    pub fn write_xml(&self) -> Vec<Element> {
        let mut out = Vec::new();
        match self.calibrator_type {
            CalibratorType::Line => {
                let mut e = Element::new("CalibrationLine");
                e.children = self.calibrator.write_line_xml();
                out.push(e);
            }
            CalibratorType::Plane => {
                let mut e = Element::new("CalibrationPlane");
                e.children = self.calibrator.write_plane_xml();
                out.push(e);
            }
            CalibratorType::None => {}
        }
        out.push(Element::leaf("CalibrationDrawingId", &self.drawing_id.to_string()));
        let mut unit = Element::leaf("Unit", self.units.length.name());
        unit.attributes.push(("Abbreviation".into(), self.length_abbreviation().into()));
        out.push(unit);
        if let Some(p) = self.distortion.parameters() {
            out.push(to_element(p, None));
        }
        out
    }

    /// Reads a `Calibration` element; the quad is scaled by `scale`, distortion parameters are
    /// rescaled to `image_size`.
    pub fn read_xml(&mut self, root: &Element, scale: PointF, image_size: Size) {
        for c in &root.children {
            match c.name.as_str() {
                "CalibrationPlane" => {
                    self.calibrator_type = CalibratorType::Plane;
                    self.calibrator.read_plane_xml(c, scale);
                }
                "CalibrationLine" => {
                    self.calibrator_type = CalibratorType::Line;
                    self.calibrator.read_line_xml(c, scale);
                }
                "CalibrationDrawingId" => {
                    if let Ok(id) = Uuid::parse_str(c.text.trim()) {
                        self.drawing_id = id;
                    }
                }
                "Unit" => {
                    if let Ok(unit) = c.text.trim().parse() {
                        self.units.length = unit;
                    }
                }
                "CameraCalibration" => {
                    let parameters = from_element(c, image_size);
                    self.distortion.initialize(parameters, image_size);
                }
                _ => {}
            }
        }
    }
}
