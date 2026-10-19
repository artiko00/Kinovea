use crate::frame::VideoFrame;
use crate::prebuffer::evict::{evict_behind, evict_one};

fn frame(ts: i64, prev: i64) -> VideoFrame {
    VideoFrame::new(None, ts, prev)
}

fn chain(stamps: std::ops::RangeInclusive<i64>) -> Vec<VideoFrame> {
    stamps.map(|ts| frame(ts, ts - 1)).collect()
}

fn stamps(frames: &[VideoFrame]) -> Vec<i64> {
    frames.iter().map(|f| f.timestamp).collect()
}

#[test]
fn evict_behind_needs_two_frames() {
    let mut frames = vec![frame(0, -1)];
    assert!(evict_behind(&mut frames, None, 0, 8).is_none());
}

#[test]
fn evict_behind_keeps_retention_window_and_protects_current() {
    let mut frames = chain(0..=9);
    let removed = evict_behind(&mut frames, Some(2), 9, 3).unwrap();
    assert_eq!(stamps(&frames), vec![2, 6, 7, 8, 9]);
    assert_eq!(removed.len(), 5);
}

#[test]
fn evict_behind_reports_an_empty_removal_as_some() {
    let mut frames = chain(0..=1);
    let removed = evict_behind(&mut frames, None, 1, 8).unwrap();
    assert!(removed.is_empty());
    assert_eq!(stamps(&frames), vec![0, 1]);
}

#[test]
fn evict_one_refuses_without_current_or_when_the_frame_is_current() {
    let mut frames = chain(0..=1);
    assert!(evict_one(&mut frames, None, 0).is_none());
    assert!(evict_one(&mut frames, Some(0), 0).is_none());
    assert_eq!(evict_one(&mut frames, Some(1), 0).unwrap().timestamp, 0);
    assert_eq!(stamps(&frames), vec![1]);
}

#[test]
fn evict_one_refuses_a_single_frame() {
    let mut frames = chain(0..=0);
    assert!(evict_one(&mut frames, Some(5), 0).is_none());
    assert_eq!(stamps(&frames), vec![0]);
}
