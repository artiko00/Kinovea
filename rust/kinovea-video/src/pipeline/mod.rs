//! Port of `Kinovea.Pipeline/FramePipeline.cs`: binds one producer to batch consumers through a
//! ring buffer, counts refused claims as drops and publishes the measured frequency.
//! Consumer threads are owned here (C# left them to the caller) so `teardown` can join them.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Instant;

pub mod consumer;
pub mod consumer_state;
pub mod frequency;
pub mod producer;
pub mod ring_buffer;

#[cfg(test)]
mod consumer_state_test;
#[cfg(test)]
mod consumer_test;
#[cfg(test)]
mod frequency_test;
#[cfg(test)]
mod mod_test;
#[cfg(test)]
mod producer_test;
#[cfg(test)]
mod ring_buffer_test;

pub use consumer::BatchConsumer;
pub use consumer_state::ConsumerState;
pub use frequency::FrequencyCounter;
pub use producer::{Frame, FrameHandler, FrameProducer, ImageDescriptor, ImageFormat};
pub use ring_buffer::{RingBuffer, RingBufferError};

/// State shared between the producer handler and the pipeline owner.
struct Shared {
    ring: Arc<RingBuffer>,
    drops: AtomicU64,
    frequency: AtomicU64,
}

pub struct FramePipeline {
    shared: Arc<Shared>,
    consumers: Vec<Arc<BatchConsumer>>,
    threads: Vec<JoinHandle<()>>,
}

impl FramePipeline {
    /// Binds `producer` and starts one thread per consumer. Returns once every consumer runs.
    pub fn new(
        producer: &mut dyn FrameProducer,
        consumers: Vec<Arc<BatchConsumer>>,
        capacity: usize,
        buffer_size: usize,
    ) -> Result<Self, RingBufferError> {
        let states = consumers.iter().map(|c| c.state()).collect();
        let ring = Arc::new(RingBuffer::new(capacity, buffer_size, states)?);

        let mut threads = Vec::new();
        for consumer in &consumers {
            consumer.set_ring_buffer(Arc::clone(&ring));
            let runner = Arc::clone(consumer);
            threads.push(thread::spawn(move || runner.run()));
        }
        for consumer in &consumers {
            while !consumer.started() {
                thread::yield_now();
            }
        }

        let shared = Arc::new(Shared {
            ring,
            drops: AtomicU64::new(0),
            frequency: AtomicU64::new(0f64.to_bits()),
        });
        let handler_shared = Arc::clone(&shared);
        let mut counter = FrequencyCounter::new(24, 48);
        producer.set_frame_handler(Some(Box::new(
            move |bytes: &[u8], length: usize, at: Instant| {
                counter.tick_at(at);
                handler_shared
                    .frequency
                    .store(counter.frequency().to_bits(), Ordering::Release);
                if !write_slot(&handler_shared.ring, bytes, length) {
                    handler_shared.drops.fetch_add(1, Ordering::Relaxed);
                }
            },
        )));

        Ok(Self {
            shared,
            consumers,
            threads,
        })
    }

    /// Frames refused because a consumer still held the slot they would overwrite.
    pub fn drops(&self) -> u64 {
        self.shared.drops.load(Ordering::Acquire)
    }

    /// Measured producer frequency in hertz.
    pub fn frequency(&self) -> f64 {
        f64::from_bits(self.shared.frequency.load(Ordering::Acquire))
    }

    /// Removes the producer handler, stops and joins the consumer threads, and unbinds them.
    /// A consumer blocked inside its processor keeps this call blocked.
    pub fn teardown(self, producer: &mut dyn FrameProducer) {
        producer.set_frame_handler(None);
        for consumer in &self.consumers {
            consumer.stop();
        }
        for thread in self.threads {
            thread.join().expect("consumer thread");
        }
        for consumer in &self.consumers {
            consumer.clear_ring_buffer();
        }
    }
}

/// Returns `false` when the claim is refused. An oversized payload is still committed, as in C#.
fn write_slot(ring: &RingBuffer, bytes: &[u8], payload_length: usize) -> bool {
    let Some(mut slot) = ring.try_claim() else {
        return false;
    };
    slot.fill(bytes, payload_length);
    drop(slot);
    ring.commit();
    true
}
