//! Track bindings of trackable points and the content hash of the trackability state
//! (`InitializeTracking`, `IsObjectTrackingInitialized`, `ContentHash`). As in the original, the hash
//! covers the drawing, the object tracking flag and the reference values, not the bound track ids.

use super::{DrawingTracker, TrackabilityManager, TrackablePoint};
use uuid::Uuid;

/// `Guid.GetHashCode`: `_a ^ ((_b << 16) | _c) ^ ((_f << 24) | _k)` over the fields of the text form,
/// with `_c` read as unsigned. `_f` is the 3rd byte of the last group, `_k` the last one.
fn uuid_hash(id: Uuid) -> i32 {
    let (a, b, c, d) = id.as_fields();
    (a as i32) ^ (((b as i32) << 16) | c as i32) ^ (((d[2] as i32) << 24) | d[7] as i32)
}

/// `Int64.GetHashCode`.
fn i64_hash(v: i64) -> i32 {
    (v ^ (v >> 32)) as i32
}

impl TrackablePoint {
    /// `TrackablePoint.ContentHash` (`TrackablePoint.cs:51-61`): reference value and timestamp.
    fn content_hash(&self) -> i32 {
        self.reference_value.hash_code() ^ i64_hash(self.reference_timestamp)
    }
}

impl DrawingTracker {
    /// `DrawingTracker.ContentHash` (`DrawingTracker.cs:83-94`).
    pub fn content_hash(&self) -> i32 {
        let initialized = i32::from(self.object_tracking_initialized);
        self.points
            .iter()
            .fold(uuid_hash(self.drawing_id) ^ initialized, |h, (_, p)| {
                h ^ p.content_hash()
            })
    }
}

impl TrackabilityManager {
    pub fn content_hash(&self) -> i32 {
        self.trackers.iter().fold(0, |h, t| h ^ t.content_hash())
    }

    /// `IsObjectTrackingInitialized`.
    pub fn is_object_tracking_initialized(&self, drawing_id: Uuid) -> bool {
        self.tracker(drawing_id)
            .is_some_and(|t| t.object_tracking_initialized)
    }

    /// The object tracking part of `InitializeTracking` (`DrawingTracker.cs:300-305`). The tracking flag
    /// (`isCurrentlyTracking`) is runtime state: see `TrabilityRuntime::initialize_tracking`.
    pub fn start_object_tracking(&mut self, drawing_id: Uuid) {
        if let Some(t) = self
            .trackers
            .iter_mut()
            .find(|t| t.drawing_id == drawing_id)
        {
            t.object_tracking_initialized = true;
        }
    }

    /// Binds the point `key` of the drawing to `track_id` and starts object tracking; unknown drawings
    /// or keys are ignored.
    pub fn bind_track(&mut self, drawing_id: Uuid, key: &str, track_id: Uuid) {
        let point = self
            .trackers
            .iter_mut()
            .find(|t| t.drawing_id == drawing_id)
            .and_then(|t| t.points.iter_mut().find(|(k, _)| k == key));
        if let Some((_, p)) = point {
            p.track_id = Some(track_id);
            self.start_object_tracking(drawing_id);
        }
    }

    /// Drops the binding to `track_id` without leaving object tracking: what `Assign` does with the
    /// tracks that are gone (`DrawingTracker.cs:186-222`).
    pub fn unbind_track(&mut self, track_id: Uuid) {
        for (_, p) in self.trackers.iter_mut().flat_map(|t| t.points.iter_mut()) {
            if p.track_id == Some(track_id) {
                p.track_id = None;
            }
        }
    }
}
