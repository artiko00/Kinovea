//! Measurement units and their abbreviations (port of `Types/Units/*.cs` and the
//! abbreviation tables of `UnitHelper.cs`). Conversions live in `units_convert`.

use crate::timecode::TimecodeFormat;
use std::fmt;
use std::str::FromStr;

/// Error returned when a unit name is unknown.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown {kind} unit: {name}")]
pub struct UnknownUnit {
    pub kind: &'static str,
    pub name: String,
}

macro_rules! unit_enum {
    ($(#[$doc:meta])* $name:ident, $kind:literal, [$($variant:ident => $abbr:literal),+ $(,)?]) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name { $($variant),+ }

        impl $name {
            /// Every value, in declaration order.
            pub const ALL: &'static [$name] = &[$(Self::$variant),+];

            /// The C# enum member name, used in KVA and preference files.
            pub fn name(self) -> &'static str {
                match self { $(Self::$variant => stringify!($variant)),+ }
            }

            /// The abbreviation shown next to values.
            pub fn abbreviation(self) -> &'static str {
                match self { $(Self::$variant => $abbr),+ }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.name())
            }
        }

        impl FromStr for $name {
            type Err = UnknownUnit;
            fn from_str(s: &str) -> Result<Self, UnknownUnit> {
                Self::ALL.iter().copied().find(|u| u.name() == s).ok_or_else(|| UnknownUnit {
                    kind: $kind,
                    name: s.to_owned(),
                })
            }
        }
    };
}

unit_enum!(
    /// Length units. `Pixels` is the native unit; `Percentage` abbreviates as `%` unless the
    /// user configured a custom abbreviation in the preferences.
    LengthUnit, "length",
    [Millimeters => "mm", Centimeters => "cm", Meters => "m", Inches => "in", Feet => "ft",
     Yards => "yd", Percentage => "%", Pixels => "px"]
);

unit_enum!(
    /// Speed units.
    SpeedUnit, "speed",
    [MetersPerSecond => "m/s", KilometersPerHour => "km/h", FeetPerSecond => "ft/s",
     MilesPerHour => "mph", Knots => "kn", PixelsPerSecond => "px/s"]
);

unit_enum!(
    /// Acceleration units.
    AccelerationUnit, "acceleration",
    [MetersPerSecondSquared => "m/s²", FeetPerSecondSquared => "ft/s²", PixelsPerSecondSquared => "px/s²"]
);

unit_enum!(
    /// Angle units.
    AngleUnit, "angle",
    [Degree => "°", Radian => "rad"]
);

unit_enum!(
    /// Angular velocity units.
    AngularVelocityUnit, "angular velocity",
    [DegreesPerSecond => "deg/s", RadiansPerSecond => "rad/s", RevolutionsPerMinute => "rpm"]
);

unit_enum!(
    /// Angular acceleration units.
    AngularAccelerationUnit, "angular acceleration",
    [DegreesPerSecondSquared => "deg/s²", RadiansPerSecondSquared => "rad/s²"]
);

unit_enum!(
    /// Cadence (frequency) units. Activity based, so some abbreviations repeat.
    CadenceUnit, "cadence",
    [Hertz => "Hz", CyclesPerSecond => "cyc./s", CyclesPerMinute => "cyc./min",
     StepsPerSecond => "sps", StepsPerMinute => "spm", StrokesPerSecond => "sps",
     StrokesPerMinute => "spm", RevolutionsPerMinute => "rpm"]
);

/// Symbol for a timecode format (`UnitHelper.TimeAbbreviation`). Non-numerical formats
/// are converted to seconds by the exporters, hence `s`.
pub fn time_abbreviation(format: TimecodeFormat) -> &'static str {
    match format {
        TimecodeFormat::Frames => "F",
        TimecodeFormat::HundredthOfMinutes => "min/100",
        TimecodeFormat::Microseconds => "μs",
        TimecodeFormat::Milliseconds => "ms",
        TimecodeFormat::TenThousandthOfHours => "hour/10000",
        TimecodeFormat::Timestamps => "ts",
        _ => "s",
    }
}
