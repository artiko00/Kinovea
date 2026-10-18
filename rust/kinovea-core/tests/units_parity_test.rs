//! Parity with `UnitHelper` using the oracle tables under `tests/fixtures/units/conversions/`.

use kinovea_core::timecode::TimecodeFormat;
use kinovea_core::units::*;
use kinovea_core::units_convert::*;
use kinovea_core::units_precision::pixel_size_text;
use kinovea_fixtures::fixtures_root;

fn rows(file: &str) -> Vec<Vec<String>> {
    let path = fixtures_root().join("units/conversions").join(file);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    text.lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(str::to_owned).collect())
        .collect()
}

fn assert_close(got: f64, want: f64, ctx: &str) {
    let tol = 1e-6 * want.abs().max(1e-30);
    assert!(
        kinovea_fixtures::approx_eq(got, want, tol),
        "{ctx}: got {got}, want {want}"
    );
}

fn f(s: &str) -> f64 {
    s.parse().unwrap()
}

#[test]
fn abbreviations_match() {
    let rows = rows("abbreviations.csv");
    assert_eq!(rows.len(), 41);
    for r in rows {
        let (kind, unit, want) = (r[0].as_str(), r[1].as_str(), r[2].as_str());
        let got = match kind {
            "Length" => unit.parse::<LengthUnit>().unwrap().abbreviation(),
            "Speed" => unit.parse::<SpeedUnit>().unwrap().abbreviation(),
            "Acceleration" => unit.parse::<AccelerationUnit>().unwrap().abbreviation(),
            "Angle" => unit.parse::<AngleUnit>().unwrap().abbreviation(),
            "AngularVelocity" => unit.parse::<AngularVelocityUnit>().unwrap().abbreviation(),
            "AngularAcceleration" => unit
                .parse::<AngularAccelerationUnit>()
                .unwrap()
                .abbreviation(),
            "Cadence" => unit.parse::<CadenceUnit>().unwrap().abbreviation(),
            "Time" => time_abbreviation(unit.parse::<TimecodeFormat>().unwrap()),
            _ => panic!("unknown kind {kind}"),
        };
        assert_eq!(got, want, "{kind}/{unit}");
    }
}

#[test]
fn velocity_and_acceleration_match() {
    for r in rows("velocity.csv") {
        let got = convert_velocity(
            f(&r[0]) as f32,
            r[1].parse().unwrap(),
            r[2].parse().unwrap(),
        );
        assert_close(
            f64::from(got),
            f(&r[3]),
            &format!("velocity {} {} {}", r[0], r[1], r[2]),
        );
    }
    for r in rows("acceleration.csv") {
        let got = convert_acceleration(
            f(&r[0]) as f32,
            r[1].parse().unwrap(),
            r[2].parse().unwrap(),
        );
        assert_close(
            f64::from(got),
            f(&r[3]),
            &format!("acceleration {} {} {}", r[0], r[1], r[2]),
        );
    }
    for r in rows("length_from_speed.csv") {
        let got = convert_for_length_unit(
            f(&r[0]) as f32,
            r[1].parse().unwrap(),
            r[2].parse().unwrap(),
        );
        assert_close(
            f64::from(got),
            f(&r[3]),
            &format!("length {} {} {}", r[0], r[1], r[2]),
        );
    }
}

#[test]
fn angular_and_frequency_match() {
    for r in rows("angular_velocity.csv") {
        let got = convert_angular_velocity(f(&r[0]), r[1].parse().unwrap());
        assert_close(
            got,
            f(&r[2]),
            &format!("angular velocity {} {}", r[0], r[1]),
        );
    }
    for r in rows("angular_acceleration.csv") {
        let got = convert_angular_acceleration(f(&r[0]) as f32, r[1].parse().unwrap());
        assert_close(
            f64::from(got),
            f(&r[2]),
            &format!("angular acceleration {} {}", r[0], r[1]),
        );
    }
    for r in rows("frequency.csv") {
        let got = convert_frequency(f(&r[0]) as f32, r[1].parse().unwrap());
        assert_close(
            f64::from(got),
            f(&r[2]),
            &format!("frequency {} {}", r[0], r[1]),
        );
    }
}

#[test]
fn pixel_size_text_matches_exactly() {
    let rows = rows("pixel_size.csv");
    assert_eq!(rows.len(), 12);
    for r in rows {
        let got = pixel_size_text(
            f(&r[0]) as f32,
            f(&r[1]) as f32,
            f(&r[2]) as f32,
            f(&r[3]) as f32,
            r[4].parse().unwrap(),
        );
        assert_eq!(
            got, r[5],
            "pixel size {} {} {} {} {}",
            r[0], r[1], r[2], r[3], r[4]
        );
    }
}
