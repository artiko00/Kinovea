//! Scripted producer against a gated slow consumer and a fast consumer (task 6.4).
//!
//! The slow consumer blocks inside its first frame until the test releases it, so its
//! position stays at -1 and every claim beyond the capacity is refused. The drop count
//! is therefore exact: no timing decides it.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use kinovea_video::pipeline::{BatchConsumer, FrameHandler, FramePipeline, FrameProducer};

const CAPACITY: usize = 8;
const FRAMES: usize = 200;
const STEP: Duration = Duration::from_millis(20);
const TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Default)]
struct ScriptedProducer {
    handler: Option<FrameHandler>,
}

impl FrameProducer for ScriptedProducer {
    fn set_frame_handler(&mut self, handler: Option<FrameHandler>) {
        self.handler = handler;
    }
}

impl ScriptedProducer {
    fn produce(&mut self, bytes: &[u8], at: Instant) {
        let handler = self.handler.as_mut().expect("handler installed");
        handler(bytes, bytes.len(), at);
    }
}

fn wait_until(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + TIMEOUT;
    while !condition() {
        assert!(Instant::now() < deadline, "condition not reached in time");
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn drops_are_exact_and_both_consumers_process_the_accepted_frames_in_order() {
    let fast_log = Arc::new(Mutex::new(Vec::new()));
    let fast_sink = Arc::clone(&fast_log);
    let fast = Arc::new(BatchConsumer::new(move |position, _frame| {
        fast_sink.lock().expect("log").push(position);
    }));

    let (gate_tx, gate_rx) = mpsc::channel::<()>();
    let slow_log = Arc::new(Mutex::new(Vec::new()));
    let slow_sink = Arc::clone(&slow_log);
    let mut gate = Some(gate_rx);
    let slow = Arc::new(BatchConsumer::new(move |position, _frame| {
        if let Some(rx) = gate.take() {
            rx.recv().expect("gate released");
        }
        slow_sink.lock().expect("log").push(position);
    }));

    let mut producer = ScriptedProducer::default();
    let pipeline = FramePipeline::new(
        &mut producer,
        vec![Arc::clone(&fast), Arc::clone(&slow)],
        CAPACITY,
        16,
    )
    .expect("pipeline");

    fast.activate();
    slow.activate();
    wait_until(|| fast.active() && slow.active());

    let base = Instant::now();
    for k in 0..FRAMES {
        producer.produce(&[k as u8], base + STEP * (k as u32));
    }
    assert_eq!(pipeline.drops(), (FRAMES - CAPACITY) as u64);

    gate_tx.send(()).expect("release gate");
    let last = CAPACITY as i64 - 1;
    wait_until(|| fast.position() == last && slow.position() == last);

    let accepted: Vec<i64> = (0..=last).collect();
    assert_eq!(*fast_log.lock().expect("log"), accepted);
    assert_eq!(*slow_log.lock().expect("log"), accepted);

    let hz = pipeline.frequency();
    assert!((hz - 50.0).abs() <= 2.5, "measured {hz} Hz");

    pipeline.teardown(&mut producer);
    assert!(!fast.started());
    assert!(!slow.started());
}
