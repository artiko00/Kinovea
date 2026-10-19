//! Port of `Kinovea.Pipeline/RingBuffer.cs`. The producer and consumer positions are atomics.
//! Each slot payload sits behind its own `Mutex`; the barrier keeps it uncontended, so the
//! port needs no `unsafe` (design D5). `wait_for` also returns when `interrupted()` is true,
//! because the C# barrier cannot be cancelled and would block `stop` forever. Benchmark modes
//! and `Teardown` are not ported.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use super::consumer_state::ConsumerState;
use super::producer::Frame;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RingBufferError {
    NotPowerOfTwo(usize),
    Allocation,
}

impl std::fmt::Display for RingBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPowerOfTwo(capacity) => {
                write!(
                    f,
                    "ring buffer capacity must be a power of two, got {capacity}"
                )
            }
            Self::Allocation => write!(f, "ring buffer slots could not be allocated"),
        }
    }
}

impl std::error::Error for RingBufferError {}

pub struct RingBuffer {
    slots: Vec<Mutex<Frame>>,
    mask: i64,
    frame_length: usize,
    producer_position: AtomicI64,
    consumers: Vec<Arc<ConsumerState>>,
}

impl RingBuffer {
    /// Allocates `capacity` slots of `buffer_size` bytes. Capacity must be a power of two.
    pub fn new(
        capacity: usize,
        buffer_size: usize,
        consumers: Vec<Arc<ConsumerState>>,
    ) -> Result<Self, RingBufferError> {
        if !capacity.is_power_of_two() {
            return Err(RingBufferError::NotPowerOfTwo(capacity));
        }
        let slots = (0..capacity)
            .map(|_| Frame::try_new(buffer_size).map(Mutex::new))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| RingBufferError::Allocation)?;
        Ok(Self {
            slots,
            mask: capacity as i64 - 1,
            frame_length: buffer_size,
            producer_position: AtomicI64::new(-1),
            consumers,
        })
    }

    pub fn frame_length(&self) -> usize {
        self.frame_length
    }

    /// Last position committed by the producer, -1 before the first commit.
    pub fn producer_position(&self) -> i64 {
        self.producer_position.load(Ordering::Acquire)
    }

    /// Producer side. Returns the next slot only when every active consumer has read the slot
    /// it overwrites; `None` is a refused claim (a dropped frame for the caller to count).
    pub fn try_claim(&self) -> Option<MutexGuard<'_, Frame>> {
        let next = self.producer_position() + 1;
        if !self.is_writeable(next) {
            return None;
        }
        Some(self.entry(next))
    }

    /// Producer side. Publishes the claimed slot to consumers.
    pub fn commit(&self) {
        self.producer_position
            .store(self.producer_position() + 1, Ordering::Release);
    }

    /// Returns the slot for `position` without barrier. Positions wrap onto the slots.
    pub fn entry(&self, position: i64) -> MutexGuard<'_, Frame> {
        self.slots[(position & self.mask) as usize]
            .lock()
            .expect("slot lock")
    }

    /// Consumer side. Waits until the producer reaches `position` or `interrupted()` is true,
    /// then returns the current producer position so a slow consumer can drain a batch.
    pub fn wait_for(&self, position: i64, interrupted: impl Fn() -> bool) -> i64 {
        while position > self.producer_position() && !interrupted() {
            thread::yield_now();
        }
        self.producer_position()
    }

    fn is_writeable(&self, position: i64) -> bool {
        let must_have_read = position - self.slots.len() as i64;
        self.consumers
            .iter()
            .all(|c| !c.active() || c.position() >= must_have_read)
    }
}
