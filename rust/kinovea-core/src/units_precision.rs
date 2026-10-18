//! Calibration precision text and significant-digit rounding (`UnitHelper.GetPixelSize`,
//! `UnitHelper.MinimalDigits`), plus the `.NET` default `float.ToString()` they rely on.

use crate::units::LengthUnit;

/// `.NET Framework` `float.ToString()`: up to 7 significant digits, trailing zeros trimmed,
/// scientific notation (`1E-05`) when the exponent is below -5 or at least 7.
pub fn format_f32_default(value: f32) -> String {
    if value == 0.0 {
        return "0".to_owned();
    }
    let sci = format!("{:.6e}", f64::from(value));
    let (mantissa, exponent) = sci.split_once('e').expect("exponent");
    let exponent: i32 = exponent.parse().expect("exponent value");
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let digits = digits.trim_end_matches('0');
    let digits = if digits.is_empty() { "0" } else { digits };
    let sign = if value < 0.0 { "-" } else { "" };
    if (-4..7).contains(&exponent) {
        let point = exponent + 1;
        if point <= 0 {
            format!("{sign}0.{}{digits}", "0".repeat((-point) as usize))
        } else if (point as usize) >= digits.len() {
            format!(
                "{sign}{digits}{}",
                "0".repeat(point as usize - digits.len())
            )
        } else {
            let (int, frac) = digits.split_at(point as usize);
            format!("{sign}{int}.{frac}")
        }
    } else {
        let (first, rest) = digits.split_at(1);
        let frac = if rest.is_empty() {
            String::new()
        } else {
            format!(".{rest}")
        };
        let esign = if exponent < 0 { "-" } else { "+" };
        format!("{sign}{first}{frac}E{esign}{:02}", exponent.abs())
    }
}

/// `.NET` `Math.Round(double)`: round half to even.
fn round_even(v: f64) -> f64 {
    v.round_ties_even()
}

/// Limits the number of significant digits (`UnitHelper.MinimalDigits`).
pub fn minimal_digits(value: f32) -> f32 {
    let magnitude = f64::from(value).log10().floor() as f32;
    if magnitude >= 1.0 {
        round_even(f64::from(value)) as f32
    } else if magnitude >= 0.0 {
        (round_even(f64::from(value) * 10.0) / 10.0) as f32
    } else {
        let scale = 10f64.powf(f64::from(magnitude)) as f32;
        (f64::from(scale) * round_even(f64::from(value / scale))) as f32
    }
}

/// "Units per pixel" text describing calibration precision (`UnitHelper.GetPixelSize`).
/// Metric units are lowered (m → cm → mm) when the value is below one unit.
pub fn pixel_size_text(
    world1: f32,
    world2: f32,
    pixel1: f32,
    pixel2: f32,
    unit: LengthUnit,
) -> String {
    if unit == LengthUnit::Pixels {
        return String::new();
    }
    let mut upp1 = world1 / pixel1;
    let mut upp2 = world2 / pixel2;
    let smallest = upp1.min(upp2);
    let magnitude = f64::from(smallest).log10() as f32;
    let mut unit = unit;
    if unit == LengthUnit::Meters {
        if magnitude < -2.0 {
            upp1 *= 1000.0;
            upp2 *= 1000.0;
            unit = LengthUnit::Millimeters;
        } else if magnitude < 0.0 {
            upp1 *= 100.0;
            upp2 *= 100.0;
            unit = LengthUnit::Centimeters;
        }
    } else if unit == LengthUnit::Centimeters && magnitude < 0.0 {
        upp1 *= 10.0;
        upp2 *= 10.0;
        unit = LengthUnit::Millimeters;
    }
    format!(
        "{} × {} {}",
        format_f32_default(minimal_digits(upp1)),
        format_f32_default(minimal_digits(upp2)),
        unit.abbreviation()
    )
}
