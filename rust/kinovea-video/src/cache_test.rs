use crate::cache::Cache;
use crate::container::{CacheAddResult, FramesContainer};
use crate::frame::VideoFrame;

fn frame(ts: i64) -> VideoFrame {
    VideoFrame::new(None, ts, ts - 1)
}

fn cache_with(stamps: &[i64]) -> Cache {
    let mut cache = Cache::new();
    for &ts in stamps {
        cache.add(frame(ts));
    }
    cache
}

fn current_ts(cache: &Cache) -> Option<i64> {
    cache.current().map(|f| f.timestamp)
}

#[test]
fn add_stores_sorted_and_rejects_duplicates_without_publishing() {
    let mut cache = cache_with(&[20, 10]);
    assert_eq!(cache.timestamps(), vec![10, 20]);
    assert_eq!(cache.add(frame(10)), CacheAddResult::Duplicate);
    assert_eq!(cache.snapshot().version, 2);
}

#[test]
fn acquire_closest_picks_the_earlier_frame_on_ties() {
    let mut cache = cache_with(&[10, 20]);
    cache.acquire_closest(15);
    assert_eq!(current_ts(&cache), Some(10));
    cache.acquire_closest(25);
    assert_eq!(current_ts(&cache), Some(20));
}

#[test]
fn acquire_closest_on_empty_cache_clears_current() {
    let mut cache = Cache::new();
    cache.acquire_closest(5);
    assert_eq!(current_ts(&cache), None);
}

#[test]
fn acquire_next_stops_at_the_last_frame() {
    let mut cache = cache_with(&[0, 10]);
    cache.acquire_closest(0);
    assert!(cache.acquire_next());
    assert_eq!(current_ts(&cache), Some(10));
    assert!(!cache.acquire_next());
}

#[test]
fn acquire_next_needs_two_frames_and_a_current_frame() {
    assert!(!cache_with(&[0]).acquire_next());
    assert!(!cache_with(&[0, 10]).acquire_next());
}

#[test]
fn trim_keeps_current_in_range_and_publishes_one_section() {
    let mut cache = cache_with(&[0, 10, 20, 30]);
    cache.acquire_closest(10);
    cache.trim(10, 20);
    assert_eq!(cache.timestamps(), vec![10, 20]);
    assert_eq!(current_ts(&cache), Some(10));
    assert_eq!(cache.snapshot().version, 5);
    assert_eq!(cache.snapshot().to_string(), "[10;20] ");
}

#[test]
fn trim_moves_current_to_first_frame_when_it_falls_outside() {
    let mut cache = cache_with(&[0, 10, 20]);
    cache.acquire_closest(0);
    cache.trim(10, 20);
    assert_eq!(current_ts(&cache), Some(10));
}

#[test]
fn trim_outside_every_frame_empties_the_cache() {
    let mut cache = cache_with(&[0, 10]);
    cache.acquire_closest(10);
    cache.trim(50, 60);
    assert!(cache.timestamps().is_empty());
    assert_eq!(current_ts(&cache), None);
}

#[test]
fn clear_removes_current_and_publishes() {
    let mut cache = cache_with(&[0, 10]);
    cache.acquire_closest(0);
    cache.clear();
    assert!(cache.timestamps().is_empty());
    assert_eq!(current_ts(&cache), None);
    assert_eq!(cache.snapshot().version, 3);
}

#[test]
fn shutdown_only_clears_because_the_dense_cache_has_no_adds_to_refuse() {
    let mut cache = cache_with(&[0, 10]);
    cache.acquire_closest(0);
    cache.shutdown();
    assert!(cache.timestamps().is_empty());
    assert_eq!(current_ts(&cache), None);
    assert_eq!(cache.add(frame(5)), CacheAddResult::Added);
}

#[test]
fn working_zone_spans_first_to_last_or_is_empty() {
    let zone = Cache::new().working_zone();
    assert_eq!((zone.start, zone.end), (-1, -1));
    let zone = cache_with(&[0, 10, 30]).working_zone();
    assert_eq!((zone.start, zone.end), (0, 30));
}

#[test]
fn snapshot_version_grows_on_structural_changes_only() {
    let mut cache = Cache::new();
    assert_eq!(cache.snapshot().version, -1);
    cache.add(frame(0));
    assert_eq!(cache.snapshot().version, 1);
    assert_eq!(cache.snapshot().to_string(), "[0;0] ");
    cache.acquire_closest(0);
    assert_eq!(cache.snapshot().version, 1);
}

#[test]
fn force_add_behaves_like_add() {
    let mut cache = cache_with(&[0]);
    assert_eq!(cache.force_add(frame(10)), CacheAddResult::Added);
    assert_eq!(cache.force_add(frame(10)), CacheAddResult::Duplicate);
    assert_eq!(cache.timestamps(), vec![0, 10]);
}
