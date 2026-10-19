//! Port of `PreBuffer2.Purge`, `Shutdown` and `Clear`.

use super::PreBuffer;

impl PreBuffer {
    /// `Purge`: removes every frame except current. Publishes only when something was removed.
    pub fn purge(&self) {
        let mut state = self.lock();
        let current = state.current.as_ref().map(|c| c.timestamp);
        let before = state.frames.len();
        state.frames.retain(|f| Some(f.timestamp) == current);
        if state.frames.len() != before {
            self.publish(&mut state);
        }
    }

    /// `Shutdown`: removes every frame including current, and closes the buffer to adds
    /// until `reset_interrupt_add`.
    pub fn shutdown(&self) {
        let mut state = self.lock();
        state.frames.clear();
        state.current = None;
        state.interrupt_add = true;
        self.publish(&mut state);
        self.wake_adds();
    }

    /// Removes every frame including current. The C# `PreBuffer2.Clear` throws
    /// `NotImplementedException` (`PreBuffer2.cs:522-525`) and leaves the buffer untouched. As with
    /// the dense cache's `shutdown` (design D3), the Rust port clears instead. The replay adapter
    /// reports the C# exception without calling this.
    pub fn clear(&self) {
        let mut state = self.lock();
        state.frames.clear();
        state.current = None;
        self.publish(&mut state);
    }
}
