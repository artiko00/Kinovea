//! Camera-track positions of a trackable point (`TrackablePoint.CameraTrack`). Each time is transformed
//! once; the cache is dropped as soon as the reference it was computed from changes.

use crate::camera::CameraTransform;
use kinovea_geometry::PointF;
use kinovea_metadata::trackability::TrackablePoint;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CameraCache {
    reference: Option<(i64, PointF)>,
    positions: HashMap<i64, PointF>,
}

impl CameraCache {
    /// The reference value without camera motion, else the position transformed from the reference time.
    pub fn camera_track(
        &mut self,
        point: &TrackablePoint,
        time: i64,
        camera: &dyn CameraTransform,
    ) -> PointF {
        if !camera.initialized() {
            return point.reference_value;
        }
        let reference = (point.reference_timestamp, point.reference_value);
        if self.reference != Some(reference) {
            self.positions.clear();
            self.reference = Some(reference);
        }
        *self.positions.entry(time).or_insert_with(|| {
            camera.transform(point.reference_timestamp, time, point.reference_value)
        })
    }
}
