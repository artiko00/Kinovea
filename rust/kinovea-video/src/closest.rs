//! Helpers shared by `Cache` and `PreBuffer`, over frames sorted by timestamp.
//! Port of `FindClosest`, `LowerBound` and `RelateTimestamp`, which the C# containers duplicate.

use crate::container::CacheTimestampRelation;
use crate::frame::VideoFrame;

/// Index of the first frame whose timestamp is not before `timestamp`.
pub fn lower_bound(frames: &[VideoFrame], timestamp: i64) -> usize {
    frames.partition_point(|f| f.timestamp < timestamp)
}

/// Index of the frame closest to `timestamp`. Ties go to the earlier frame. `None` when empty.
pub fn find_closest(frames: &[VideoFrame], timestamp: i64) -> Option<usize> {
    if frames.is_empty() {
        return None;
    }
    let index = lower_bound(frames, timestamp);
    if index == 0 {
        Some(0)
    } else if index == frames.len() {
        Some(frames.len() - 1)
    } else {
        let delta_before = timestamp - frames[index - 1].timestamp;
        let delta_after = frames[index].timestamp - timestamp;
        Some(if delta_before <= delta_after {
            index - 1
        } else {
            index
        })
    }
}

/// Index of the frame stored with exactly this timestamp.
pub fn index_of(frames: &[VideoFrame], timestamp: i64) -> Option<usize> {
    frames
        .binary_search_by_key(&timestamp, |f| f.timestamp)
        .ok()
}

/// Inserts the frame in timestamp order. Returns false, changing nothing, when a frame with
/// the same timestamp is already stored.
pub fn insert_sorted(frames: &mut Vec<VideoFrame>, frame: VideoFrame) -> bool {
    match frames.binary_search_by_key(&frame.timestamp, |f| f.timestamp) {
        Ok(_) => false,
        Err(at) => {
            frames.insert(at, frame);
            true
        }
    }
}

/// `RelateTimestamp`: where `target` sits relative to the frames. `tolerance` decides matches,
/// and `far_ahead` separates ahead from far ahead.
pub fn relate_timestamp(
    frames: &[VideoFrame],
    target: i64,
    tolerance: f64,
    far_ahead: f64,
) -> CacheTimestampRelation {
    let Some(closest) = find_closest(frames, target) else {
        return CacheTimestampRelation::Empty;
    };
    if ((frames[closest].timestamp - target).abs() as f64) <= tolerance {
        return CacheTimestampRelation::InBoundsMatch;
    }
    let first = frames[0].timestamp;
    let last = frames[frames.len() - 1].timestamp;
    if target < first {
        CacheTimestampRelation::Behind
    } else if target > last {
        if (target - last) as f64 > far_ahead {
            CacheTimestampRelation::FarAhead
        } else {
            CacheTimestampRelation::Ahead
        }
    } else {
        CacheTimestampRelation::InBoundsNoMatch
    }
}
