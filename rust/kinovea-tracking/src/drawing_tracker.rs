//! Trackable points of one drawing as the original delivers them (`DrawingTracker`): `GetPointAtTime`
//! for readers, and the tracking and camera steps that send positions to the drawing.

use crate::camera::CameraTransform;
use crate::trackable_point::CameraCache;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Track;
use kinovea_metadata::trackability::DrawingTracker;
use kinovea_metadata::{Drawing, Uuid};
use std::collections::HashMap;

/// Receives the positions of a drawing's trackable points (`SetTrackablePointValue`). `age` is the time
/// between the sample and the current time, -1 when the value does not come from a track.
pub trait Trackable {
    fn set_trackable_point_value(&mut self, key: &str, value: PointF, age: i64);
    /// `AfterAllTrackablePointsSet`: calibratable drawings recompute what depends on all their points.
    fn after_all_points_set(&mut self) {}
}

/// `GetTimedPoint` (`DrawingTrack.cs:1663-1679`): the sample at `time`, the first or last one outside
/// the track, nothing inside a gap.
pub fn track_point_at(track: &Track, time: i64) -> Option<TimedPoint> {
    let first = *track.positions.first()?;
    let last = *track.positions.last()?;
    if time < first.t {
        return Some(first);
    }
    if time > last.t {
        return Some(last);
    }
    track.positions.iter().find(|p| p.t == time).copied()
}

pub(crate) fn find_track(tracks: &[Drawing], id: Uuid) -> Option<&Track> {
    tracks.iter().find_map(|d| match d {
        Drawing::Track(t) if t.header.id == id => Some(t),
        _ => None,
    })
}

/// `GetPointAtTime` (`DrawingTracker.cs:682-706`). With object tracking initialized, the point of its
/// bound track at `time`, and `EMPTY` for an unbound key (`:684-692`). Otherwise the reference value
/// (`:695-699`). Camera motion is not applied here. The original dereferences a null sample inside a
/// gap; this returns `EMPTY` instead.
pub fn point_at_time(tracker: &DrawingTracker, tracks: &[Drawing], key: &str, time: i64) -> PointF {
    let Some((_, point)) = tracker.points.iter().find(|(k, _)| k == key) else {
        return PointF::EMPTY;
    };
    if !tracker.object_tracking_initialized {
        return point.reference_value;
    }
    point
        .track_id
        .and_then(|id| find_track(tracks, id))
        .and_then(|t| track_point_at(t, time))
        .map_or(PointF::EMPTY, |tp| PointF::new(tp.x, tp.y))
}

/// Runtime state of a drawing's tracker that KVA does not store.
#[derive(Debug, Default)]
pub struct DrawingState {
    /// `isCurrentlyTracking`: set by the user, cleared when object tracking ends.
    pub currently_tracking: bool,
    caches: HashMap<String, CameraCache>,
}

impl DrawingState {
    /// The steps that send positions to the drawing at `time`. With object tracking initialized, each
    /// bound point with a sample at `time` (`ObjectTrackingStep`, `:428-500`); points without one keep
    /// their value. Otherwise, with camera motion known, every point is camera-transformed
    /// (`CameraTrackingStep`, `:517-546`). Then `after_all_points_set` runs, as `AfterObjectTrackingStep`
    /// does for every tracker (`TrackabilityManager.cs:325-330`).
    pub fn apply_at(
        &mut self,
        tracker: &DrawingTracker,
        tracks: &[Drawing],
        time: i64,
        camera: &dyn CameraTransform,
        drawing: &mut dyn Trackable,
    ) {
        if tracker.object_tracking_initialized {
            for (key, p) in &tracker.points {
                let sample = p
                    .track_id
                    .and_then(|id| find_track(tracks, id))
                    .and_then(|t| track_point_at(t, time));
                if let Some(tp) = sample {
                    // Tracking sets the sample as the current step (age 0); viewing takes its distance (`:484`).
                    let age = if self.currently_tracking {
                        0
                    } else {
                        tp.t - time
                    };
                    drawing.set_trackable_point_value(key, PointF::new(tp.x, tp.y), age);
                }
            }
        } else if camera.initialized() {
            for (key, p) in &tracker.points {
                let cache = self.caches.entry(key.clone()).or_default();
                drawing.set_trackable_point_value(key, cache.camera_track(p, time, camera), -1);
            }
        }
        drawing.after_all_points_set();
    }
}
