//! Unit conversions and calibration precision text (port of `UnitHelper.cs`).
//! Float widths follow the original (`f32` where C# uses `float`).

use crate::units::{
    AccelerationUnit, AngularAccelerationUnit, AngularVelocityUnit, CadenceUnit, LengthUnit,
    SpeedUnit,
};

const MILLIMETER: f64 = 0.001;
const CENTIMETER: f64 = 0.01;
const INCH: f64 = 0.0254;
const FOOT: f64 = 0.3048;
const YARD: f64 = 0.9144;
const KILOMETER: f64 = 1000.0;
const MILE: f64 = 1609.344;
const NAUTICAL_MILE: f64 = 1852.0;
const RADIANS_TO_DEGREES: f64 = 180.0 / std::f64::consts::PI;

fn is_native(unit: LengthUnit) -> bool {
    matches!(unit, LengthUnit::Pixels | LengthUnit::Percentage)
}

/// Any length unit to metres (`UnitHelper.GetMeters(float, LengthUnit)`).
fn meters_from_length(length: f32, unit: LengthUnit) -> f32 {
    let l = f64::from(length);
    let m = match unit {
        LengthUnit::Millimeters => l * MILLIMETER,
        LengthUnit::Centimeters => l * CENTIMETER,
        LengthUnit::Meters => l,
        LengthUnit::Inches => l * INCH,
        LengthUnit::Feet => l * FOOT,
        LengthUnit::Yards => l * YARD,
        LengthUnit::Percentage | LengthUnit::Pixels => 0.0,
    };
    m as f32
}

/// Length implied by a speed unit, to metres (`UnitHelper.GetMeters(float, SpeedUnit)`).
fn meters_from_speed(speed: f32, unit: SpeedUnit) -> f32 {
    let s = f64::from(speed);
    let m = match unit {
        SpeedUnit::MetersPerSecond | SpeedUnit::PixelsPerSecond => s,
        SpeedUnit::KilometersPerHour => s * KILOMETER,
        SpeedUnit::FeetPerSecond => s * FOOT,
        SpeedUnit::MilesPerHour => s * MILE,
        SpeedUnit::Knots => s * NAUTICAL_MILE,
    };
    m as f32
}

fn length_for_speed_unit(length: f32, length_unit: LengthUnit, speed_unit: SpeedUnit) -> f32 {
    if is_native(length_unit) {
        return length;
    }
    let meters = f64::from(meters_from_length(length, length_unit));
    let r = match speed_unit {
        SpeedUnit::FeetPerSecond => meters / FOOT,
        SpeedUnit::MetersPerSecond => meters,
        SpeedUnit::KilometersPerHour => meters / KILOMETER,
        SpeedUnit::MilesPerHour => meters / MILE,
        SpeedUnit::Knots => meters / NAUTICAL_MILE,
        SpeedUnit::PixelsPerSecond => f64::from(length),
    };
    r as f32
}

/// Velocity in length units per second to the speed unit (`ConvertVelocity`).
pub fn convert_velocity(v: f32, length_unit: LengthUnit, unit: SpeedUnit) -> f32 {
    let per_second = length_for_speed_unit(v, length_unit, unit);
    match unit {
        SpeedUnit::FeetPerSecond | SpeedUnit::MetersPerSecond | SpeedUnit::PixelsPerSecond => {
            per_second
        }
        SpeedUnit::KilometersPerHour | SpeedUnit::MilesPerHour | SpeedUnit::Knots => {
            per_second * 3600.0
        }
    }
}

/// Acceleration in length units per second squared to the acceleration unit
/// (`ConvertAcceleration`). Calibrated space with pixel output is not supported and yields 0.
pub fn convert_acceleration(a: f32, length_unit: LengthUnit, unit: AccelerationUnit) -> f32 {
    if is_native(length_unit) {
        return a;
    }
    let meters = f64::from(meters_from_length(a, length_unit));
    let r = match unit {
        AccelerationUnit::FeetPerSecondSquared => meters / FOOT,
        AccelerationUnit::MetersPerSecondSquared => meters,
        AccelerationUnit::PixelsPerSecondSquared => 0.0,
    };
    r as f32
}

/// Radians per second to the angular velocity unit (`ConvertAngularVelocity`).
pub fn convert_angular_velocity(radians_per_second: f64, unit: AngularVelocityUnit) -> f64 {
    match unit {
        AngularVelocityUnit::DegreesPerSecond => radians_per_second * RADIANS_TO_DEGREES,
        AngularVelocityUnit::RadiansPerSecond => radians_per_second,
        AngularVelocityUnit::RevolutionsPerMinute => {
            radians_per_second / (2.0 * std::f64::consts::PI) * 60.0
        }
    }
}

/// Radians per second squared to the angular acceleration unit (`ConvertAngularAcceleration`).
pub fn convert_angular_acceleration(radians: f32, unit: AngularAccelerationUnit) -> f32 {
    match unit {
        AngularAccelerationUnit::DegreesPerSecondSquared => {
            (f64::from(radians) * RADIANS_TO_DEGREES) as f32
        }
        AngularAccelerationUnit::RadiansPerSecondSquared => radians,
    }
}

/// Hertz to the cadence unit (`ConvertFrequency`); only the time base changes.
pub fn convert_frequency(hertz: f32, unit: CadenceUnit) -> f32 {
    use CadenceUnit::*;
    match unit {
        Hertz | CyclesPerSecond | StepsPerSecond | StrokesPerSecond => hertz,
        CyclesPerMinute | StepsPerMinute | StrokesPerMinute | RevolutionsPerMinute => hertz * 60.0,
    }
}

/// Value in the length implied by a speed unit, to a length unit (`ConvertForLengthUnit`).
pub fn convert_for_length_unit(v: f32, speed_unit: SpeedUnit, length_unit: LengthUnit) -> f32 {
    if speed_unit == SpeedUnit::PixelsPerSecond {
        return v;
    }
    let mps = f64::from(meters_from_speed(v, speed_unit));
    let r = match length_unit {
        LengthUnit::Centimeters => mps / CENTIMETER,
        LengthUnit::Feet => mps / FOOT,
        LengthUnit::Inches => mps / INCH,
        LengthUnit::Meters => mps,
        LengthUnit::Millimeters => mps / MILLIMETER,
        LengthUnit::Yards => mps / YARD,
        LengthUnit::Percentage | LengthUnit::Pixels => f64::from(v),
    };
    r as f32
}
