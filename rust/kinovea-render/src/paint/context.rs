//! What a painter needs besides the drawing (`Draw` parameters of the original).

use crate::viewport::ImageToViewport;
use kinovea_geometry::PointF;
use kinovea_metadata::{Fading, Metadata};

/// Lens distortion applied to image points (`DistortionHelper`); identity when `None`.
pub trait Distorter {
    fn distort(&self, p: PointF) -> PointF;
}

/// Camera motion compensation (`CameraTransformer`); identity when `None`.
pub trait CameraTransform {
    fn transform(&self, p: PointF, timestamp: i64) -> PointF;
}

/// `Draw(canvas, distorter, cameraTransformer, transformer, selected, currentTimestamp)` plus the
/// parent metadata (calibration for measure texts, keyframes for track labels) and the default
/// fading of the player preferences.
pub struct PaintContext<'a> {
    pub viewport: &'a ImageToViewport,
    pub timestamp: i64,
    pub selected: bool,
    pub metadata: &'a Metadata,
    pub default_fading: &'a Fading,
    pub distorter: Option<&'a dyn Distorter>,
    pub camera: Option<&'a dyn CameraTransform>,
}
