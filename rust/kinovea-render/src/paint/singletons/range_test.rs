//! `RangeHelper.FindUsableStepSize` on ranges whose step is worked out by hand: the minimum step
//! `range / targetSteps` gives the order of magnitude (10^floor(log10 min)) and the residual picks
//! 1, 2, 5 or 10 times that order.

use super::range::find_usable_step_size;

#[test]
fn residual_above_five_takes_ten_times_the_order() {
    // 100 / 15 = 6.67: order 1, residual 6.67 > 5 -> 10.
    assert_eq!(find_usable_step_size(100.0, 15.0), 10.0);
}

#[test]
fn residual_exactly_one_keeps_the_order() {
    // 100 / 10 = 10: order 10, residual 1 is not above 1 -> 10.
    assert_eq!(find_usable_step_size(100.0, 10.0), 10.0);
}

#[test]
fn residual_above_one_takes_twice_the_order() {
    // 30 / 15 = 2: order 1, residual 2 is not above 2, above 1 -> 2.
    assert_eq!(find_usable_step_size(30.0, 15.0), 2.0);
}

#[test]
fn residual_above_two_takes_five_times_the_order() {
    // 45 / 15 = 3: order 1, residual 3 > 2 -> 5.
    assert_eq!(find_usable_step_size(45.0, 15.0), 5.0);
}

#[test]
fn large_ranges_scale_the_order_of_magnitude() {
    // 240 / 15 = 16: order 10, residual 1.6 > 1 -> 20.
    assert_eq!(find_usable_step_size(240.0, 15.0), 20.0);
}
