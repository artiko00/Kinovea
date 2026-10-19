//! Unmasked `TM_CCOEFF_NORMED`: window means and energies come from integral images, the cross
//! term is direct. Mirrors `common_matchTemplate` of OpenCV 4.x.

use crate::ncc::Plane;
use crate::ncc_integral::Integral;
use crate::ncc_map::Map;

pub(crate) fn match_unmasked<T: Copy + Into<f64>>(image: Plane<T>, template: Plane<T>) -> Map {
    let (width, height) = (
        image.width - template.width + 1,
        image.height - template.height + 1,
    );
    let n = (template.width * template.height) as f64;
    let means = channel_means(template, n);
    let ssd: f64 = template_deviations(template, &means).sum();
    // OpenCV: a template whose variance is below DBL_EPSILON makes the whole map 1.
    if ssd / n < f64::EPSILON {
        return Map {
            width,
            height,
            values: vec![1.0; width * height],
        };
    }
    let template_norm = ssd.sqrt();
    let integral = Integral::new(image);
    let mut values = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let num =
                correlate(image, template, (x, y)) - mean_term(&integral, &means, template, (x, y));
            let (window_sq, window_mean_sq) =
                window_energies(&integral, &means, template, (x, y), n);
            let diff2 = (window_sq - window_mean_sq).max(0.0);
            let t = if diff2 <= 0.5f64.min(10.0 * f64::from(f32::EPSILON) * window_sq) {
                0.0
            } else {
                diff2.sqrt() * template_norm
            };
            values.push(snap(num, t) as f32);
        }
    }
    Map {
        width,
        height,
        values,
    }
}

/// Per-channel template means.
fn channel_means<T: Copy + Into<f64>>(template: Plane<T>, n: f64) -> Vec<f64> {
    let mut sums = vec![0.0; template.channels];
    for pixel in template.data.chunks_exact(template.channels) {
        for (s, &v) in sums.iter_mut().zip(pixel) {
            *s += v.into();
        }
    }
    sums.iter().map(|s| s / n).collect()
}

/// Squared deviations of every template value from its channel mean.
fn template_deviations<'a, T: Copy + Into<f64>>(
    template: Plane<'a, T>,
    means: &'a [f64],
) -> impl Iterator<Item = f64> + 'a {
    template
        .data
        .chunks_exact(template.channels)
        .flat_map(move |pixel| {
            pixel
                .iter()
                .zip(means)
                .map(|(&v, &m)| (v.into() - m).powi(2))
        })
}

/// `sum_c sum_x T(x, c) I(origin + x, c)` over the template window.
fn correlate<T: Copy + Into<f64>>(
    image: Plane<T>,
    template: Plane<T>,
    (x, y): (usize, usize),
) -> f64 {
    let ch = template.channels;
    let mut total = 0.0;
    for (ty, template_row) in template.data.chunks_exact(template.width * ch).enumerate() {
        let image_row = &image.data[((y + ty) * image.width + x) * ch..];
        total += image_row
            .iter()
            .zip(template_row)
            .map(|(&i, &t)| i.into() * t.into())
            .sum::<f64>();
    }
    total
}

/// `sum_c S_c mean_c` over the window at `origin`, where `S_c` is the window sum of channel `c`.
fn mean_term<T>(
    integral: &Integral,
    means: &[f64],
    template: Plane<T>,
    origin: (usize, usize),
) -> f64 {
    let size = (template.width, template.height);
    means
        .iter()
        .enumerate()
        .map(|(c, &mean)| integral.window(origin, size, c).0 * mean)
        .sum()
}

/// `(sum_c sum I^2, sum_c S_c^2 / n)` over the window at `origin`.
fn window_energies<T>(
    integral: &Integral,
    means: &[f64],
    template: Plane<T>,
    origin: (usize, usize),
    n: f64,
) -> (f64, f64) {
    let size = (template.width, template.height);
    (0..means.len()).fold((0.0, 0.0), |(sq_total, mean_sq), c| {
        let (sum, sq) = integral.window(origin, size, c);
        (sq_total + sq, mean_sq + sum * sum / n)
    })
}

/// OpenCV's `common_matchTemplate` snapping: exact quotient below `t`, `±1` up to 12.5 % above it,
/// else 0. With `t == 0` (flat window) the result is 0.
fn snap(num: f64, t: f64) -> f64 {
    if num.abs() < t {
        num / t
    } else if num.abs() < t * 1.125 {
        if num > 0.0 {
            1.0
        } else {
            -1.0
        }
    } else {
        0.0
    }
}
