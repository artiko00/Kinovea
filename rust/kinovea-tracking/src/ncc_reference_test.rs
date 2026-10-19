//! Brute-force reference shared by the `ncc` tests. Window statistics are computed directly
//! (two passes, no integral images) from the definition of the correlation coefficient.
use crate::ncc::Plane;

/// Pseudo-random bytes from a linear congruential generator (no `rand`).
pub(crate) fn lcg_bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut state = seed;
    (0..len)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 56) as u8
        })
        .collect()
}

pub(crate) fn plane(data: &[u8], width: usize, height: usize, channels: usize) -> Plane<'_, u8> {
    Plane {
        data,
        width,
        height,
        channels,
    }
}

/// Copies the `w x h` block whose top-left corner is `(x, y)`, keeping the channels interleaved.
pub(crate) fn crop(
    data: &[u8],
    width: usize,
    origin: (usize, usize),
    size: (usize, usize),
    channels: usize,
) -> Vec<u8> {
    let (x, y) = origin;
    let (w, h) = size;
    (y..y + h)
        .flat_map(|row| &data[((row * width + x) * channels)..((row * width + x + w) * channels)])
        .copied()
        .collect()
}

/// `TM_CCOEFF_NORMED` at `(x, y)` from its definition: over the template pixels accepted by
/// `included`, the sum of `(I - mean_I)(T - mean_T)` divided by the square root of the product of
/// the sums of squares, with the means and sums taken per channel and accumulated over channels.
pub(crate) fn reference_score(
    image: Plane<u8>,
    template: Plane<u8>,
    (x, y): (usize, usize),
    included: &dyn Fn(usize) -> bool,
) -> f64 {
    let (tw, ch) = (template.width, template.channels);
    let pixels: Vec<usize> = (0..tw * template.height).filter(|&p| included(p)).collect();
    let n = pixels.len() as f64;
    let (mut num, mut di, mut dt) = (0.0, 0.0, 0.0);
    for c in 0..ch {
        let at_image = |p: usize| {
            let (tx, ty) = (p % tw, p / tw);
            f64::from(image.data[((y + ty) * image.width + x + tx) * ch + c])
        };
        let at_template = |p: usize| f64::from(template.data[p * ch + c]);
        let mean_i = pixels.iter().map(|&p| at_image(p)).sum::<f64>() / n;
        let mean_t = pixels.iter().map(|&p| at_template(p)).sum::<f64>() / n;
        for &p in &pixels {
            let (a, b) = (at_image(p) - mean_i, at_template(p) - mean_t);
            num += a * b;
            di += a * a;
            dt += b * b;
        }
    }
    if di * dt == 0.0 {
        0.0
    } else {
        num / (di * dt).sqrt()
    }
}
