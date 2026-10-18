//! Text form of enumerations: the C# member name. Shared macro and trait for every enum
//! stored by name in KVA and preference files.

use std::fmt;
use std::str::FromStr;

/// Error returned when an enumeration name is unknown.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown {kind}: {name}")]
pub struct UnknownName {
    pub kind: &'static str,
    pub name: String,
}

/// An enumeration whose text form is the C# member name.
pub trait NamedEnum: Copy + fmt::Display + FromStr + 'static {
    const ALL: &'static [Self];
    fn name(self) -> &'static str;
}

macro_rules! named_enum {
    ($(#[$doc:meta])* $name:ident, $kind:literal, [$($variant:ident),+ $(,)?]) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name { $($variant),+ }

        impl NamedEnum for $name {
            const ALL: &'static [$name] = &[$(Self::$variant),+];
            fn name(self) -> &'static str {
                match self { $(Self::$variant => stringify!($variant)),+ }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.name())
            }
        }

        impl FromStr for $name {
            type Err = UnknownName;
            fn from_str(s: &str) -> Result<Self, UnknownName> {
                <Self as NamedEnum>::ALL.iter().copied().find(|u| u.name() == s).ok_or_else(|| UnknownName {
                    kind: $kind,
                    name: s.to_owned(),
                })
            }
        }
    };
}
pub(crate) use named_enum;

macro_rules! impl_named_enum {
    ($($t:ty),+ $(,)?) => {$(
        impl NamedEnum for $t {
            const ALL: &'static [$t] = &<$t>::ALL;
            fn name(self) -> &'static str {
                <$t>::name(self)
            }
        }
    )+};
}

impl_named_enum!(
    crate::timecode::TimecodeFormat,
    crate::units::LengthUnit,
    crate::units::SpeedUnit,
    crate::units::AccelerationUnit,
    crate::units::AngleUnit,
    crate::units::AngularVelocityUnit,
    crate::units::AngularAccelerationUnit,
    crate::units::CadenceUnit,
);
