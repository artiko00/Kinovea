use std::time::{Duration, Instant};

use super::frequency::FrequencyCounter;

const STEP_50_HZ: Duration = Duration::from_millis(20);

fn feed(counter: &mut FrequencyCounter, from: Instant, frames: u32, step: Duration) {
    for k in 0..frames {
        counter.tick_at(from + step * k);
    }
}

#[test]
fn steady_50_hz_is_measured_within_5_percent() {
    let mut counter = FrequencyCounter::new(24, 0);
    feed(&mut counter, Instant::now(), 48, STEP_50_HZ);
    assert!((counter.frequency() - 50.0).abs() <= 2.5);
}

#[test]
fn ignored_intervals_delay_the_first_publication() {
    let base = Instant::now();
    let mut counter = FrequencyCounter::new(24, 48);
    feed(&mut counter, base, 48, STEP_50_HZ);
    assert_eq!(counter.frequency(), 0.0);
    feed(&mut counter, base, 73, STEP_50_HZ);
    assert!((counter.frequency() - 50.0).abs() <= 2.5);
}

#[test]
fn reset_forgets_the_gap_before_the_next_first_tick() {
    let base = Instant::now();
    let mut counter = FrequencyCounter::new(24, 0);
    feed(&mut counter, base, 25, STEP_50_HZ);
    counter.reset();
    assert_eq!(counter.frequency(), 0.0);

    let restart = base + Duration::from_secs(1);
    counter.tick_at(restart);
    assert_eq!(counter.frequency(), 0.0);
    for k in 1..=24 {
        counter.tick_at(restart + STEP_50_HZ * k);
    }
    assert!((counter.frequency() - 50.0).abs() <= 2.5);
}

#[test]
fn moving_average_smooths_a_rate_change() {
    let base = Instant::now();
    let mut counter = FrequencyCounter::new(24, 0);
    feed(&mut counter, base, 25, STEP_50_HZ);
    let last = base + STEP_50_HZ * 24;
    for k in 1..=24 {
        counter.tick_at(last + Duration::from_millis(10) * k);
    }
    // EMA alpha 0.1: 0.01 * 0.1 + 0.02 * 0.9 = 0.019 seconds per interval.
    assert!((counter.frequency() - 1.0 / 0.019).abs() < 1e-6);
}
