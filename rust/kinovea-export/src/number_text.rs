//! Text of a single-precision number as `float.ToString(nfi)` writes it on .NET Framework 4.8: the
//! "G" format with seven significant digits, fixed notation only when -5 < exponent < 7.

#[cfg(test)]
#[path = "number_text_test.rs"]
mod number_text_test;

/// Formats `value` with `decimal` as the decimal separator (`NaN`, `Infinity` as invariant culture).
pub fn to_text(value: f32, decimal: char) -> String {
    if value.is_nan() {
        return "NaN".to_owned();
    }
    if value.is_infinite() {
        return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_owned();
    }
    let sign = if value < 0.0 { "-" } else { "" };
    let scientific = format!("{:.6e}", value.abs());
    let (mantissa, exponent) = scientific.split_once('e').expect("finite value");
    let exponent: i32 = exponent.parse().expect("exponent");
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let body = if exponent > -5 && exponent < 7 {
        fixed(&digits, exponent, decimal)
    } else {
        exponential(&digits, exponent, decimal)
    };
    format!("{sign}{body}")
}

fn fixed(digits: &str, exponent: i32, decimal: char) -> String {
    let padded = format!("{}{digits}", "0".repeat((-exponent - 1).max(0) as usize));
    let (whole, fraction) = if exponent >= 0 {
        digits.split_at(exponent as usize + 1)
    } else {
        ("0", padded.as_str())
    };
    let fraction = fraction.trim_end_matches('0');
    if fraction.is_empty() {
        whole.to_owned()
    } else {
        format!("{whole}{decimal}{fraction}")
    }
}

fn exponential(digits: &str, exponent: i32, decimal: char) -> String {
    let (first, rest) = digits.split_at(1);
    let rest = rest.trim_end_matches('0');
    let fraction = if rest.is_empty() {
        String::new()
    } else {
        format!("{decimal}{rest}")
    };
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{first}{fraction}E{sign}{:02}", exponent.abs())
}
