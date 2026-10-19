use crate::container::TryAcquireResult;
use crate::frame::VideoFrame;
use crate::prebuffer::{PreBuffer, PreBufferConfig};
use crate::state::{PlayerAction, PlayerState};

fn frame(ts: i64, prev: i64) -> VideoFrame {
    VideoFrame::new(None, ts, prev)
}

fn request(action: PlayerAction, reference: i64) -> PlayerState {
    PlayerState::new(0, action, false, reference, 0.0, 0.0)
}

fn current_ts(pb: &PreBuffer) -> Option<i64> {
    pb.current().map(|f| f.timestamp)
}

/// Frames 0, 10, 20, ... 90 chained by one step of 10, with the given tolerance.
fn stepped(tolerance: f64) -> PreBuffer {
    let pb = PreBuffer::new(PreBufferConfig {
        tolerance,
        ..PreBufferConfig::default()
    });
    for ts in (0..100).step_by(10) {
        pb.add(frame(ts, ts - 10));
    }
    pb
}

#[test]
fn timestamp_request_matches_within_tolerance() {
    let pb = stepped(5.0);
    let result = pb.try_acquire(&request(PlayerAction::Timestamp, 34));
    assert_eq!(result, TryAcquireResult::new(true, 30, true));
    assert_eq!(current_ts(&pb), Some(30));
}

#[test]
fn timestamp_request_beyond_tolerance_is_not_matched() {
    let pb = PreBuffer::new(PreBufferConfig {
        tolerance: 5.0,
        ..PreBufferConfig::default()
    });
    pb.add(frame(0, -1));
    pb.add(frame(100, 99));
    let result = pb.try_acquire(&request(PlayerAction::Timestamp, 50));
    assert_eq!(result, TryAcquireResult::new(false, -1, false));
    assert_eq!(current_ts(&pb), None);
}

#[test]
fn playback_request_matches_within_tolerance() {
    let pb = stepped(5.0);
    let result = pb.try_acquire(&request(PlayerAction::Playback, 55));
    assert_eq!(result, TryAcquireResult::new(true, 50, true));
    assert_eq!(current_ts(&pb), Some(50));
}

#[test]
fn playback_request_outside_tolerance_keeps_current_unchanged() {
    let pb = stepped(2.0);
    pb.acquire_closest(30);
    let result = pb.try_acquire(&request(PlayerAction::Playback, 55));
    assert!(!result.target_acquired);
    assert_eq!(result.acquired_timestamp, -1);
    assert_eq!(current_ts(&pb), Some(30));
}

#[test]
fn step_forward_acquires_a_linked_neighbour() {
    let pb = stepped(0.0);
    pb.acquire_closest(0);
    let result = pb.try_acquire(&request(PlayerAction::StepForward, 0));
    assert_eq!(result, TryAcquireResult::new(true, 10, true));
    assert_eq!(current_ts(&pb), Some(10));
}

#[test]
fn step_forward_without_link_keeps_current_but_still_evicts() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    pb.add(frame(0, -10));
    pb.add(frame(10, 0));
    pb.add(frame(30, 20));
    pb.acquire_closest(10);
    let version = pb.snapshot().version;
    let result = pb.try_acquire(&request(PlayerAction::StepForward, 0));
    assert!(!result.target_acquired);
    assert_eq!(current_ts(&pb), Some(10));
    assert_eq!(pb.snapshot().version, version + 1);
}

#[test]
fn step_backward_acquires_the_linked_previous_frame() {
    let pb = stepped(0.0);
    pb.acquire_closest(20);
    let result = pb.try_acquire(&request(PlayerAction::StepBackward, 0));
    assert_eq!(result, TryAcquireResult::new(true, 10, true));
}

#[test]
fn step_without_current_or_past_the_end_gives_the_empty_result() {
    let pb = stepped(0.0);
    let version = pb.snapshot().version;
    assert_eq!(
        pb.try_acquire(&request(PlayerAction::StepForward, 0)),
        TryAcquireResult::empty()
    );
    assert_eq!(pb.snapshot().version, version);
    pb.acquire_closest(90);
    let version = pb.snapshot().version;
    assert_eq!(
        pb.try_acquire(&request(PlayerAction::StepForward, 0)),
        TryAcquireResult::empty()
    );
    assert_eq!(pb.snapshot().version, version);
}

#[test]
fn try_acquire_on_empty_buffer_is_the_empty_result() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    assert_eq!(
        pb.try_acquire(&request(PlayerAction::Playback, 5)),
        TryAcquireResult::empty()
    );
}
