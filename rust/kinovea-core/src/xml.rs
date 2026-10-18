//! Text forms of XML values shared by the KVA and calibration fragments (`XmlHelper`, numeric
//! part). Parsers return the original's fallback (zero) on malformed input instead of failing.

pub use crate::preferences::helpers::parse_bool as parse_boolean;
pub use crate::units_precision::format_f32_default as format_f32_invariant;
use crate::video_section::VideoSection;

/// `XmlHelper.ParseFloat`: invariant culture, 0 when unreadable.
pub fn parse_float(s: &str) -> f32 {
    s.trim().parse().unwrap_or(0.0)
}

fn pair<T: Copy + Default>(s: &str, parse: impl Fn(&str) -> Option<T>) -> (T, T) {
    let mut parts = s.split(';');
    match (parts.next().and_then(&parse), parts.next().and_then(&parse)) {
        (Some(a), Some(b)) => (a, b),
        _ => (T::default(), T::default()),
    }
}

/// `x;y` → `(x, y)`; `(0, 0)` when either part is unreadable.
pub fn parse_point_f(s: &str) -> (f32, f32) {
    pair(s, |p| p.trim().parse().ok())
}

/// `width;height` → `(width, height)` as floats.
pub fn parse_size_f(s: &str) -> (f32, f32) {
    parse_point_f(s)
}

/// `width;height` → `(width, height)` as integers; `(0, 0)` when unreadable.
pub fn parse_size(s: &str) -> (i32, i32) {
    pair(s, |p| p.trim().parse().ok())
}

/// `a;b;c` → integers; empty when any part is unreadable, as the original logs and returns the
/// partial list it had, which is empty for the first failure in practice.
pub fn parse_int_list(s: &str) -> Vec<i32> {
    let mut out = Vec::new();
    for part in s.split(';') {
        match part.trim().parse() {
            Ok(v) => out.push(v),
            Err(_) => return out,
        }
    }
    out
}

/// `12` → 12; `$3` (variable reference) → −4; 0 when unreadable.
pub fn parse_point_reference(s: &str) -> i32 {
    if let Some(var) = s.strip_prefix('$') {
        var.parse::<i32>().map_or(0, |v| -(v + 1))
    } else {
        s.parse().unwrap_or(0)
    }
}

/// `string.Format(Invariant, "{0}", float)`: 7 significant digits.
pub fn write_float(value: f32) -> String {
    format_f32_invariant(value)
}

pub fn write_point_f(x: f32, y: f32) -> String {
    format!("{};{}", write_float(x), write_float(y))
}

pub fn write_rectangle_f(x: f32, y: f32, width: f32, height: f32) -> String {
    format!("{};{}", write_point_f(x, y), write_point_f(width, height))
}

pub fn write_size_f(width: f32, height: f32) -> String {
    write_point_f(width, height)
}

pub fn write_size(width: i32, height: i32) -> String {
    format!("{width};{height}")
}

/// `true` / `false` in lowercase.
pub fn write_boolean(value: bool) -> String {
    value.to_string()
}

/// `start;end` with `long.MaxValue` written as −1.
pub fn write_video_section(section: &VideoSection) -> String {
    let open = |t: i64| if t == i64::MAX { -1 } else { t };
    format!("{};{}", open(section.start), open(section.end))
}

pub fn write_timestamp(timestamp: i64) -> String {
    timestamp.to_string()
}
