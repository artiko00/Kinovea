//! Reflected padding used around the Butterworth passes.

/// Prepends and appends `padding` samples reflected about the first and last values.
pub fn add_padding(samples: &[f64], padding: usize) -> Vec<f64> {
    let n = samples.len();
    let first = samples[0];
    let last = samples[n - 1];
    let mut padded = Vec::with_capacity(n + 2 * padding);
    padded.extend((0..padding).map(|i| first + first - samples[padding - i]));
    padded.extend_from_slice(samples);
    padded.extend((0..padding).map(|i| last + last - samples[n - 1 - i - 1]));
    padded
}

/// Drops `padding` samples on each side.
pub fn remove_padding(samples: &[f64], padding: usize) -> Vec<f64> {
    samples[padding..samples.len() - padding].to_vec()
}
