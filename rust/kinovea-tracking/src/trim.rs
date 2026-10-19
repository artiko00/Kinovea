//! Trimming of the tracks bound to a drawing (`TrimAttachedTracks`, `Track.Trim`).

use kinovea_metadata::drawing::Track;
use kinovea_metadata::trackability::TrackabilityManager;
use kinovea_metadata::{Drawing, Uuid};

/// `TrimAttachedTracks` (`Metadata.cs:1251-1260`): cuts the tracks bound to the drawing after `time`.
/// `trim_tracker` receives the id of each cut track and the time of its closest sample, where the
/// original trims the track's tracker (`DrawingTrack.cs:1562`).
pub fn trim_attached_tracks(
    data: &TrackabilityManager,
    tracks: &mut [Drawing],
    drawing_id: Uuid,
    time: i64,
    mut trim_tracker: impl FnMut(Uuid, i64),
) {
    let Some(tracker) = data.tracker(drawing_id) else {
        return;
    };
    let bound: Vec<Uuid> = tracker
        .points
        .iter()
        .filter_map(|(_, p)| p.track_id)
        .collect();
    tracks
        .iter_mut()
        .filter_map(|d| match d {
            Drawing::Track(t) if bound.contains(&t.header.id) => Some(t),
            _ => None,
        })
        .for_each(|t| {
            if let Some(sample) = trim_track(t, time) {
                trim_tracker(t.header.id, sample);
            }
        });
}

/// `Track.Trim` (`DrawingTrack.cs:1540-1565`): keeps the positions up to the one closest to `time`.
/// Returns the time of that position when samples were removed, and `None` otherwise.
pub fn trim_track(track: &mut Track, time: i64) -> Option<i64> {
    let last = track.closest_point(time);
    if last + 1 >= track.positions.len() {
        return None;
    }
    track.positions.truncate(last + 1);
    Some(track.positions[last].t)
}
