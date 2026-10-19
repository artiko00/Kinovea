use crate::container::CacheAddResult;
use crate::frame::VideoFrame;
use crate::prebuffer::{PreBuffer, PreBufferConfig};

fn frame(ts: i64) -> VideoFrame {
    VideoFrame::new(None, ts, ts - 1)
}

fn filled(stamps: std::ops::RangeInclusive<i64>) -> PreBuffer {
    let pb = PreBuffer::new(PreBufferConfig::default());
    for ts in stamps {
        pb.add(frame(ts));
    }
    pb
}

#[test]
fn purge_keeps_only_current_and_publishes_when_something_was_removed() {
    let pb = filled(0..=4);
    pb.acquire_closest(2);
    let version = pb.snapshot().version;
    pb.purge();
    assert_eq!(pb.timestamps(), vec![2]);
    assert_eq!(pb.snapshot().version, version + 1);
    pb.purge();
    assert_eq!(pb.snapshot().version, version + 1);
}

#[test]
fn purge_without_current_removes_everything() {
    let pb = filled(0..=2);
    pb.purge();
    assert!(pb.timestamps().is_empty());
}

#[test]
fn shutdown_removes_current_and_refuses_adds_until_reset() {
    let pb = filled(0..=1);
    pb.acquire_closest(0);
    pb.shutdown();
    assert!(pb.timestamps().is_empty());
    assert_eq!(pb.current().map(|f| f.timestamp), None);
    assert_eq!(pb.add(frame(5)), CacheAddResult::Interrupted);
    pb.reset_interrupt_add();
    assert_eq!(pb.add(frame(5)), CacheAddResult::Added);
}

#[test]
fn clear_removes_current_without_interrupting_adds() {
    let pb = filled(0..=1);
    pb.acquire_closest(0);
    pb.clear();
    assert!(pb.timestamps().is_empty());
    assert_eq!(pb.current().map(|f| f.timestamp), None);
    assert_eq!(pb.add(frame(5)), CacheAddResult::Added);
}

#[test]
fn shutdown_publishes_even_when_empty() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    pb.shutdown();
    assert_eq!(pb.snapshot().version, 1);
}
