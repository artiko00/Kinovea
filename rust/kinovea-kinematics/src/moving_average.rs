//! Centred moving average with reflected padding (`MovingAverage`), used to smooth
//! derivatives of high frame rate footage.

/// Averages `samples` over a window of `span / (2 · interval)` samples on each side, where
/// `interval = 1000 / fs` ms. Returns the input unchanged when half the span does not exceed
/// one interval. `sentinels` NaN-padded values at each end are excluded from the reflection.
pub fn filter_samples(samples: &[f64], fs: f64, span_ms: f64, sentinels: usize) -> Vec<f64> {
    let interval = 1000.0 / fs;
    if span_ms / 2.0 <= interval {
        return samples.to_vec();
    }
    let padding = 50.min(samples.len().saturating_sub(sentinels * 4));
    let padded = add_padding(samples, padding, sentinels);
    let frames = (span_ms / (2.0 * interval)) as usize;
    let smoothed: Vec<f64> = (0..padded.len())
        .map(|i| {
            let first = i.saturating_sub(frames);
            let last = (i + frames).min(padded.len() - 1);
            padded[first..=last].iter().sum::<f64>() / (last - first + 1) as f64
        })
        .collect();
    smoothed[padding..smoothed.len() - padding].to_vec()
}

fn add_padding(samples: &[f64], padding: usize, sentinels: usize) -> Vec<f64> {
    let n = samples.len();
    let mut padded = vec![0.0; n + 2 * padding];
    let pivot = sentinels;
    for i in 0..padding + sentinels {
        padded[i] = samples[pivot] + (samples[pivot] - samples[pivot + padding - i + sentinels]);
    }
    padded[padding + sentinels..padding + n - sentinels].copy_from_slice(&samples[sentinels..n - sentinels]);
    let pivot = n - 1 - sentinels;
    for i in 0..padding + sentinels {
        padded[padding + n - sentinels + i] = samples[pivot] + (samples[pivot] - samples[pivot - i - 1]);
    }
    padded
}
