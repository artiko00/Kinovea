//! Blocking behavior of `PreBuffer::add` across threads: a producer waits on a full buffer,
//! released by an acquire, by an interrupt, or never (forced adds).

use kinovea_video::container::CacheAddResult;
use kinovea_video::frame::VideoFrame;
use kinovea_video::prebuffer::{PreBuffer, PreBufferConfig};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

const STILL_BLOCKED: Duration = Duration::from_millis(200);
const WITHIN: Duration = Duration::from_secs(1);

fn frame(ts: i64) -> VideoFrame {
    VideoFrame::new(None, ts, ts - 1)
}

/// Frames 0..=3 with capacity 4 and retention 1.
fn full_buffer() -> Arc<PreBuffer> {
    let config = PreBufferConfig {
        capacity: 4,
        retention: 1,
        ..PreBufferConfig::default()
    };
    let pb = Arc::new(PreBuffer::new(config));
    for ts in 0..4 {
        assert_eq!(pb.add(frame(ts)), CacheAddResult::Added);
    }
    pb
}

/// Runs `add` on a worker thread and returns the channel that receives its result.
fn spawn_add(pb: &Arc<PreBuffer>, ts: i64) -> mpsc::Receiver<CacheAddResult> {
    let (tx, rx) = mpsc::channel();
    let pb = Arc::clone(pb);
    thread::spawn(move || {
        tx.send(pb.add(frame(ts))).expect("receiver alive");
    });
    rx
}

#[test]
fn add_waits_on_a_full_buffer_until_an_acquire_frees_room() {
    let pb = full_buffer();
    let rx = spawn_add(&pb, 4);
    assert!(
        rx.recv_timeout(STILL_BLOCKED).is_err(),
        "add must wait while full"
    );

    pb.acquire_closest(3);
    assert_eq!(rx.recv_timeout(WITHIN), Ok(CacheAddResult::Added));
    assert_eq!(pb.timestamps(), vec![2, 3, 4]);
}

/// `Purge` and `ForceAdd` never pulse (`PreBuffer2.cs:530-558`, `:299-319`), so a producer blocked
/// in `Monitor.Wait` stays asleep until `Add`, `AcquireClosest`, `AcquireNext`, `InterruptAdd` or
/// `Shutdown` pulses.
#[test]
fn purge_and_force_add_do_not_wake_a_blocked_add() {
    let pb = full_buffer();
    pb.acquire_closest(0);
    let rx = spawn_add(&pb, 4);
    assert!(
        rx.recv_timeout(STILL_BLOCKED).is_err(),
        "add must wait while full"
    );

    pb.purge();
    assert!(
        rx.recv_timeout(STILL_BLOCKED).is_err(),
        "purge must not wake the add"
    );

    pb.force_add(frame(1));
    assert!(
        rx.recv_timeout(STILL_BLOCKED).is_err(),
        "force_add must not wake the add"
    );

    pb.acquire_closest(1);
    assert_eq!(rx.recv_timeout(WITHIN), Ok(CacheAddResult::Added));
}

#[test]
fn interrupt_releases_a_blocked_add_without_storing_its_frame() {
    let pb = full_buffer();
    let rx = spawn_add(&pb, 4);
    assert!(
        rx.recv_timeout(STILL_BLOCKED).is_err(),
        "add must wait while full"
    );

    pb.interrupt_add();
    assert_eq!(rx.recv_timeout(WITHIN), Ok(CacheAddResult::Interrupted));
    assert!(!pb.timestamps().contains(&4));
}

#[test]
fn forced_add_on_a_full_buffer_never_blocks() {
    let pb = full_buffer();
    let (tx, rx) = mpsc::channel();
    let worker = Arc::clone(&pb);
    thread::spawn(move || {
        tx.send(worker.force_add(frame(4))).expect("receiver alive");
    });
    assert_eq!(rx.recv_timeout(WITHIN), Ok(CacheAddResult::Added));
    assert!(pb.timestamps().contains(&4));
}
