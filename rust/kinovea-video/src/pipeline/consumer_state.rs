//! Port of `Kinovea.Pipeline/Interfaces/IFrameConsumer.cs`: the started, active and position
//! flags that the producer barrier reads. Written only by the batch consumer, in the same pipeline module.

use std::sync::atomic::{AtomicBool, AtomicI64, Ordering::SeqCst};

/// Started, active and position flags that the producer barrier reads.
#[derive(Debug)]
pub struct ConsumerState {
    pub(super) started: AtomicBool,
    pub(super) active: AtomicBool,
    pub(super) position: AtomicI64,
}

impl ConsumerState {
    pub fn new(active: bool, position: i64) -> Self {
        Self {
            started: AtomicBool::new(false),
            active: AtomicBool::new(active),
            position: AtomicI64::new(position),
        }
    }

    pub fn started(&self) -> bool {
        self.started.load(SeqCst)
    }

    pub fn active(&self) -> bool {
        self.active.load(SeqCst)
    }

    pub fn position(&self) -> i64 {
        self.position.load(SeqCst)
    }
}
