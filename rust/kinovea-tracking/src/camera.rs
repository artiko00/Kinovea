//! The camera-motion transform (`CameraTransformer`): where a point at a reference time is at another
//! time. Estimating the motion belongs to a later change; this crate only consumes it.

use kinovea_geometry::PointF;

pub trait CameraTransform {
    /// `Initialized`: whether camera motion is known. Without it points keep their reference value.
    fn initialized(&self) -> bool;
    /// `Transform`: the position at `target_t` of a point that was at `point` at `source_t`.
    fn transform(&self, source_t: i64, target_t: i64, point: PointF) -> PointF;
}
