use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use super::consumer::BatchConsumer;
use super::ring_buffer::RingBuffer;

const TIMEOUT: Duration = Duration::from_secs(5);

type Log = Arc<Mutex<Vec<(i64, u8)>>>;

fn wait_until(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + TIMEOUT;
    while !condition() {
        assert!(Instant::now() < deadline, "condition not reached in time");
        thread::sleep(Duration::from_millis(1));
    }
}

fn recording_consumer() -> (Arc<BatchConsumer>, Log) {
    let log: Log = Arc::default();
    let sink = Arc::clone(&log);
    let consumer = BatchConsumer::new(move |position, frame| {
        sink.lock()
            .expect("log")
            .push((position, frame.payload()[0]));
    });
    (Arc::new(consumer), log)
}

fn bind(consumer: &BatchConsumer, capacity: usize) -> Arc<RingBuffer> {
    let ring = Arc::new(RingBuffer::new(capacity, 8, vec![consumer.state()]).expect("ring"));
    consumer.set_ring_buffer(Arc::clone(&ring));
    ring
}

fn commit_tag(ring: &RingBuffer, tag: u8) {
    let mut slot = ring.try_claim().expect("claim");
    slot.fill(&[tag], 1);
    drop(slot);
    ring.commit();
}

fn spawn_run(consumer: &Arc<BatchConsumer>) -> JoinHandle<()> {
    let runner = Arc::clone(consumer);
    thread::spawn(move || runner.run())
}

fn finish(consumer: &BatchConsumer, handle: JoinHandle<()>) {
    consumer.stop();
    handle.join().expect("consumer thread");
}

#[test]
fn a_new_consumer_is_idle_before_its_position() {
    let (consumer, _) = recording_consumer();
    assert!(!consumer.started());
    assert!(!consumer.active());
    assert_eq!(consumer.position(), -1);
}

#[test]
fn run_starts_and_stop_ends_an_idle_consumer() {
    let (consumer, _) = recording_consumer();
    let (done_tx, done_rx) = mpsc::channel();
    let runner = Arc::clone(&consumer);
    thread::spawn(move || {
        runner.run();
        done_tx.send(()).expect("send");
    });
    wait_until(|| consumer.started());
    consumer.stop();
    done_rx.recv_timeout(TIMEOUT).expect("run returned");
    assert!(!consumer.started());
}

#[test]
fn stop_interrupts_a_wait_for_a_position_never_produced() {
    let (consumer, _) = recording_consumer();
    let _ring = bind(&consumer, 4);
    let handle = spawn_run(&consumer);
    consumer.activate();
    wait_until(|| consumer.active());
    finish(&consumer, handle);
    assert!(!consumer.active());
}

#[test]
fn activation_starts_at_the_current_producer_position() {
    let (consumer, log) = recording_consumer();
    let ring = bind(&consumer, 4);
    commit_tag(&ring, 0);
    commit_tag(&ring, 1);
    let handle = spawn_run(&consumer);
    consumer.activate();
    wait_until(|| consumer.position() == 1);
    commit_tag(&ring, 2);
    wait_until(|| log.lock().expect("log").len() == 2);
    finish(&consumer, handle);
    assert_eq!(*log.lock().expect("log"), vec![(1, 1), (2, 2)]);
}

#[test]
fn reactivation_skips_slots_written_while_idle() {
    let (consumer, log) = recording_consumer();
    let ring = bind(&consumer, 4);
    let handle = spawn_run(&consumer);
    consumer.activate();
    wait_until(|| consumer.active());
    commit_tag(&ring, 0);
    commit_tag(&ring, 1);
    wait_until(|| consumer.position() == 1);

    consumer.deactivate();
    wait_until(|| !consumer.active());
    commit_tag(&ring, 2);
    commit_tag(&ring, 3);

    consumer.activate();
    wait_until(|| consumer.active());
    commit_tag(&ring, 4);
    wait_until(|| log.lock().expect("log").len() == 4);
    finish(&consumer, handle);
    assert_eq!(
        *log.lock().expect("log"),
        vec![(0, 0), (1, 1), (3, 3), (4, 4)]
    );
}
