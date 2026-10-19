use crate::capabilities::CacheLoadMode;
use crate::state::{PlayerAction, PlayerState, SeekProgress, WorkingZoneRequest};
use kinovea_core::video_section::VideoSection;

#[test]
fn clone_keeps_every_field() {
    let state = PlayerState::new(7, PlayerAction::Playback, false, 100, 40.0, 16.7);
    assert_eq!(state.clone(), state);
}

#[test]
fn invalid_state_is_a_synchronous_timestamp_request_at_zero() {
    let state = PlayerState::make_invalid();
    assert_eq!(state.id, -1);
    assert_eq!(state.action, PlayerAction::Timestamp);
    assert!(state.synchronous_fulfill);
    assert_eq!(state.reference_timestamp, 0);
}

#[test]
fn display_matches_csharp_tostring() {
    let state = PlayerState::new(3, PlayerAction::StepForward, true, 42, 0.0, 0.0);
    assert_eq!(
        state.to_string(),
        "#3 Mode: StepForward. Sync: True. Ref: [~42]."
    );
    let lazy = PlayerState::new(4, PlayerAction::Playback, false, 0, 0.0, 0.0);
    assert_eq!(
        lazy.to_string(),
        "#4 Mode: Playback. Sync: False. Ref: [~0]."
    );
}

#[test]
fn player_action_names_are_member_names() {
    assert_eq!(PlayerAction::Playback.name(), "Playback");
    assert_eq!(PlayerAction::StepBackward.name(), "StepBackward");
}

#[test]
fn working_zone_request_keeps_its_policy() {
    let request = WorkingZoneRequest::new(VideoSection::new(0, 99), CacheLoadMode::Reload, 512);
    assert_eq!(request.working_zone.start, 0);
    assert_eq!(request.working_zone.end, 99);
    assert_eq!(request.cache_load_mode, CacheLoadMode::Reload);
    assert_eq!(request.max_memory_mb, 512);
}

#[test]
fn empty_seek_progress_is_not_in_progress() {
    let progress = SeekProgress::empty();
    assert_eq!(progress.version, -1);
    assert!(!progress.in_progress());
}

#[test]
fn seek_progress_with_a_section_is_in_progress() {
    let progress = SeekProgress::new(4, VideoSection::new(10, 20));
    assert_eq!(progress.version, 4);
    assert!(progress.in_progress());
}
