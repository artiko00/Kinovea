//! Tests for `relation.rs`: `RelateTimestamps` and `GetPlaybackTimestamp` from `VideoReader.cs`.

use crate::relation::{playback_timestamp, relate_timestamps, TimestampRelation};
use crate::state::{PlayerAction, PlayerState};
use std::time::Duration;

fn playback_state(reference: i64, frame_interval_ms: f64) -> PlayerState {
    PlayerState::new(
        1,
        PlayerAction::Playback,
        false,
        reference,
        frame_interval_ms,
        0.0,
    )
}

#[test]
fn match_within_half_a_frame() {
    assert_eq!(relate_timestamps(4.0, 2, 0), TimestampRelation::Match);
    assert_eq!(relate_timestamps(4.0, 0, 2), TimestampRelation::Match);
    assert_eq!(relate_timestamps(4.0, 3, 0), TimestampRelation::Ahead);
}

#[test]
fn tolerance_edge_is_inclusive() {
    assert_eq!(relate_timestamps(1.0, 0, 0), TimestampRelation::Match);
    assert_eq!(relate_timestamps(1.0, 1, 0), TimestampRelation::Ahead);
}

#[test]
fn behind_and_ahead_up_to_fifty_frames() {
    assert_eq!(relate_timestamps(4.0, 0, 3), TimestampRelation::Behind);
    assert_eq!(relate_timestamps(1.0, 50, 0), TimestampRelation::Ahead);
    assert_eq!(relate_timestamps(1.0, 0, 50), TimestampRelation::Behind);
}

#[test]
fn far_beyond_fifty_frames() {
    assert_eq!(relate_timestamps(1.0, 51, 0), TimestampRelation::FarAhead);
    assert_eq!(relate_timestamps(1.0, 0, 51), TimestampRelation::FarBehind);
}

#[test]
fn negative_timestamp_is_unknown() {
    assert_eq!(relate_timestamps(1.0, -1, 0), TimestampRelation::Unknown);
    assert_eq!(relate_timestamps(1.0, 0, -1), TimestampRelation::Unknown);
}

#[test]
fn playback_projects_two_elapsed_frames() {
    let state = playback_state(100, 40.0);
    assert_eq!(
        playback_timestamp(1.0, &state, Duration::from_millis(80)),
        102
    );
}

#[test]
fn playback_rounds_half_to_even() {
    let state = playback_state(100, 40.0);
    // Half a frame is 100.5, which Math.Round takes to 100.
    assert_eq!(
        playback_timestamp(1.0, &state, Duration::from_millis(20)),
        100
    );
    // A frame and a half is 101.5, which rounds to 102.
    assert_eq!(
        playback_timestamp(1.0, &state, Duration::from_millis(60)),
        102
    );
}

#[test]
fn playback_scales_by_timestamps_per_frame() {
    let state = playback_state(100, 40.0);
    assert_eq!(
        playback_timestamp(2.0, &state, Duration::from_millis(80)),
        104
    );
}

#[test]
fn non_playback_state_projects_zero() {
    let state = PlayerState::new(1, PlayerAction::Timestamp, true, 100, 40.0, 0.0);
    assert_eq!(
        playback_timestamp(1.0, &state, Duration::from_millis(80)),
        0
    );
}
