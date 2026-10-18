//! A range of video timestamps (port of `VideoSection.cs`).

use std::cmp::Ordering;
use std::fmt;

/// Start and end timestamps of a segment. Empty when `end < 0`, wrapped when the end
/// precedes the start (the segment goes through the end of the video).
#[derive(Debug, Clone, Copy)]
pub struct VideoSection {
    pub start: i64,
    pub end: i64,
}

impl VideoSection {
    pub const fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    /// `[-1, -1]`, the empty section.
    pub const fn empty() -> Self {
        Self::new(-1, -1)
    }

    pub fn is_empty(&self) -> bool {
        self.end < 0
    }

    pub fn wrapped(&self) -> bool {
        !self.is_empty() && self.end < self.start
    }

    /// `end - start`, or 0 for the empty section. Negative when wrapped, like the original.
    pub fn length(&self) -> i64 {
        if self.is_empty() {
            0
        } else {
            self.end - self.start
        }
    }

    /// Inclusive bounds; not defined when wrapped (returns the raw comparison).
    pub fn contains(&self, timestamp: i64) -> bool {
        !self.is_empty() && timestamp >= self.start && timestamp <= self.end
    }

    /// The other section lies within or equals this one.
    pub fn contains_section(&self, other: VideoSection) -> bool {
        !self.is_empty() && other <= *self
    }
}

impl Default for VideoSection {
    /// `[0, 99]`, the default section.
    fn default() -> Self {
        Self::new(0, 99)
    }
}

impl PartialEq for VideoSection {
    /// Two empty sections are never equal, matching `VideoSection.Equals`.
    fn eq(&self, other: &Self) -> bool {
        !self.is_empty() && self.start == other.start && self.end == other.end
    }
}

impl PartialOrd for VideoSection {
    /// A section is less than another when it is entirely enclosed in it.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.start == other.start && self.end == other.end {
            Some(Ordering::Equal)
        } else if self.start >= other.start && self.end <= other.end {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl fmt::Display for VideoSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.wrapped() {
            write!(f, "? --> {}] [{} --> ?", self.end, self.start)
        } else {
            write!(f, "[{} --> {}]", self.start, self.end)
        }
    }
}
