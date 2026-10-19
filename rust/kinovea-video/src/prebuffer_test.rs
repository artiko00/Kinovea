use crate::container::{CacheAddResult, CacheTimestampRelation};
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

#[test]
fn default_config_matches_prebuffer2() {
    let config = PreBufferConfig::default();
    assert_eq!(config.capacity, 32);
    assert_eq!(config.retention, 8);
    assert_eq!(config.tolerance, 0.0);
    assert_eq!(config.far_ahead, 0.0);
}

#[test]
fn add_publishes_one_version_per_structural_change() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    assert_eq!(pb.snapshot().version, -1);
    pb.add(frame(0, 0));
    assert_eq!(pb.snapshot().version, 1);
    assert_eq!(pb.snapshot().to_string(), "[0;0] ");
    pb.add(frame(1, 0));
    assert_eq!(pb.snapshot().version, 2);
    assert_eq!(pb.snapshot().to_string(), "[0;1] ");
}

#[test]
fn non_contiguous_add_starts_a_new_section() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    pb.add(frame(0, 0));
    pb.add(frame(5, 4));
    assert_eq!(pb.snapshot().to_string(), "[0;0] [5;5] ");
    assert!(!pb.is_contiguous());
}

#[test]
fn duplicate_add_is_reported_without_a_version_change() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    pb.add(frame(0, 0));
    assert_eq!(pb.add(frame(0, 0)), CacheAddResult::Duplicate);
    assert_eq!(pb.snapshot().version, 1);
}

#[test]
fn current_is_absent_until_acquired() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    fill(&pb, 0..=2);
    assert!(pb.current().is_none());
    pb.acquire_closest(2);
    assert_eq!(pb.current().map(|f| f.timestamp), Some(2));
}

#[test]
fn relate_uses_tolerance_and_far_ahead_threshold() {
    let pb = PreBuffer::new(PreBufferConfig {
        tolerance: 0.5,
        far_ahead: 50.0,
        ..PreBufferConfig::default()
    });
    assert_eq!(pb.relate_timestamp(0), CacheTimestampRelation::Empty);
    pb.add(frame(0, 0));
    pb.add(frame(10, 0));
    assert_eq!(
        pb.relate_timestamp(10),
        CacheTimestampRelation::InBoundsMatch
    );
    assert_eq!(
        pb.relate_timestamp(5),
        CacheTimestampRelation::InBoundsNoMatch
    );
    assert_eq!(pb.relate_timestamp(-1), CacheTimestampRelation::Behind);
    assert_eq!(pb.relate_timestamp(60), CacheTimestampRelation::Ahead);
    assert_eq!(pb.relate_timestamp(61), CacheTimestampRelation::FarAhead);
}

#[test]
fn a_snapshot_taken_earlier_keeps_its_version() {
    let pb = PreBuffer::new(PreBufferConfig::default());
    pb.add(frame(0, 0));
    let before = pb.snapshot();
    pb.add(frame(1, 0));
    assert_eq!(before.version, 1);
    assert_eq!(pb.snapshot().version, 2);
}
