use crate::frame::VideoFrame;
use crate::prebuffer::{PreBuffer, PreBufferConfig};

fn frame(ts: i64, prev: i64) -> VideoFrame {
    VideoFrame::new(None, ts, prev)
}

fn fill(pb: &PreBuffer, stamps: std::ops::RangeInclusive<i64>) {
    for ts in stamps {
        pb.add(frame(ts, ts - 1));
    }
}

fn current_ts(pb: &PreBuffer) -> Option<i64> {
    pb.current().map(|f| f.timestamp)
}

#[test]
fn acquire_closest_evicts_behind_the_retention_window() {
    let pb = PreBuffer::new(PreBufferConfig {
        retention: 3,
        ..PreBufferConfig::default()
    });
    fill(&pb, 0..=19);
    pb.acquire_closest(15);
    assert_eq!(current_ts(&pb), Some(15));
    assert_eq!(pb.timestamps(), (12..=19).collect::<Vec<_>>());
}

#[test]
fn acquire_closest_on_the_current_frame_changes_nothing() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    fill(&pb, 0..=5);
    pb.acquire_closest(3);
    let version = pb.snapshot().version;
    pb.acquire_closest(3);
    assert_eq!(pb.snapshot().version, version);
}

#[test]
fn acquire_closest_on_empty_buffer_leaves_no_current() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    pb.acquire_closest(3);
    assert_eq!(current_ts(&pb), None);
}

#[test]
fn acquire_next_without_current_takes_the_first_frame() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    fill(&pb, 0..=2);
    assert!(pb.acquire_next());
    assert_eq!(current_ts(&pb), Some(0));
}

#[test]
fn acquire_next_moves_forward_and_stops_at_the_end() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    fill(&pb, 0..=2);
    pb.acquire_closest(1);
    assert!(pb.acquire_next());
    assert_eq!(current_ts(&pb), Some(2));
    assert!(!pb.acquire_next());
}

#[test]
fn acquire_next_evicts_behind_the_new_current_within_retention() {
    let pb = PreBuffer::new(PreBufferConfig {
        retention: 2,
        ..PreBufferConfig::default()
    });
    fill(&pb, 0..=9);
    pb.acquire_closest(5);
    assert_eq!(pb.timestamps(), (3..=9).collect::<Vec<_>>());
    assert!(pb.acquire_next());
    assert_eq!(current_ts(&pb), Some(6));
    assert_eq!(pb.timestamps(), (4..=9).collect::<Vec<_>>());
}
