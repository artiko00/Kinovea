//! Idempotence of the containers and singletons.

use super::containers::*;
use super::rounds::{
    check, coordinate_round, keyframe_round, numbers_round, spotlights_round, test_grid_round,
};

#[test]
fn keyframe_is_idempotent() {
    check(keyframe, &keyframe_round);
}

#[test]
fn spotlights_are_idempotent() {
    check(spotlights, &spotlights_round);
}

#[test]
fn number_sequence_is_idempotent() {
    check(number_sequence, &numbers_round);
}

#[test]
fn coordinate_system_is_idempotent() {
    check(coordinate_system, &coordinate_round);
}

#[test]
fn test_grid_is_idempotent() {
    check(test_grid, &test_grid_round);
}
