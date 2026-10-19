//! Number text of the measurement labels: `Math.Round` to a number of decimal places, then the
//! .NET Framework text of the result.

use kinovea_geometry::helpers::round;

/// `PlayerPreferences.decimalPlaces` default, the precision of graduation and angle labels.
pub const DEFAULT_DECIMAL_PLACES: i32 = 2;

/// `String.Format("{0}", Math.Round(value, decimal_places))`: ties go to the even digit and negative
/// zero prints as `0`.
pub fn format_rounded(value: f32, decimal_places: i32) -> String {
    let rounded = round(value, decimal_places);
    if rounded == 0.0 {
        "0".to_string()
    } else {
        format!("{rounded}")
    }
}

/// `format_rounded` with the default precision.
pub fn format_value(value: f32) -> String {
    format_rounded(value, DEFAULT_DECIMAL_PLACES)
}
