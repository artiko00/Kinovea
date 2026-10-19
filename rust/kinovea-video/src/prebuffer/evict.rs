//! Port of the eviction helpers of `PreBuffer2`: `EvictBehind` and `EvictOne`.
//! They work on the sorted frames and never remove the current frame, which is identified by
//! its timestamp (timestamps are unique in the buffer). The caller publishes the changes.

use crate::frame::VideoFrame;

/// `EvictBehind`: keeps at most `keep` frames before `pivot`, plus the pivot and everything after it.
/// `None` when there are fewer than two frames. `Some` with an empty list when nothing was removed.
pub fn evict_behind(
    frames: &mut Vec<VideoFrame>,
    current: Option<i64>,
    pivot: usize,
    keep: usize,
) -> Option<Vec<VideoFrame>> {
    if frames.len() < 2 {
        return None;
    }
    let retain_from = pivot.saturating_sub(keep);
    Some(remove_descending(frames, current, (0..retain_from).rev()))
}

// `EvictNonContiguous` is not ported: the C# `TryAcquire` only calls it in a comment
// (`PreBuffer2.cs:455`), so no observable behavior depends on it.

/// `EvictOne`: removes the frame at `index`, unless there is no current frame, fewer than two
/// frames, or the frame at `index` is the current one.
pub fn evict_one(
    frames: &mut Vec<VideoFrame>,
    current: Option<i64>,
    index: usize,
) -> Option<VideoFrame> {
    if current.is_none() || frames.len() < 2 || current == Some(frames[index].timestamp) {
        return None;
    }
    Some(frames.remove(index))
}

/// Removes the frames at `indices`, which must come in descending order so that the remaining
/// indices stay valid. The current frame is skipped.
fn remove_descending(
    frames: &mut Vec<VideoFrame>,
    current: Option<i64>,
    indices: impl Iterator<Item = usize>,
) -> Vec<VideoFrame> {
    let mut removed = Vec::new();
    for i in indices {
        if current == Some(frames[i].timestamp) {
            continue;
        }
        removed.push(frames.remove(i));
    }
    removed
}
