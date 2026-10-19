//! Port of `Kinovea.Video/FitHelper.cs`. Pure functions; `Math.Round` is reproduced with
//! `round_ties_even` for `Fit` and `round` (away from zero) for `GetNearestEvenMultiplier`.

use kinovea_geometry::Size;

/// `FitHelper.Fit`: scales `source` to fit `bounds` keeping its ratio. Non-positive bounds
/// return `source`. Without `allow_upscale` the scale never exceeds 1. Each side is at least
/// 1 pixel.
pub fn fit(source: Size, bounds: Size, allow_upscale: bool) -> Size {
    if bounds.width <= 0 || bounds.height <= 0 {
        return source;
    }
    let scale_x = bounds.width as f64 / source.width as f64;
    let scale_y = bounds.height as f64 / source.height as f64;
    let mut scale = scale_x.min(scale_y);
    if !allow_upscale {
        scale = scale.min(1.0);
    }
    let width = (source.width as f64 * scale).round_ties_even() as i32;
    let height = (source.height as f64 * scale).round_ties_even() as i32;
    Size::new(width.max(1), height.max(1))
}

/// `GetReferenceMultiplier`: reduces `reference` to `(ratio_width, ratio_height)` and returns
/// the multiplier, e.g. 1920x1080 gives `(16, 9, 120)`.
pub fn reference_multiplier(reference: Size) -> (i32, i32, i32) {
    let gcd = gcd(reference.width, reference.height);
    (reference.width / gcd, reference.height / gcd, gcd)
}

/// `GetFitEvenMultiplier`: largest even multiplier that fits `ratio` into `bounds`, so both
/// final dimensions are even. Zero for non-positive bounds.
pub fn fit_even_multiplier(bounds: Size, ratio_width: i32, ratio_height: i32) -> i32 {
    if bounds.width <= 0 || bounds.height <= 0 {
        return 0;
    }
    let multiplier = (bounds.width / ratio_width).min(bounds.height / ratio_height);
    multiplier & !1
}

/// `GetMinimumMultiplier`: smallest even multiplier giving a size of at least `min_size` in
/// both dimensions.
pub fn minimum_multiplier(ratio_width: i32, ratio_height: i32, min_size: i32) -> i32 {
    let multiplier = ((min_size + ratio_width - 1) / ratio_width)
        .max((min_size + ratio_height - 1) / ratio_height);
    (multiplier + 1) & !1
}

/// `GetNearestEvenMultiplier`: `multiplier / 2` rounded away from zero, doubled, at least 2.
pub fn nearest_even_multiplier(multiplier: f64) -> i32 {
    ((multiplier / 2.0).round() as i32 * 2).max(2)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
