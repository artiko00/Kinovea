//! Idempotence of the time-based drawings.

use super::rounds::{check, drawing_round};
use super::timed::*;

#[test]
fn chrono_is_idempotent() {
    check(chrono, &|x| drawing_round("Chrono", x));
}

#[test]
fn chrono_multi_is_idempotent() {
    check(chrono_multi, &|x| drawing_round("ChronoMulti", x));
}

#[test]
fn counter_is_idempotent() {
    check(counter, &|x| drawing_round("Counter", x));
}

#[test]
fn track_is_idempotent() {
    check(track, &|x| drawing_round("Track", x));
}
