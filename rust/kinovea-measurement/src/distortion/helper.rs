//! Distort and rectify image points (`DistortionHelper`); the identity until initialized.

use super::undistort::{distort_normalized, undistort_normalized};
use super::DistortionParameters;
use kinovea_geometry::{PointF, Quadrilateral, Size, Vector};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DistortionHelper {
    parameters: Option<DistortionParameters>,
    image_size: Size,
    content_hash: i32,
}

impl DistortionHelper {
    /// Parameters with a zero principal point are replaced by the defaults for `image_size`.
    pub fn initialize(&mut self, parameters: DistortionParameters, image_size: Size) {
        self.content_hash = parameters.content_hash();
        let p = if parameters.cx == 0.0 && parameters.cy == 0.0 { DistortionParameters::defaults(image_size) } else { parameters };
        self.parameters = Some(p);
        self.image_size = image_size;
    }

    pub fn uninitialize(&mut self) {
        *self = DistortionHelper::default();
    }

    pub fn is_initialized(&self) -> bool {
        self.parameters.is_some()
    }

    pub fn parameters(&self) -> Option<&DistortionParameters> {
        self.parameters.as_ref()
    }

    pub fn image_size(&self) -> Size {
        self.image_size
    }

    /// Hash of the parameters as given to [`initialize`](Self::initialize); 0 when uninitialized.
    pub fn content_hash(&self) -> i32 {
        self.content_hash
    }

    /// Rectifies a distorted image point (OpenCV `undistortPoints` semantics, float in/out).
    pub fn undistort(&self, point: PointF) -> PointF {
        let Some(p) = &self.parameters else {
            return point;
        };
        let x = (f64::from(point.x) - p.cx) / p.fx;
        let y = (f64::from(point.y) - p.cy) / p.fy;
        let (xu, yu) = undistort_normalized(p, x, y);
        // OpenCV returns the normalized result as float before it is scaled back.
        let (xu, yu) = (f64::from(xu as f32), f64::from(yu as f32));
        PointF::new((xu * p.fx + p.cx) as f32, (yu * p.fy + p.cy) as f32)
    }

    /// Applies the distortion to a rectified image point.
    pub fn distort(&self, point: PointF) -> PointF {
        let Some(p) = &self.parameters else {
            return point;
        };
        let x = (f64::from(point.x) - p.cx) / p.fx;
        let y = (f64::from(point.y) - p.cy) / p.fy;
        let (xd, yd) = distort_normalized(p, x, y);
        PointF::new((xd * p.fx + p.cx) as f32, (yd * p.fy + p.cy) as f32)
    }

    /// Seven points following the distorted curve of the segment given in distorted space.
    pub fn distort_line(&self, start: PointF, end: PointF) -> Vec<PointF> {
        let mut points = vec![start];
        self.distort_inner_points(self.undistort(start), self.undistort(end), &mut points);
        points.push(end);
        points
    }

    /// Seven points following the distorted curve of a segment given in rectified space.
    pub fn distort_rectified_line(&self, start: PointF, end: PointF) -> Vec<PointF> {
        let mut points = vec![self.distort(start)];
        self.distort_inner_points(start, end, &mut points);
        points.push(self.distort(end));
        points
    }

    fn distort_inner_points(&self, start: PointF, end: PointF, points: &mut Vec<PointF>) {
        const INNER: i32 = 5;
        let factor = 1.0 / (INNER as f32 + 1.0);
        let v = Vector::from_points(start, end);
        for i in 1..=INNER {
            let undistorted = start + v * (i as f32 * factor);
            points.push(self.distort(undistorted));
        }
    }

    pub fn undistort_quad(&self, quad: &Quadrilateral) -> Quadrilateral {
        Quadrilateral {
            vertices: quad.vertices.map(|v| self.undistort(v)),
        }
    }
}
