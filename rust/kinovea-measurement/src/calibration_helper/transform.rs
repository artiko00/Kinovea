//! Value computers of the calibration: image ↔ world points, scalars, times and the image
//! ellipse of a world circle.

use super::CalibrationHelper;
use crate::calibrator_plane::make_quad;
use crate::calibrator_type::CalibratorType;
use kinovea_geometry::helpers::distance;
use kinovea_geometry::shapes::{Circle, Ellipse};
use kinovea_geometry::{PointF, ProjectiveMapper, Quadrilateral};
use uuid::Uuid;

impl CalibrationHelper {
    /// Image → world with the static calibration.
    pub fn point(&self, p: PointF) -> PointF {
        self.calibrator.transform(self.distortion.undistort(p))
    }

    /// Image → world at `time`, following the tracked calibration object or origin.
    pub fn point_at_time(&self, p: PointF, time: i64) -> PointF {
        if self.calibrator_type == CalibratorType::None || self.drawing_id == Uuid::nil() {
            return self.point(p);
        }
        let query = self.distortion.undistort(p);
        if self.source.has_tracking_data(self.drawing_id) {
            let mut quad = self.source.calibration_quad(time, self.calibrator_type, self.drawing_id);
            if self.calibrator_type == CalibratorType::Line {
                quad = make_quad(quad.a(), quad.b(), self.calibrator.axis());
            }
            let undistorted = self.distortion.undistort_quad(&quad);
            let mut at_time = self.calibrator.clone_calibrator();
            at_time.update_quad(undistorted);
            at_time.transform_with_origin(query, undistorted.d())
        } else {
            let origin = self.distortion.undistort(self.source.calibration_origin(time));
            self.calibrator.transform_with_origin(query, origin)
        }
    }

    /// Rectified image → world.
    pub fn point_from_rectified(&self, p: PointF) -> PointF {
        self.calibrator.transform(p)
    }

    /// Image length along X from the image origin → world length, keeping the sign.
    pub fn scalar(&self, v: f32) -> f32 {
        let d = distance(self.point(PointF::EMPTY), self.point(PointF::new(v, 0.0)));
        if v < 0.0 { -d } else { d }
    }

    /// `(float)(frames / fps)`.
    pub fn time(&self, frames: i32) -> f32 {
        (f64::from(frames) / self.capture_fps) as f32
    }

    pub fn image_length(&self, p1: PointF, p2: PointF) -> f32 {
        distance(self.image_point(p1), self.image_point(p2))
    }

    pub fn image_scalar(&self, v: f32) -> f32 {
        let d = distance(self.image_point(PointF::EMPTY), self.image_point(PointF::new(v, 0.0)));
        if v < 0.0 { -d } else { d }
    }

    /// World → image.
    pub fn image_point(&self, p: PointF) -> PointF {
        self.distortion.distort(self.calibrator.untransform(p))
    }

    /// Image ellipse of a world circle; a plain circle under a line calibration.
    pub fn ellipse_from_circle(&self, circle: Circle) -> Ellipse {
        let (c, r) = (circle.center, circle.radius);
        if self.calibrator_type == CalibratorType::Line {
            let radius = self.image_scalar(r);
            return Ellipse::new(self.image_point(c), radius, radius, 0.0);
        }
        self.ellipse_of_world_square(c, r)
    }

    /// Image ellipse of the circle drawn in image space at `center` with `radius`, plus the
    /// image points at the left and right ends of its horizontal world diameter.
    pub fn ellipse_from_image_circle(&self, center: PointF, radius: f32) -> (Ellipse, PointF, PointF) {
        let left = PointF::new(center.x - radius, center.y);
        let right = PointF::new(center.x + radius, center.y);
        if self.calibrator_type == CalibratorType::Line {
            return (Ellipse::new(center, radius, radius, 0.0), left, right);
        }
        let c = self.point(center);
        let r = self.scalar(radius);
        let left = self.image_point(c.translate(-r, 0.0));
        let right = self.image_point(c.translate(r, 0.0));
        (self.ellipse_of_world_square(c, r), left, right)
    }

    fn ellipse_of_world_square(&self, c: PointF, r: f32) -> Ellipse {
        let quad = Quadrilateral::new(
            self.image_point(c.translate(-r, -r)),
            self.image_point(c.translate(r, -r)),
            self.image_point(c.translate(r, r)),
            self.image_point(c.translate(-r, r)),
        );
        ProjectiveMapper::new(&Quadrilateral::centered_unit_square(), &quad)
            .map(|m| m.ellipse())
            .unwrap_or_default()
    }
}
