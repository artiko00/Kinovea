//! Port of `IVideoFramesContainer`, `CacheAddResult`, `TryAcquireResult` and `CacheTimestampRelation`.

use crate::frame::VideoFrame;
use crate::snapshot::CacheSnapshot;
use kinovea_core::text_enum;
use std::sync::Arc;

text_enum!(
    /// `CacheAddResult`: the outcome of an add request.
    CacheAddResult [Added, Duplicate, Interrupted]
);

text_enum!(
    /// `CacheTimestampRelation`: where a request timestamp sits relative to the cached frames.
    CacheTimestampRelation [Empty, Behind, InBoundsMatch, InBoundsNoMatch, Ahead, FarAhead]
);

/// `TryAcquireResult`: whether the target of a player state was acquired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryAcquireResult {
    /// When true the current frame is now the match.
    pub target_acquired: bool,
    /// Timestamp of the match, -1 when nothing was acquired.
    pub acquired_timestamp: i64,
    /// Whether the cached frames can serve timestamp requests without a decode.
    pub is_contiguous: bool,
}

impl TryAcquireResult {
    pub fn new(target_acquired: bool, acquired_timestamp: i64, is_contiguous: bool) -> Self {
        Self {
            target_acquired,
            acquired_timestamp,
            is_contiguous,
        }
    }

    /// `TryAcquireResult.MakeEmpty()`.
    pub fn empty() -> Self {
        Self::new(false, -1, false)
    }
}

/// `IVideoFramesContainer`: what the player and the decoder do with a container.
/// `Cache` and `PreBuffer` implement it; `PreBuffer` also serves `&self` calls from two threads.
pub trait FramesContainer {
    /// The frame set by the last acquire, if any.
    fn current(&self) -> Option<VideoFrame>;

    /// The sections and version published after the last structural change.
    fn snapshot(&self) -> Arc<CacheSnapshot>;

    /// Timestamps of the stored frames in ascending order.
    fn timestamps(&self) -> Vec<i64>;

    /// Sets current to the frame closest to `timestamp`.
    fn acquire_closest(&mut self, timestamp: i64);

    /// Sets current to the frame after it. Returns false when there is none.
    fn acquire_next(&mut self) -> bool;

    fn add(&mut self, frame: VideoFrame) -> CacheAddResult;

    fn force_add(&mut self, frame: VideoFrame) -> CacheAddResult;

    /// Removes every frame, including the current one.
    fn clear(&mut self);

    /// Removes every frame, including the current one. The dense cache only clears here.
    fn shutdown(&mut self);
}
