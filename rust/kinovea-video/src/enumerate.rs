//! Port of `VideoReader.EnumerateFrames` (`Kinovea.Video/VideoReaders/VideoReader.cs`). The C#
//! `yield` state machine becomes an iterator. The caller disables prebuffering before and
//! re-enables it after, through the reader's enumeration hooks.

use crate::frame::VideoFrame;
use crate::reader::VideoReader;

/// Walks the working zone of a reader, yielding a copy of each frame it lands on.
/// `interval` 0 visits every frame; a positive interval visits the frame nearest each step.
pub struct FrameEnumerator<'a, R: VideoReader + ?Sized> {
    reader: &'a mut R,
    interval: f64,
    started: bool,
    done: bool,
}

impl<'a, R: VideoReader + ?Sized> FrameEnumerator<'a, R> {
    pub fn new(reader: &'a mut R, interval: f64) -> Self {
        Self {
            reader,
            interval,
            started: false,
            done: false,
        }
    }

    /// Moves the reader to the next frame of the walk. False ends the walk.
    fn advance(&mut self) -> bool {
        if !self.started {
            self.started = true;
            let start = self.reader.working_zone().start;
            return self.reader.move_to(start);
        }
        if self.interval == 0.0 {
            return self.reader.move_next();
        }
        let current = self.reader.current().timestamp as f64;
        let target = (current + self.interval).round_ties_even() as i64;
        if target > self.reader.working_zone().end {
            return false;
        }
        self.reader.move_to(target)
    }
}

impl<R: VideoReader + ?Sized> Iterator for FrameEnumerator<'_, R> {
    type Item = VideoFrame;

    fn next(&mut self) -> Option<VideoFrame> {
        if self.done || !self.advance() {
            self.done = true;
            return None;
        }
        Some(self.reader.current().clone())
    }
}
