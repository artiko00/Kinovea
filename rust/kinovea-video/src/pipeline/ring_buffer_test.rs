use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use super::consumer_state::ConsumerState;
use super::ring_buffer::{RingBuffer, RingBufferError};

fn consumer(active: bool, position: i64) -> Arc<ConsumerState> {
    Arc::new(ConsumerState::new(active, position))
}

fn commit_one(ring: &RingBuffer, tag: u8) -> bool {
    let Some(mut slot) = ring.try_claim() else {
        return false;
    };
    slot.fill(&[tag], 1);
    drop(slot);
    ring.commit();
    true
}

#[test]
fn rejects_a_capacity_that_is_not_a_power_of_two() {
    let error = RingBuffer::new(6, 4, vec![]).err().expect("error");
    assert_eq!(error, RingBufferError::NotPowerOfTwo(6));
    assert!(error.to_string().contains("power of two"));
}

#[test]
fn rejects_zero_capacity() {
    assert_eq!(
        RingBuffer::new(0, 4, vec![]).err(),
        Some(RingBufferError::NotPowerOfTwo(0))
    );
}

#[test]
fn reports_an_allocation_failure() {
    assert_eq!(
        RingBuffer::new(2, usize::MAX, vec![]).err(),
        Some(RingBufferError::Allocation)
    );
}

#[test]
fn starts_before_the_first_position_with_the_frame_length() {
    let ring = RingBuffer::new(4, 16, vec![]).expect("ring");
    assert_eq!(ring.frame_length(), 16);
    assert_eq!(ring.producer_position(), -1);
}

#[test]
fn refuses_a_claim_when_an_active_consumer_has_not_read_the_slot() {
    let ring = RingBuffer::new(4, 8, vec![consumer(true, -1)]).expect("ring");
    for tag in 0..4 {
        assert!(commit_one(&ring, tag));
    }
    assert!(ring.try_claim().is_none());
}

#[test]
fn accepts_a_claim_once_the_consumer_has_read_the_overwritten_slot() {
    let ring = RingBuffer::new(4, 8, vec![consumer(true, 0)]).expect("ring");
    for tag in 0..4 {
        assert!(commit_one(&ring, tag));
    }
    assert!(ring.try_claim().is_some());
}

#[test]
fn an_inactive_consumer_does_not_block_claims() {
    let ring = RingBuffer::new(4, 8, vec![consumer(false, -1)]).expect("ring");
    for tag in 0..5 {
        assert!(commit_one(&ring, tag));
    }
}

#[test]
fn commit_publishes_the_claimed_payload() {
    let ring = RingBuffer::new(4, 8, vec![]).expect("ring");
    assert!(commit_one(&ring, 7));
    assert_eq!(ring.producer_position(), 0);
    assert_eq!(ring.entry(0).payload(), &[7]);
}

#[test]
fn positions_wrap_onto_the_slots() {
    let ring = RingBuffer::new(4, 8, vec![]).expect("ring");
    for tag in 10..15 {
        assert!(commit_one(&ring, tag));
    }
    assert_eq!(ring.entry(4).payload(), &[14]);
    assert_eq!(ring.entry(0).payload(), &[14]);
}

#[test]
fn wait_for_returns_once_the_producer_reaches_the_position() {
    let ring = Arc::new(RingBuffer::new(4, 8, vec![]).expect("ring"));
    let (tx, rx) = mpsc::channel();
    let reader = Arc::clone(&ring);
    thread::spawn(move || tx.send(reader.wait_for(5, || false)).expect("send"));
    for tag in 0..6 {
        assert!(commit_one(&ring, tag));
    }
    let reached = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("barrier released");
    assert_eq!(reached, 5);
}

#[test]
fn wait_for_returns_immediately_when_interrupted() {
    let ring = RingBuffer::new(4, 8, vec![]).expect("ring");
    assert_eq!(ring.wait_for(10, || true), -1);
}
