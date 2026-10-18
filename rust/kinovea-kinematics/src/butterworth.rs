//! Zero-lag second-order Butterworth low-pass with automatic cutoff selection
//! (`ButterworthFilter`, after Winter and Challis).

mod padding;
#[cfg(test)]
mod padding_test;

pub use padding::{add_padding, remove_padding};

use crate::filtering_result::FilteringResult;
use crate::stats::{durbin_watson, subtract};
use std::fmt;

const SQRT2: f64 = std::f64::consts::SQRT_2;
const PADDING: usize = 10;

/// Fewer than 11 samples were supplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooFewSamples;

impl fmt::Display for TooFewSamples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "number of samples must be superior to 10")
    }
}

impl std::error::Error for TooFewSamples {}

/// Every filtered series tried, plus the index of the one with the lowest residual score
/// (`None` when no score was finite).
#[derive(Debug, Clone, PartialEq)]
pub struct Filtered {
    pub results: Vec<FilteringResult>,
    pub best_cutoff_index: Option<usize>,
}

/// Second-order section coefficients for one pass.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coefficients {
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub b1: f64,
    pub b2: f64,
}

/// Correction factor so that `passes` cascaded passes keep the −3 dB point at `fc`.
pub fn correction_factor(passes: u32) -> f64 {
    (2f64.powf(1.0 / f64::from(passes)) - 1.0).powf(0.25)
}

/// Coefficients for sampling frequency `fs` and cutoff `fc`, corrected for two passes.
pub fn coefficients(fs: f64, fc: f64, correction: f64) -> Coefficients {
    let o = (std::f64::consts::PI * fc / fs).tan() / correction;
    let k1 = SQRT2 * o;
    let k2 = o * o;
    let a0 = k2 / (1.0 + k1 + k2);
    let a1 = 2.0 * a0;
    let a2 = a0;
    let k3 = 2.0 * a0 / k2;
    let b1 = -2.0 * a0 + k3;
    let b2 = 1.0 - a0 - a1 - a2 - b1;
    Coefficients { a0, a1, a2, b1, b2 }
}

/// Filters `samples` at `fc_tests` cutoffs between 0.5 Hz and Nyquist and scores each result.
pub fn filter_samples(samples: &[f64], fs: f64, fc_tests: usize) -> Result<Filtered, TooFewSamples> {
    if samples.len() <= 10 {
        return Err(TooFewSamples);
    }
    let correction = correction_factor(2);
    let padded = add_padding(samples, PADDING);
    let min = 0.5;
    let max = fs / 2.0;
    let step = (max - min) / fc_tests as f64;
    let mut results = Vec::new();
    let mut best = (1.0, None);
    let mut fc = min;
    while fc < max {
        let filtered = remove_padding(&two_pass(&padded, coefficients(fs, fc, correction)), PADDING);
        let dw = durbin_watson(&subtract(samples, &filtered));
        fc += step;
        if dw.is_nan() {
            continue;
        }
        let score = FilteringResult::normalized_score(dw);
        if score < best.0 {
            best = (score, Some(results.len()));
        }
        results.push(FilteringResult::new(fc - step, filtered, score));
    }
    Ok(Filtered {
        results,
        best_cutoff_index: best.1,
    })
}

/// Forward then backward pass; the first two outputs of each pass copy the inputs.
pub fn two_pass(samples: &[f64], c: Coefficients) -> Vec<f64> {
    let forward = one_pass(samples, c);
    let reversed: Vec<f64> = forward.iter().rev().copied().collect();
    let mut backward = one_pass(&reversed, c);
    backward.reverse();
    backward
}

fn one_pass(raw: &[f64], c: Coefficients) -> Vec<f64> {
    let (mut x1, mut x0) = (raw[0], raw[1]);
    let (mut y1, mut y0) = (raw[0], raw[1]);
    let mut out = Vec::with_capacity(raw.len());
    out.push(y1);
    out.push(y0);
    for &sample in &raw[2..] {
        let x2 = x1;
        x1 = x0;
        x0 = sample;
        let y2 = y1;
        y1 = y0;
        y0 = c.a0 * x0 + c.a1 * x1 + c.a2 * x2 + c.b1 * y1 + c.b2 * y2;
        out.push(y0);
    }
    out
}
