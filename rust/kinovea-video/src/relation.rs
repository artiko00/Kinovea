//! Port of `Kinovea.Video/TimestampRelation.cs`, `RelateTimestamps` and `GetPlaybackTimestamp`
//! from `Kinovea.Video/VideoReaders/VideoReader.cs`. `Math.Round` is reproduced with
//! `round_ties_even`.

use crate::state::{PlayerAction, PlayerState};
use std::time::Duration;

kinovea_core::text_enum!(
    /// `TimestampRelation`: where a request sits relative to a reference timestamp.
    TimestampRelation [Unknown, Match, FarBehind, Behind, Ahead, FarAhead]
);

/// `RelateTimestamps`: fuzzy comparison with a tolerance of half a frame and a far threshold
/// of 50 frames. Either timestamp being negative gives `Unknown`.
pub fn relate_timestamps(
    average_timestamps_per_frame: f64,
    request: i64,
    reference: i64,
) -> TimestampRelation {
    let tolerance = average_timestamps_per_frame * 0.5;
    let far_ahead_threshold = average_timestamps_per_frame * 50.0;

    if request < 0 || reference < 0 {
        return TimestampRelation::Unknown;
    }

    let delta = (request - reference) as f64;
    if delta.abs() <= tolerance {
        return TimestampRelation::Match;
    }
    if delta < -far_ahead_threshold {
        return TimestampRelation::FarBehind;
    }
    if delta < 0.0 {
        return TimestampRelation::Behind;
    }
    if delta > far_ahead_threshold {
        return TimestampRelation::FarAhead;
    }
    TimestampRelation::Ahead
}

/// `GetPlaybackTimestamp`: the timestamp the player expects `elapsed` after the playback
/// started. Outside playback it is 0. The caller measures `elapsed` from its own epoch.
pub fn playback_timestamp(
    average_timestamps_per_frame: f64,
    state: &PlayerState,
    elapsed: Duration,
) -> i64 {
    if state.action != PlayerAction::Playback {
        return 0;
    }

    let elapsed_frames = elapsed.as_secs_f64() * 1000.0 / state.playback_frame_interval;
    let elapsed_timestamps = elapsed_frames * average_timestamps_per_frame;
    (state.reference_timestamp as f64 + elapsed_timestamps).round_ties_even() as i64
}
