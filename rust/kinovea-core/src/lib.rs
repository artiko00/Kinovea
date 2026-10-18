//! Kinovea `core` crate: timecodes, units, video time primitives and preferences.

/// Declares an enum whose text form is the C# member name (`ToString()` / `Enum.Parse`).
#[macro_export]
macro_rules! text_enum {
    ($(#[$doc:meta])* $name:ident [$($v:ident),+ $(,)?]) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name { $($v),+ }

        impl $name {
            pub const ALL: &'static [$name] = &[$(Self::$v),+];

            pub fn name(self) -> &'static str {
                match self { $(Self::$v => stringify!($v)),+ }
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, String> {
                $(if s == stringify!($v) { return Ok(Self::$v); })+
                Err(format!("unknown {}: {s}", stringify!($name)))
            }
        }
    };
}

#[cfg(test)]
mod lib_test;

pub mod preferences;
pub mod timecode;
mod timecode_format;
#[cfg(test)]
mod timecode_format_test;
#[cfg(test)]
mod timecode_test;
pub mod timed_point;
#[cfg(test)]
mod timed_point_test;
pub mod timestamp_mapper;
#[cfg(test)]
mod timestamp_mapper_test;
pub mod units;
pub mod units_convert;
#[cfg(test)]
mod units_convert_test;
pub mod units_precision;
#[cfg(test)]
mod units_precision_test;
#[cfg(test)]
mod units_test;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
pub mod version;
#[cfg(test)]
mod version_test;
pub mod video_section;
#[cfg(test)]
mod video_section_test;
pub mod xml;
#[cfg(test)]
mod xml_test;
