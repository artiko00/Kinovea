//! Port of `PreBuffer2.Add`, `ForceAdd`, `InterruptAdd` and `ResetInterruptAdd`.

use super::{PreBuffer, State};
use crate::closest::{index_of, insert_sorted};
use crate::container::CacheAddResult;
use crate::frame::VideoFrame;

impl PreBuffer {
    /// Adds a frame, blocking while the buffer is full, until room appears or the adds are
    /// interrupted.
    pub fn add(&self, frame: VideoFrame) -> CacheAddResult {
        let mut state = self.lock();
        loop {
            if let Some(result) = self.try_add_locked(&mut state, &frame) {
                return result;
            }
            state = self.ready.wait(state).expect("prebuffer condvar");
        }
    }

    /// Non-blocking `add`: `None` when the add would wait for room. A `None` result changes nothing.
    pub fn try_add(&self, frame: VideoFrame) -> Option<CacheAddResult> {
        let mut state = self.lock();
        self.try_add_locked(&mut state, &frame)
    }

    /// `Add` without blocking. Interrupt wins over fullness, and a duplicate is only reported
    /// after a zombie removal, as in the C# code.
    fn try_add_locked(&self, state: &mut State, frame: &VideoFrame) -> Option<CacheAddResult> {
        if state.interrupt_add {
            return Some(CacheAddResult::Interrupted);
        }
        if state.frames.len() >= self.config.capacity {
            if !self.is_zombie(state, frame) {
                return None;
            }
            state.frames.pop();
        }
        if !insert_sorted(&mut state.frames, frame.clone()) {
            return Some(CacheAddResult::Duplicate);
        }
        self.publish(state);
        Some(CacheAddResult::Added)
    }

    /// A zombie is the last frame when it lies far past the new frame and is not current. A backward
    /// seek pins the current frame, which then ends up ahead of the rest and never gets evicted.
    fn is_zombie(&self, state: &State, frame: &VideoFrame) -> bool {
        let Some(last) = state.frames.last() else {
            return false;
        };
        let current = state.current.as_ref().map(|c| c.timestamp);
        (last.timestamp - frame.timestamp) as f64 > self.config.far_ahead
            && current != Some(last.timestamp)
    }

    /// `ForceAdd`: never blocks and never checks the interrupt flag. When full it evicts the
    /// oldest frame, unless that frame is current.
    pub fn force_add(&self, frame: VideoFrame) -> CacheAddResult {
        let mut state = self.lock();
        if index_of(&state.frames, frame.timestamp).is_some() {
            return CacheAddResult::Duplicate;
        }
        if state.frames.len() >= self.config.capacity {
            let current = state.current.as_ref().map(|c| c.timestamp);
            super::evict::evict_one(&mut state.frames, current, 0);
        }
        insert_sorted(&mut state.frames, frame);
        self.publish(&mut state);
        CacheAddResult::Added
    }

    /// `InterruptAdd`: releases blocked adds, which report `Interrupted` until reset.
    pub fn interrupt_add(&self) {
        self.lock().interrupt_add = true;
        self.ready.notify_all();
    }

    /// `ResetInterruptAdd`.
    pub fn reset_interrupt_add(&self) {
        self.lock().interrupt_add = false;
    }
}
