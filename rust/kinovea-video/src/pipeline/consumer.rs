//! Port of `Consumers/AbstractConsumer.cs`. The `ConsumerState` the producer barrier reads lives
//! in `consumer_state.rs`, which keeps the ring and the runner apart (no `Arc` cycle). The
//! `AutoResetEvent` becomes a flag plus `Condvar`, so an early activation is kept.
//! Cache-line padding and the `BeforeActivate`/`Initialize` hooks are not ported.

use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::{Arc, Condvar, Mutex};

use super::consumer_state::ConsumerState;
use super::producer::Frame;
use super::ring_buffer::RingBuffer;

type Processor = Box<dyn FnMut(i64, &Frame) + Send>;

/// Batch consumer: processes every committed slot in order while active.
pub struct BatchConsumer {
    state: Arc<ConsumerState>,
    ring: Mutex<Option<Arc<RingBuffer>>>,
    process: Mutex<Processor>,
    signal: Mutex<bool>,
    wake: Condvar,
    deactivate_asked: AtomicBool,
    stop_asked: AtomicBool,
}

impl BatchConsumer {
    pub fn new(process: impl FnMut(i64, &Frame) + Send + 'static) -> Self {
        Self {
            state: Arc::new(ConsumerState::new(false, -1)),
            ring: Mutex::new(None),
            process: Mutex::new(Box::new(process)),
            signal: Mutex::new(false),
            wake: Condvar::new(),
            deactivate_asked: AtomicBool::new(false),
            stop_asked: AtomicBool::new(false),
        }
    }

    pub fn state(&self) -> Arc<ConsumerState> {
        Arc::clone(&self.state)
    }

    pub fn set_ring_buffer(&self, ring: Arc<RingBuffer>) {
        *self.ring.lock().expect("ring lock") = Some(ring);
    }

    pub fn clear_ring_buffer(&self) {
        *self.ring.lock().expect("ring lock") = None;
    }

    /// Requests a batch run. The request is kept if the runner is not waiting yet.
    pub fn activate(&self) {
        *self.signal.lock().expect("signal lock") = true;
        self.wake.notify_one();
    }

    pub fn deactivate(&self) {
        self.deactivate_asked.store(true, SeqCst);
    }

    /// Ends `run`. Does nothing if the runner has not started.
    pub fn stop(&self) {
        if !self.started() {
            return;
        }
        self.stop_asked.store(true, SeqCst);
        let _signal = self.signal.lock().expect("signal lock");
        self.wake.notify_all();
    }

    /// Blocks the calling thread until `stop`. Activations are processed in turn.
    pub fn run(&self) {
        self.state.started.store(true, SeqCst);
        while !self.stop_asked.load(SeqCst) {
            self.wait_for_activation();
            if self.stop_asked.load(SeqCst) {
                break;
            }
            self.consume();
        }
        self.state.started.store(false, SeqCst);
    }

    fn wait_for_activation(&self) {
        let mut requested = self.signal.lock().expect("signal lock");
        while !*requested && !self.stop_asked.load(SeqCst) {
            requested = self.wake.wait(requested).expect("signal lock");
        }
        *requested = false;
    }

    fn consume(&self) {
        let Some(ring) = self.ring.lock().expect("ring lock").clone() else {
            return;
        };
        // Start at the frame committed at activation, not at the oldest one still in the ring.
        let start = (ring.producer_position() - 1).max(-1);
        self.state.position.store(start, SeqCst);
        self.state.active.store(true, SeqCst);

        let mut next = start + 1;
        while !self.interrupted() {
            let readable = ring.wait_for(next, || self.interrupted());
            let mut process = self.process.lock().expect("process lock");
            while next <= readable {
                let slot = ring.entry(next);
                (*process)(next, &slot);
                next += 1;
            }
            self.state.position.store(next - 1, SeqCst);
        }
        self.state.active.store(false, SeqCst);
        self.deactivate_asked.store(false, SeqCst);
    }

    fn interrupted(&self) -> bool {
        self.deactivate_asked.load(SeqCst) || self.stop_asked.load(SeqCst)
    }
}

/// Exposes `started`, `active` and `position` of the shared state.
impl Deref for BatchConsumer {
    type Target = ConsumerState;
    fn deref(&self) -> &ConsumerState {
        &self.state
    }
}
