//! One filtered series with its cutoff and residual score (`FilteringResult`).

#[derive(Debug, Clone, PartialEq)]
pub struct FilteringResult {
    /// Cutoff frequency in Hz.
    pub cutoff_frequency: f64,
    /// Filtered samples, same length as the input.
    pub data: Vec<f64>,
    /// Normalized Durbin-Watson score `|2 − dw| / 2`: 0 is uncorrelated residuals.
    pub durbin_watson: f64,
}

impl FilteringResult {
    pub fn new(cutoff_frequency: f64, data: Vec<f64>, durbin_watson: f64) -> Self {
        FilteringResult {
            cutoff_frequency,
            data,
            durbin_watson,
        }
    }

    /// `|2 − dw| / 2`.
    pub fn normalized_score(dw: f64) -> f64 {
        (2.0 - dw).abs() / 2.0
    }
}
