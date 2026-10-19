use crate::container::CacheAddResult;
use crate::frame::VideoFrame;
use crate::prebuffer::{PreBuffer, PreBufferConfig};

fn frame(ts: i64, prev: i64) -> VideoFrame {
    VideoFrame::new(None, ts, prev)
}

fn with_capacity(capacity: usize, far_ahead: f64) -> PreBuffer {
    PreBuffer::new(PreBufferConfig {
        capacity,
        far_ahead,
        ..PreBufferConfig::default()
    })
}

#[test]
fn try_add_reports_none_when_full_and_changes_nothing() {
    let pb = with_capacity(2, 0.0);
    pb.add(frame(0, -1));
    pb.add(frame(1, 0));
    let version = pb.snapshot().version;
    assert_eq!(pb.try_add(frame(2, 1)), None);
    assert_eq!(pb.timestamps(), vec![0, 1]);
    assert_eq!(pb.snapshot().version, version);
}

#[test]
fn try_add_removes_a_far_ahead_zombie_to_make_room() {
    let pb = with_capacity(2, 3.0);
    pb.add(frame(0, -1));
    pb.add(frame(50, 49));
    // The zombie removal does not publish; the add publishes once.
    assert_eq!(pb.try_add(frame(3, 2)), Some(CacheAddResult::Added));
    assert_eq!(pb.timestamps(), vec![0, 3]);
    assert_eq!(pb.snapshot().version, 3);
}

#[test]
fn zombie_that_is_current_is_not_removed() {
    let pb = with_capacity(2, 3.0);
    pb.add(frame(0, -1));
    pb.add(frame(50, 49));
    pb.acquire_closest(50);
    assert_eq!(pb.try_add(frame(3, 2)), None);
    assert_eq!(pb.timestamps(), vec![0, 50]);
}

#[test]
fn interrupt_is_reported_before_fullness() {
    let pb = with_capacity(2, 0.0);
    pb.interrupt_add();
    assert_eq!(pb.try_add(frame(0, -1)), Some(CacheAddResult::Interrupted));
    pb.reset_interrupt_add();
    assert_eq!(pb.try_add(frame(0, -1)), Some(CacheAddResult::Added));
}

#[test]
fn force_add_evicts_the_oldest_frame_that_is_not_current() {
    let pb = with_capacity(2, 0.0);
    pb.add(frame(0, -1));
    pb.add(frame(1, 0));
    pb.acquire_closest(1);
    assert_eq!(pb.force_add(frame(2, 1)), CacheAddResult::Added);
    assert_eq!(pb.timestamps(), vec![1, 2]);
}

#[test]
fn force_add_keeps_current_at_the_front_and_exceeds_capacity() {
    let pb = with_capacity(2, 0.0);
    pb.add(frame(0, -1));
    pb.add(frame(1, 0));
    pb.acquire_closest(0);
    assert_eq!(pb.force_add(frame(2, 1)), CacheAddResult::Added);
    assert_eq!(pb.timestamps(), vec![0, 1, 2]);
}

#[test]
fn force_add_without_current_does_not_evict() {
    let pb = with_capacity(2, 0.0);
    pb.add(frame(0, -1));
    pb.add(frame(1, 0));
    pb.force_add(frame(2, 1));
    assert_eq!(pb.timestamps(), vec![0, 1, 2]);
}

#[test]
fn force_add_reports_duplicates_without_publishing() {
    let pb = with_capacity(2, 0.0);
    pb.add(frame(0, -1));
    let version = pb.snapshot().version;
    assert_eq!(pb.force_add(frame(0, -1)), CacheAddResult::Duplicate);
    assert_eq!(pb.snapshot().version, version);
}
