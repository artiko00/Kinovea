//! `TickMarkVisible`: up to the fifth line every graduation is kept, then only perfect squares.

use super::grid_lines::tick_mark_visible;

#[test]
fn first_five_lines_keep_their_graduations() {
    for step in [-7, 0, 1, 2, 3, 4, 5] {
        assert!(tick_mark_visible(step), "step {step}");
    }
}

#[test]
fn beyond_five_only_perfect_squares_keep_their_graduations() {
    assert!(!tick_mark_visible(6));
    assert!(!tick_mark_visible(10));
    assert!(tick_mark_visible(9));
    assert!(tick_mark_visible(16));
    assert!(tick_mark_visible(25));
}
