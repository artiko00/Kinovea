//! `FramesContainer` for `PreBuffer`: each trait method delegates to the inherent method of the
//! same name, which takes `&self` so the decoder and the player can share the buffer.

use super::PreBuffer;
use crate::container::{CacheAddResult, FramesContainer};
use crate::frame::VideoFrame;
use crate::snapshot::CacheSnapshot;
use std::sync::Arc;

impl FramesContainer for PreBuffer {
    fn current(&self) -> Option<VideoFrame> {
        PreBuffer::current(self)
    }

    fn snapshot(&self) -> Arc<CacheSnapshot> {
        PreBuffer::snapshot(self)
    }

    fn timestamps(&self) -> Vec<i64> {
        PreBuffer::timestamps(self)
    }

    fn acquire_closest(&mut self, timestamp: i64) {
        PreBuffer::acquire_closest(self, timestamp)
    }

    fn acquire_next(&mut self) -> bool {
        PreBuffer::acquire_next(self)
    }

    fn add(&mut self, frame: VideoFrame) -> CacheAddResult {
        PreBuffer::add(self, frame)
    }

    fn force_add(&mut self, frame: VideoFrame) -> CacheAddResult {
        PreBuffer::force_add(self, frame)
    }

    fn clear(&mut self) {
        PreBuffer::clear(self)
    }

    fn shutdown(&mut self) {
        PreBuffer::shutdown(self)
    }
}
