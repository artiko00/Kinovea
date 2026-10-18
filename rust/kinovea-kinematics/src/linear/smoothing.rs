//! Second pass over derivatives for high frame rate footage: a moving average whose span is a
//! fixed duration, so it only acts when the frame interval is shorter than half the span.

use crate::component::Kinematics;
use crate::moving_average::filter_samples;
use crate::time_series::TimeSeriesCollection;

/// Span applied to the three velocity series (1 sentinel).
pub const VELOCITY_SPAN_MS: f64 = 40.0;
/// Span applied to the three acceleration series (2 sentinels).
pub const ACCELERATION_SPAN_MS: f64 = 50.0;

/// Replaces each series in `keys` by its moving average.
pub fn smooth(tsc: &mut TimeSeriesCollection, keys: &[Kinematics], fps: f64, span_ms: f64, sentinels: usize) {
    for key in keys {
        let averaged = filter_samples(&tsc[*key], fps, span_ms, sentinels);
        tsc[*key] = averaged;
    }
}
