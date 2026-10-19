//! Port of `PreBuffer2.TryAcquire`: acquires the target of a player state, then evicts around it.

use super::{position_of_current, PreBuffer, State};
use crate::closest::find_closest;
use crate::container::TryAcquireResult;
use crate::snapshot::is_contiguous;
use crate::state::{PlayerAction, PlayerState};

impl PreBuffer {
    /// Tries to acquire the target of a player state. The eviction runs at the candidate even when
    /// the target is not matched.
    pub fn try_acquire(&self, request: &PlayerState) -> TryAcquireResult {
        let mut state = self.lock();
        let Some((pivot, matched)) = self.candidate(&state, request) else {
            return TryAcquireResult::empty();
        };
        let acquired_timestamp = if matched {
            state.current = Some(state.frames[pivot].clone());
            state.frames[pivot].timestamp
        } else {
            -1
        };
        self.evict_around(&mut state, pivot);
        let contiguous = is_contiguous(&state.frames);
        TryAcquireResult::new(matched, acquired_timestamp, contiguous)
    }

    /// The frame the request points at and whether it matches. `None` when the C# code returns
    /// early: nothing is buffered, or a step has no current frame or no neighbour.
    fn candidate(&self, state: &State, request: &PlayerState) -> Option<(usize, bool)> {
        if state.frames.is_empty() {
            return None;
        }
        match request.action {
            PlayerAction::StepForward => step_candidate(state, true),
            PlayerAction::StepBackward => step_candidate(state, false),
            _ => {
                let index = find_closest(&state.frames, request.reference_timestamp)?;
                let distance = state.frames[index].timestamp - request.reference_timestamp;
                Some((index, distance.abs() as f64 <= self.config.tolerance))
            }
        }
    }
}

/// Neighbour of current in the given direction. It matches only when the chain links the two frames.
fn step_candidate(state: &State, forward: bool) -> Option<(usize, bool)> {
    let current = position_of_current(state)?;
    if state.frames.len() < 2 {
        return None;
    }
    let index = if forward {
        current + 1
    } else {
        current.checked_sub(1)?
    };
    if index >= state.frames.len() {
        return None;
    }
    let (cur, other) = (&state.frames[current], &state.frames[index]);
    let linked = if forward {
        other.previous_timestamp == cur.timestamp
    } else {
        cur.previous_timestamp == other.timestamp
    };
    Some((index, linked))
}
