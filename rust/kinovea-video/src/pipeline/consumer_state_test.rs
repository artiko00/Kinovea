use std::sync::atomic::Ordering::SeqCst;

use super::consumer_state::ConsumerState;

#[test]
fn a_new_state_is_not_started_and_keeps_the_given_flags() {
    let state = ConsumerState::new(true, 7);
    assert!(!state.started());
    assert!(state.active());
    assert_eq!(state.position(), 7);
}

#[test]
fn values_written_to_the_flags_are_read_back() {
    let state = ConsumerState::new(false, -1);
    state.started.store(true, SeqCst);
    state.active.store(true, SeqCst);
    state.position.store(3, SeqCst);
    assert!(state.started());
    assert!(state.active());
    assert_eq!(state.position(), 3);
}
