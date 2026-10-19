//! Port of `Kinovea.Video/Caching/Cache.cs`: the dense cache of a whole working zone.
//! Single-threaded (UI thread): acquires are synchronous and nothing is evicted.

use crate::closest::{find_closest, index_of, insert_sorted};
use crate::container::{CacheAddResult, FramesContainer};
use crate::frame::VideoFrame;
use crate::snapshot::{single_span, CacheSnapshot};
use kinovea_core::video_section::VideoSection;
use std::sync::Arc;

/// Frames sorted by timestamp. A `Vec` keeps the positional arithmetic of the C# `SortedList`.
#[derive(Debug)]
pub struct Cache {
    frames: Vec<VideoFrame>,
    current: Option<VideoFrame>,
    version: i64,
    snapshot: Arc<CacheSnapshot>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            current: None,
            version: 0,
            snapshot: Arc::new(CacheSnapshot::empty()),
        }
    }

    /// `WorkingZone`: first to last timestamp, or the empty section.
    pub fn working_zone(&self) -> VideoSection {
        match (self.frames.first(), self.frames.last()) {
            (Some(first), Some(last)) => VideoSection::new(first.timestamp, last.timestamp),
            _ => VideoSection::empty(),
        }
    }

    /// Removes the frames outside `[start, end]`. Current stays when it is in range,
    /// otherwise it moves to the first remaining frame.
    pub fn trim(&mut self, start: i64, end: i64) {
        let section = VideoSection::new(start, end);
        let current_in_range = self
            .current
            .as_ref()
            .is_some_and(|c| section.contains(c.timestamp));
        self.frames.retain(|f| section.contains(f.timestamp));
        if !current_in_range {
            self.current = self.frames.first().cloned();
        }
        self.publish();
    }

    /// `UpdateCacheSnapshot`: bumps the version and publishes the single span.
    fn publish(&mut self) {
        self.version += 1;
        self.snapshot = Arc::new(CacheSnapshot::new(self.version, single_span(&self.frames)));
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl FramesContainer for Cache {
    fn current(&self) -> Option<VideoFrame> {
        self.current.clone()
    }

    fn snapshot(&self) -> Arc<CacheSnapshot> {
        Arc::clone(&self.snapshot)
    }

    fn timestamps(&self) -> Vec<i64> {
        self.frames.iter().map(|f| f.timestamp).collect()
    }

    /// No early return: the C# dense cache sets current even when it is already the match.
    fn acquire_closest(&mut self, timestamp: i64) {
        self.current = find_closest(&self.frames, timestamp).map(|i| self.frames[i].clone());
    }

    fn acquire_next(&mut self) -> bool {
        if self.frames.len() < 2 {
            return false;
        }
        let Some(current) = &self.current else {
            return false;
        };
        let next = index_of(&self.frames, current.timestamp).map(|i| i + 1);
        match next {
            Some(next) if next < self.frames.len() => {
                self.current = Some(self.frames[next].clone());
                true
            }
            _ => false,
        }
    }

    fn add(&mut self, frame: VideoFrame) -> CacheAddResult {
        if !insert_sorted(&mut self.frames, frame) {
            return CacheAddResult::Duplicate;
        }
        self.publish();
        CacheAddResult::Added
    }

    fn force_add(&mut self, frame: VideoFrame) -> CacheAddResult {
        self.add(frame)
    }

    fn clear(&mut self) {
        self.frames.clear();
        self.current = None;
        self.publish();
    }

    /// Design D3: the Rust dense cache clears. The C# method throws `NotImplementedException`.
    fn shutdown(&mut self) {
        self.clear();
    }
}
