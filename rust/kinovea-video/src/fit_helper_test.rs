use crate::fit_helper::{
    fit, fit_even_multiplier, minimum_multiplier, nearest_even_multiplier, reference_multiplier,
};
use kinovea_geometry::Size;

#[test]
fn downscale_keeps_the_ratio_inside_the_bounds() {
    assert_eq!(
        fit(Size::new(1920, 1080), Size::new(960, 960), false),
        Size::new(960, 540)
    );
}

#[test]
fn upscale_is_only_allowed_when_requested() {
    assert_eq!(
        fit(Size::new(100, 50), Size::new(400, 400), false),
        Size::new(100, 50)
    );
    assert_eq!(
        fit(Size::new(100, 50), Size::new(400, 400), true),
        Size::new(400, 200)
    );
}

#[test]
fn empty_bounds_return_the_source() {
    assert_eq!(
        fit(Size::new(320, 240), Size::new(0, 100), true),
        Size::new(320, 240)
    );
    assert_eq!(
        fit(Size::new(320, 240), Size::new(100, -1), true),
        Size::new(320, 240)
    );
}

#[test]
fn halves_round_to_even_like_math_round() {
    assert_eq!(
        fit(Size::new(5, 10), Size::new(10, 5), true),
        Size::new(2, 5)
    );
    assert_eq!(
        fit(Size::new(7, 10), Size::new(10, 5), true),
        Size::new(4, 5)
    );
}

#[test]
fn each_dimension_is_at_least_one_pixel() {
    assert_eq!(
        fit(Size::new(1000, 1), Size::new(10, 10), false),
        Size::new(10, 1)
    );
}

#[test]
fn reference_multiplier_reduces_the_ratio() {
    assert_eq!(reference_multiplier(Size::new(1920, 1080)), (16, 9, 120));
}

#[test]
fn fit_even_multiplier_is_even_and_fits_the_bounds() {
    assert_eq!(fit_even_multiplier(Size::new(1000, 600), 16, 9), 62);
    assert_eq!(fit_even_multiplier(Size::new(1000, 500), 16, 9), 54);
}

#[test]
fn fit_even_multiplier_is_zero_for_empty_bounds() {
    assert_eq!(fit_even_multiplier(Size::new(0, 500), 16, 9), 0);
}

#[test]
fn minimum_multiplier_is_the_smallest_even_size_covering_the_minimum() {
    assert_eq!(minimum_multiplier(16, 9, 100), 12);
    assert_eq!(minimum_multiplier(1, 1, 5), 6);
}

#[test]
fn nearest_even_multiplier_rounds_halves_away_from_zero_and_is_at_least_two() {
    assert_eq!(nearest_even_multiplier(5.0), 6);
    assert_eq!(nearest_even_multiplier(7.0), 8);
    assert_eq!(nearest_even_multiplier(1.0), 2);
}
