//! Port of `RangeHelper.FindUsableStepSize`: a step of 1, 2, 5 or 10 times a power of ten.

/// `FindUsableStepSize(range, targetSteps)`: the nicest step that gives about `target_steps` divisions.
pub fn find_usable_step_size(range: f32, target_steps: f32) -> f32 {
    let minimum = range / target_steps;
    // `(float)Math.Floor(Math.Log10(minimum))` and `(float)Math.Pow(10, magnitude)`, in double.
    let magnitude = (minimum as f64).log10().floor() as f32;
    let order = 10f64.powf(magnitude as f64) as f32;
    let residual = minimum / order;
    if residual > 5.0 {
        10.0 * order
    } else if residual > 2.0 {
        5.0 * order
    } else if residual > 1.0 {
        2.0 * order
    } else {
        order
    }
}
