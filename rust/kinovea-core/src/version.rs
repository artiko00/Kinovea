//! Application version `YYYY.N[.R]` (port of `ThreePartsVersion.cs`).

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ThreePartsVersion {
    pub major: i32,
    pub minor: i32,
    pub revision: i32,
}

/// Error returned when the version text is not `major.minor[.revision]`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid version: {0}")]
pub struct InvalidVersion(pub String);

impl ThreePartsVersion {
    pub const fn new(major: i32, minor: i32, revision: i32) -> Self {
        Self {
            major,
            minor,
            revision,
        }
    }
}

impl fmt::Display for ThreePartsVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.revision == 0 {
            write!(f, "{}.{}", self.major, self.minor)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.revision)
        }
    }
}

impl FromStr for ThreePartsVersion {
    type Err = InvalidVersion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || InvalidVersion(s.to_owned());
        let mut parts = s.split('.');
        let major = parts.next().ok_or_else(err)?.parse().map_err(|_| err())?;
        let minor = parts.next().ok_or_else(err)?.parse().map_err(|_| err())?;
        let revision = match parts.next() {
            Some(r) => r.parse().map_err(|_| err())?,
            None => 0,
        };
        Ok(Self::new(major, minor, revision))
    }
}
