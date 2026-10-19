//! Port of `Kinovea.Video/Caching/PreBuffer2.cs`: a bounded cache filled by the decoding thread
//! and read by the player thread. Adds block while the buffer is full. The methods take `&self`
//! so one `Arc<PreBuffer>` can be shared between the two threads.

use crate::closest::{index_of, relate_timestamp};
use crate::container::CacheTimestampRelation;
use crate::frame::VideoFrame;
use crate::snapshot::{contiguous_spans, is_contiguous, CacheSnapshot};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

pub mod acquire;
#[cfg(test)]
mod acquire_test;
pub mod add;
#[cfg(test)]
mod add_test;
pub mod evict;
#[cfg(test)]
mod evict_test;
mod frames_container;
#[cfg(test)]
mod frames_container_test;
pub mod release;
#[cfg(test)]
mod release_test;
pub mod try_acquire;
#[cfg(test)]
mod try_acquire_test;

/// `PreBuffer2` settings. The C# capacity is only reachable by reflection, so it is set here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PreBufferConfig {
    /// Maximum number of frames before an add blocks.
    pub capacity: usize,
    /// Frames kept behind current when acquiring.
    pub retention: usize,
    /// Half the average timestamps per frame: how far a request may be from a frame and still match.
    pub tolerance: f64,
    /// Distance past the last frame beyond which a request is far ahead.
    pub far_ahead: f64,
}

impl Default for PreBufferConfig {
    /// Capacity 32, retention 8, no tolerance and no far-ahead threshold, as in `PreBuffer2`.
    fn default() -> Self {
        Self {
            capacity: 32,
            retention: 8,
            tolerance: 0.0,
            far_ahead: 0.0,
        }
    }
}

/// Everything guarded by the lock.
#[derive(Debug)]
struct State {
    frames: Vec<VideoFrame>,
    current: Option<VideoFrame>,
    interrupt_add: bool,
    version: i64,
}

/// Bounded frame cache shared by the decoder and the player.
#[derive(Debug)]
pub struct PreBuffer {
    config: PreBufferConfig,
    state: Mutex<State>,
    /// Wakes adds blocked on a full buffer whenever the frames change.
    ready: Condvar,
    /// Read without taking `state`, so the player never waits for the decoder's bookkeeping.
    published: Mutex<Arc<CacheSnapshot>>,
}

impl PreBuffer {
    pub fn new(config: PreBufferConfig) -> Self {
        Self {
            config,
            state: Mutex::new(State {
                frames: Vec::new(),
                current: None,
                interrupt_add: false,
                version: 0,
            }),
            ready: Condvar::new(),
            published: Mutex::new(Arc::new(CacheSnapshot::empty())),
        }
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().expect("prebuffer state lock")
    }

    /// `UpdateCacheSnapshot`: bumps the version, publishes the sections and wakes blocked adds.
    /// Called after every change that the C# code publishes.
    fn publish(&self, state: &mut State) {
        state.version += 1;
        let snapshot = CacheSnapshot::new(state.version, contiguous_spans(&state.frames));
        *self.published.lock().expect("prebuffer snapshot lock") = Arc::new(snapshot);
        self.ready.notify_all();
    }

    pub fn current(&self) -> Option<VideoFrame> {
        self.lock().current.clone()
    }

    pub fn snapshot(&self) -> Arc<CacheSnapshot> {
        Arc::clone(&self.published.lock().expect("prebuffer snapshot lock"))
    }

    pub fn timestamps(&self) -> Vec<i64> {
        self.lock().frames.iter().map(|f| f.timestamp).collect()
    }

    pub fn is_contiguous(&self) -> bool {
        is_contiguous(&self.lock().frames)
    }

    pub fn relate_timestamp(&self, target: i64) -> CacheTimestampRelation {
        let state = self.lock();
        relate_timestamp(
            &state.frames,
            target,
            self.config.tolerance,
            self.config.far_ahead,
        )
    }
}

/// Position of current among the frames. `None` when there is no current frame.
fn position_of_current(state: &State) -> Option<usize> {
    let current = state.current.as_ref()?;
    index_of(&state.frames, current.timestamp)
}
