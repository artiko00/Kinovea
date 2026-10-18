//! Origin, offset and the line/plane calibration entry points.

use super::CalibrationHelper;
use crate::calibration_axis::CalibrationAxis;
use crate::calibrator_type::CalibratorType;
use kinovea_geometry::{PointF, Quadrilateral, SizeF};
use uuid::Uuid;

impl CalibrationHelper {
    /// Image position of the world origin.
    pub fn origin(&self) -> PointF {
        self.distortion.distort(self.calibrator.untransform(PointF::EMPTY))
    }

    pub fn set_origin(&mut self, p: PointF) {
        let u = self.distortion.undistort(p);
        self.calibrator.set_origin(u);
    }

    pub fn reset_origin(&mut self) {
        self.calibrator.reset_origin();
    }

    pub fn set_offset(&mut self, p: PointF) {
        self.calibrator.offset = p;
    }

    pub fn world_offset(&self) -> PointF {
        self.calibrator.offset
    }

    pub fn calibrate_by_line(&mut self, id: Uuid, length: f32, a: PointF, b: PointF, axis: CalibrationAxis) {
        self.drawing_id = id;
        let (a, b) = (self.distortion.undistort(a), self.distortion.undistort(b));
        self.calibrator.initialize_line(length, a, b, axis);
    }

    /// Ignored unless the type is line and the id matches.
    pub fn update_line(&mut self, id: Uuid, a: PointF, b: PointF) {
        if self.calibrator_type != CalibratorType::Line || id != self.drawing_id {
            return;
        }
        let (a, b) = (self.distortion.undistort(a), self.distortion.undistort(b));
        self.calibrator.update_line(a, b);
    }

    pub fn calibrate_by_plane(&mut self, id: Uuid, size: SizeF, quad_image: &Quadrilateral) {
        self.drawing_id = id;
        let undistorted = self.distortion.undistort_quad(quad_image);
        self.calibrator.initialize_plane(size, undistorted);
    }

    /// Ignored unless the type is plane and the id matches.
    pub fn update_plane(&mut self, id: Uuid, quad_image: &Quadrilateral) {
        if self.calibrator_type != CalibratorType::Plane || id != self.drawing_id {
            return;
        }
        let undistorted = self.distortion.undistort_quad(quad_image);
        self.calibrator.update_quad(undistorted);
    }
}
