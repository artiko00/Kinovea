//! Spatial calibration facade (`CalibrationHelper`): calibrator type, image size, units, lens
//! distortion and the tracked calibration source. Implements `MeasurementContext`.

mod calibrate;
#[cfg(test)]
mod calibrate_test;
mod convert;
#[cfg(test)]
mod convert_test;
mod text;
#[cfg(test)]
mod text_test;
mod transform;
#[cfg(test)]
mod transform_test;
mod xml;
#[cfg(test)]
mod xml_test;

use crate::calibration_axis::CalibrationAxis;
use crate::calibrator_plane::CalibratorPlane;
use crate::calibrator_type::CalibratorType;
use crate::distortion::DistortionHelper;
use crate::source::{StaticSource, TrackedCalibrationSource};
use crate::units::Units;
use kinovea_core::units::LengthUnit;
use kinovea_geometry::{PointF, Quadrilateral, Size, SizeF};
use uuid::Uuid;

impl std::fmt::Debug for CalibrationHelper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CalibrationHelper").field("type", &self.calibrator_type).field("image_size", &self.image_size).field("drawing_id", &self.drawing_id).finish_non_exhaustive()
    }
}

pub struct CalibrationHelper {
    calibrator_type: CalibratorType,
    calibrator: CalibratorPlane,
    distortion: DistortionHelper,
    drawing_id: Uuid,
    image_size: Size,
    units: Units,
    capture_fps: f64,
    source: Box<dyn TrackedCalibrationSource>,
}

impl CalibrationHelper {
    /// A reset calibration for `image_size` with a tracked source.
    pub fn new(image_size: Size, units: Units, capture_fps: f64, source: Box<dyn TrackedCalibrationSource>) -> Self {
        let mut h = CalibrationHelper {
            calibrator_type: CalibratorType::Line,
            calibrator: CalibratorPlane::default(),
            distortion: DistortionHelper::default(),
            drawing_id: Uuid::nil(),
            image_size,
            units,
            capture_fps,
            source,
        };
        h.reset();
        h
    }

    /// Calibration without tracking.
    pub fn without_tracking(image_size: Size, units: Units, capture_fps: f64) -> Self {
        Self::new(image_size, units, capture_fps, Box::new(StaticSource))
    }

    /// Default line calibration: 100 pixels along a horizontal line from the image centre.
    pub fn reset(&mut self) {
        self.calibrator_type = CalibratorType::Line;
        self.calibrator = CalibratorPlane::default();
        self.drawing_id = Uuid::nil();
        let center = self.image_size.center();
        let end = PointF::new(center.x + 100.0, center.y);
        self.calibrator.initialize_line(100.0, center, end, CalibrationAxis::LineHorizontal);
        self.distortion = DistortionHelper::default();
        self.units.length = LengthUnit::Pixels;
    }

    pub fn is_calibrated(&self) -> bool {
        self.units.length != LengthUnit::Pixels
    }

    pub fn units(&self) -> Units {
        self.units
    }

    /// `RefreshUnits`: replaces every unit, including the length unit.
    pub fn set_units(&mut self, units: Units) {
        self.units = units;
    }

    pub fn set_length_unit(&mut self, unit: LengthUnit) {
        self.units.length = unit;
    }

    pub fn calibrator_type(&self) -> CalibratorType {
        self.calibrator_type
    }

    pub fn set_calibrator_type(&mut self, kind: CalibratorType) {
        self.calibrator_type = kind;
    }

    pub fn calibrator(&self) -> &CalibratorPlane {
        &self.calibrator
    }

    pub fn calibration_axis(&self) -> CalibrationAxis {
        self.calibrator.axis()
    }

    pub fn distortion(&self) -> &DistortionHelper {
        &self.distortion
    }

    pub fn distortion_mut(&mut self) -> &mut DistortionHelper {
        &mut self.distortion
    }

    pub fn image_size(&self) -> Size {
        self.image_size
    }

    pub fn drawing_id(&self) -> Uuid {
        self.drawing_id
    }

    pub fn capture_fps(&self) -> f64 {
        self.capture_fps
    }

    pub fn set_capture_fps(&mut self, fps: f64) {
        self.capture_fps = fps;
    }

    /// Distortion hash XOR origin hash.
    pub fn content_hash(&self) -> i32 {
        self.distortion.content_hash() ^ self.origin().hash_code()
    }

    pub fn plane_size(&self) -> SizeF {
        self.calibrator.size
    }

    pub fn plane_is_valid(&self) -> bool {
        self.calibrator.is_valid()
    }

    pub fn projected_quad(&self) -> Quadrilateral {
        self.calibrator.quad_image()
    }
}
