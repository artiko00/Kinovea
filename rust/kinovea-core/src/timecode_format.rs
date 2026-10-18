//! Timecode format enumeration and classification (port of `TimeCodeFormat.cs`, `TimeCodeType.cs`).

use std::fmt;
use std::str::FromStr;

/// The timecode formats offered by the application, named as in the KVA files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimecodeFormat {
    ClassicTime,
    Frames,
    Milliseconds,
    Microseconds,
    TenThousandthOfHours,
    HundredthOfMinutes,
    TimeAndFrames,
    Normalized,
    Timestamps,
}

/// Whether a format renders as a plain number or as a structured string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimecodeType {
    Number,
    String,
}

/// Error returned when a format name is unknown.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown timecode format: {0}")]
pub struct UnknownTimecodeFormat(pub String);

impl TimecodeFormat {
    /// Every format, in declaration order.
    pub const ALL: [TimecodeFormat; 9] = [
        Self::ClassicTime,
        Self::Frames,
        Self::Milliseconds,
        Self::Microseconds,
        Self::TenThousandthOfHours,
        Self::HundredthOfMinutes,
        Self::TimeAndFrames,
        Self::Normalized,
        Self::Timestamps,
    ];

    /// The C# enum member name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ClassicTime => "ClassicTime",
            Self::Frames => "Frames",
            Self::Milliseconds => "Milliseconds",
            Self::Microseconds => "Microseconds",
            Self::TenThousandthOfHours => "TenThousandthOfHours",
            Self::HundredthOfMinutes => "HundredthOfMinutes",
            Self::TimeAndFrames => "TimeAndFrames",
            Self::Normalized => "Normalized",
            Self::Timestamps => "Timestamps",
        }
    }
}

impl fmt::Display for TimecodeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for TimecodeFormat {
    type Err = UnknownTimecodeFormat;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|f| f.name() == s)
            .ok_or_else(|| UnknownTimecodeFormat(s.to_owned()))
    }
}

/// Classification used by the UI and exporters (`TimeHelper.GetTimecodeType`).
pub fn timecode_type(format: TimecodeFormat) -> TimecodeType {
    use TimecodeFormat::*;
    match format {
        Frames | Milliseconds | Microseconds | HundredthOfMinutes | TenThousandthOfHours
        | Timestamps => TimecodeType::Number,
        ClassicTime | TimeAndFrames | Normalized => TimecodeType::String,
    }
}
