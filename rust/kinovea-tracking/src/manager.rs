//! Trackability at runtime (`TrackabilityManager`). The trackers themselves are owned by the caller
//! (`kinovea_metadata::TrackabilityManager`, read and written as KVA); this keeps the state KVA does
//! not store: which drawings are being tracked and the camera caches.

use crate::camera::CameraTransform;
use crate::drawing_tracker::{find_track, point_at_time, DrawingState, Trackable};
use kinovea_geometry::PointF;
use kinovea_metadata::trackability::TrackabilityManager;
use kinovea_metadata::{Drawing, Uuid};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TrackabilityRuntime {
    drawings: HashMap<Uuid, DrawingState>,
}

impl TrackabilityRuntime {
    /// `IsTracking`.
    pub fn is_tracking(&self, drawing_id: Uuid) -> bool {
        self.drawings
            .get(&drawing_id)
            .is_some_and(|s| s.currently_tracking)
    }

    /// `AnyTracking`: some drawing is being tracked.
    pub fn any_tracking(&self) -> bool {
        self.drawings.values().any(|s| s.currently_tracking)
    }

    /// `ToggleTracking` (`DrawingTracker.cs:328-336`): only with object tracking initialized.
    pub fn toggle_tracking(&mut self, data: &TrackabilityManager, drawing_id: Uuid) {
        if !data.is_object_tracking_initialized(drawing_id) {
            return;
        }
        let state = self.drawings.entry(drawing_id).or_default();
        state.currently_tracking = !state.currently_tracking;
    }

    /// `GetPointAtTime` (`TrackabilityManager.cs:385-398`): `EMPTY` for an unregistered drawing.
    pub fn point_at_time(
        &self,
        data: &TrackabilityManager,
        tracks: &[Drawing],
        drawing_id: Uuid,
        key: &str,
        time: i64,
    ) -> PointF {
        data.tracker(drawing_id)
            .map_or(PointF::EMPTY, |t| point_at_time(t, tracks, key, time))
    }

    /// Sends the positions of a registered drawing at `time` (see `DrawingState::apply_at`).
    pub fn apply_at(
        &mut self,
        data: &TrackabilityManager,
        tracks: &[Drawing],
        drawing_id: Uuid,
        time: i64,
        camera: &dyn CameraTransform,
        drawing: &mut dyn Trackable,
    ) {
        let Some(tracker) = data.tracker(drawing_id) else {
            return;
        };
        self.drawings
            .entry(drawing_id)
            .or_default()
            .apply_at(tracker, tracks, time, camera, drawing);
    }

    /// `InitializeTracking` (`DrawingTracker.cs:300-305`): object tracking starts and the drawing is
    /// tracked (`isCurrentlyTracking`, which lives here, not in the data).
    pub fn initialize_tracking(&mut self, data: &mut TrackabilityManager, drawing_id: Uuid) {
        data.start_object_tracking(drawing_id);
        self.drawings
            .entry(drawing_id)
            .or_default()
            .currently_tracking = true;
    }

    /// `ForgetTrack` (`TrackabilityManager.cs:606-615`). Drawings left without object tracking stop
    /// tracking (`DrawingTracker.cs:374-376`, `:385-402`).
    pub fn forget_track(&mut self, data: &mut TrackabilityManager, track_id: Uuid) {
        data.forget_track(track_id);
        for (id, state) in self.drawings.iter_mut() {
            if !data.is_object_tracking_initialized(*id) {
                state.currently_tracking = false;
            }
        }
    }

    /// `Remove` (`TrackabilityManager.cs:197-205`): the drawing takes its tracker and runtime state.
    pub fn remove(&mut self, data: &mut TrackabilityManager, drawing_id: Uuid) {
        data.remove(drawing_id);
        self.drawings.remove(&drawing_id);
    }
}

/// `Assign` (`TrackabilityManager.cs:92-100`, `DrawingTracker.cs:167-222`): a tracker read from a file
/// takes back its drawing. Bindings to tracks that are gone are dropped, and object tracking starts
/// when the file had any binding (`:186`, `:222`). Returns false when the file had no tracker for it.
pub fn assign(
    data: &mut TrackabilityManager,
    tracks: &[Drawing],
    drawing_id: Uuid,
    name: &str,
    timestamp: i64,
    points: &[(String, PointF)],
) -> bool {
    let Some(tracker) = data.tracker(drawing_id) else {
        return false;
    };
    let bound: Vec<Uuid> = tracker
        .points
        .iter()
        .filter_map(|(_, p)| p.track_id)
        .collect();
    for id in bound.iter().filter(|id| find_track(tracks, **id).is_none()) {
        data.unbind_track(*id);
    }
    data.add(drawing_id, name, timestamp, points);
    if !bound.is_empty() {
        data.start_object_tracking(drawing_id);
    }
    true
}
