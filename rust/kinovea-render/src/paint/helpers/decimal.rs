//! Number text of the measurement labels (`Math.Round` with the player's decimal places).

/// `PlayerPreferences.decimalPlaces` default, the precision of graduation and angle labels.
pub const DEFAULT_DECIMAL_PLACES: i32 = 2;

/// `String.Format("{0}", Math.Round(value, DecimalPlaces))` with .NET Framework output: ties go to
/// the even digit and negative zero prints as `0`.
pub fn format_value(value: f32) -> String {
    let power = 10f64.powi(DEFAULT_DECIMAL_PLACES);
    let rounded = (value as f64 * power).round_ties_even() / power;
    if rounded == 0.0 {
        "0".to_string()
    } else {
        format!("{rounded}")
    }
}
