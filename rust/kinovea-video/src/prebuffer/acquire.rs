//! Port of `PreBuffer2.AcquireClosest` and `AcquireNext`. `TryAcquire` lives in `try_acquire`.

use super::evict::evict_behind;
use super::{position_of_current, PreBuffer, State};
use crate::closest::find_closest;

impl PreBuffer {
    /// Makes the closest frame current, evicting behind it. Changes nothing when that frame
    /// already is current.
    pub fn acquire_closest(&self, timestamp: i64) {
        let mut state = self.lock();
        let Some(index) = find_closest(&state.frames, timestamp) else {
            state.current = None;
            return;
        };
        if state.current.as_ref().map(|c| c.timestamp) == Some(state.frames[index].timestamp) {
            return;
        }
        state.current = Some(state.frames[index].clone());
        if self.evict_around(&mut state, index) {
            self.wake_adds();
        }
    }

    /// Makes the frame after current the new current, evicting behind it. Without a current frame
    /// the first frame is taken. Returns false when there is no frame after current.
    pub fn acquire_next(&self) -> bool {
        let mut state = self.lock();
        if state.frames.len() < 2 {
            return false;
        }
        let next = position_of_current(&state).map_or(0, |i| i + 1);
        if next >= state.frames.len() {
            return false;
        }
        state.current = Some(state.frames[next].clone());
        if self.evict_around(&mut state, next) {
            self.wake_adds();
        }
        true
    }

    /// `EvictBehind` at `pivot` with the configured retention, publishing when it ran. Returns
    /// whether it ran, which is when C# `removed != null` and the caller pulses.
    pub(super) fn evict_around(&self, state: &mut State, pivot: usize) -> bool {
        let current = state.current.as_ref().map(|c| c.timestamp);
        let ran = evict_behind(&mut state.frames, current, pivot, self.config.retention).is_some();
        if ran {
            self.publish(state);
        }
        ran
    }
}
