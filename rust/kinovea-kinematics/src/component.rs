//! Keys of the kinematic time series (`Kinematics`) and vector components (`Component`).

use std::fmt;
use std::str::FromStr;

macro_rules! kinematics {
    ($($v:ident),* $(,)?) => {
        /// One kinematic quantity; the name doubles as CSV header.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Kinematics { $($v),* }

        impl Kinematics {
            pub const ALL: &'static [Kinematics] = &[$(Kinematics::$v),*];

            pub fn name(self) -> &'static str {
                match self { $(Kinematics::$v => stringify!($v)),* }
            }
        }

        impl FromStr for Kinematics {
            type Err = UnknownKinematics;
            fn from_str(s: &str) -> Result<Self, UnknownKinematics> {
                match s { $(stringify!($v) => Ok(Kinematics::$v),)* _ => Err(UnknownKinematics(s.to_owned())) }
            }
        }
    };
}

kinematics!(
    X,
    Y,
    XRaw,
    YRaw,
    LinearDistance,
    LinearHorizontalDisplacement,
    LinearVerticalDisplacement,
    LinearSpeed,
    LinearHorizontalVelocity,
    LinearVerticalVelocity,
    LinearAcceleration,
    LinearHorizontalAcceleration,
    LinearVerticalAcceleration,
    AngularPosition,
    AngularDisplacement,
    TotalAngularDisplacement,
    AngularVelocity,
    TangentialVelocity,
    AngularAcceleration,
    TangentialAcceleration,
    CentripetalAcceleration,
    ResultantLinearAcceleration,
);

impl fmt::Display for Kinematics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A name that is not a `Kinematics` variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownKinematics(pub String);

impl fmt::Display for UnknownKinematics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown kinematics component: {}", self.0)
    }
}

impl std::error::Error for UnknownKinematics {}

/// Which part of a vector quantity to measure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    Magnitude,
    Horizontal,
    Vertical,
}
