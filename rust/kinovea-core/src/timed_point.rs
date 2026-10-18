//! A tracked position with timestamp and radius (port of `TimedPoint.cs`).
//! Its `x;y;t;r` text form is stored inside KVA tracks and is a file-format contract.

use crate::units_precision::format_f32_default;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TimedPoint {
    pub x: f32,
    pub y: f32,
    pub t: i64,
    pub r: f32,
}

/// Error returned when the `x;y;t[;r]` text cannot be parsed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid timed point: {0}")]
pub struct InvalidTimedPoint(pub String);

impl TimedPoint {
    pub const fn new(x: f32, y: f32, t: i64) -> Self {
        Self { x, y, t, r: 0.0 }
    }

    pub const fn with_radius(x: f32, y: f32, t: i64, r: f32) -> Self {
        Self { x, y, t, r }
    }

    /// Xor of the field hashes as `.NET` computes them, used for change detection.
    pub fn content_hash(&self) -> i32 {
        dotnet_f32_hash(self.x)
            ^ dotnet_f32_hash(self.y)
            ^ dotnet_i64_hash(self.t)
            ^ dotnet_f32_hash(self.r)
    }
}

fn dotnet_f32_hash(v: f32) -> i32 {
    if v == 0.0 {
        0
    } else {
        v.to_bits() as i32
    }
}

fn dotnet_i64_hash(v: i64) -> i32 {
    (v as i32) ^ ((v >> 32) as i32)
}

impl fmt::Display for TimedPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{};{};{};{}",
            format_f32_default(self.x),
            format_f32_default(self.y),
            self.t,
            format_f32_default(self.r)
        )
    }
}

impl FromStr for TimedPoint {
    type Err = InvalidTimedPoint;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || InvalidTimedPoint(s.to_owned());
        let parts: Vec<&str> = s.split(';').collect();
        if parts.len() < 3 {
            return Err(err());
        }
        let x = parts[0].trim().parse().map_err(|_| err())?;
        let y = parts[1].trim().parse().map_err(|_| err())?;
        let t = parts[2].trim().parse().map_err(|_| err())?;
        let r = match parts.get(3) {
            Some(r) => r.trim().parse().map_err(|_| err())?,
            None => 0.0,
        };
        Ok(Self { x, y, t, r })
    }
}
