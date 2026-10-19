//! Port of `Kinovea.Video/Caching/CacheSnapshot.cs` and of the span computation in
//! `Cache.UpdateCacheSnapshot` and `PreBuffer2.UpdateCacheSnapshot`.

use crate::frame::VideoFrame;
use kinovea_core::video_section::VideoSection;
use std::fmt;

/// The cache state at one moment: a version bumped on each structural change, and the
/// sections that are cached. Used for feedback in the UI.
#[derive(Debug, Clone)]
pub struct CacheSnapshot {
    pub version: i64,
    pub sections: Vec<VideoSection>,
}

impl CacheSnapshot {
    pub fn new(version: i64, sections: Vec<VideoSection>) -> Self {
        Self { version, sections }
    }

    /// `CacheSnapshot.MakeEmpty()`: version -1 and no sections.
    pub fn empty() -> Self {
        Self::new(-1, Vec::new())
    }
}

impl fmt::Display for CacheSnapshot {
    /// Each section as `[start;end] `, trailing space included, like the C# `ToString`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for section in &self.sections {
            write!(f, "[{};{}] ", section.start, section.end)?;
        }
        Ok(())
    }
}

/// Sections of the dense cache: one span from the first to the last frame.
pub fn single_span(frames: &[VideoFrame]) -> Vec<VideoSection> {
    match (frames.first(), frames.last()) {
        (Some(first), Some(last)) => vec![VideoSection::new(first.timestamp, last.timestamp)],
        _ => Vec::new(),
    }
}

/// Sections of a prebuffer: a new span starts wherever a frame does not follow its predecessor.
pub fn contiguous_spans(frames: &[VideoFrame]) -> Vec<VideoSection> {
    let Some(first) = frames.first() else {
        return Vec::new();
    };
    let mut spans = Vec::new();
    let (mut start, mut end) = (first.timestamp, first.timestamp);
    for pair in frames.windows(2) {
        let (prev, cur) = (&pair[0], &pair[1]);
        if cur.previous_timestamp == prev.timestamp {
            end = cur.timestamp;
        } else {
            spans.push(VideoSection::new(start, end));
            (start, end) = (cur.timestamp, cur.timestamp);
        }
    }
    spans.push(VideoSection::new(start, end));
    spans
}

/// `IsContiguous`: every frame follows its predecessor.
pub fn is_contiguous(frames: &[VideoFrame]) -> bool {
    frames
        .windows(2)
        .all(|pair| pair[1].previous_timestamp == pair[0].timestamp)
}
