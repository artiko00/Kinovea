use crate::measure::{AngleFlags, Scope};
use crate::measure_test::{close, metadata, prefs};
use kinovea_core::preferences::enums::ExportSpace;
use kinovea_core::timecode::TimecodeFormat;
use kinovea_core::units::AngleUnit;
use kinovea_geometry::PointF;
use std::f32::consts::FRAC_PI_2;

fn flags(signed: bool, ccw: bool, supplementary: bool) -> AngleFlags {
    AngleFlags {
        signed,
        counter_clockwise: ccw,
        supplementary,
    }
}

#[test]
fn unsigned_angle_is_the_signed_one_plus_a_turn_when_negative() {
    let m = metadata(1000.0, 0);
    let p = prefs(TimecodeFormat::ClassicTime, ExportSpace::WorldSpace);
    let s = Scope {
        metadata: &m,
        prefs: &p,
    };
    let o = PointF::new(320.0, 240.0);
    let (a, b) = (PointF::new(420.0, 240.0), PointF::new(320.0, 140.0));
    let signed = s.angle(o, b, a, flags(true, true, false));
    let unsigned = s.angle(o, b, a, flags(false, true, false));
    let expected = if signed < 0.0 { signed + 360.0 } else { signed };
    assert!(
        close(unsigned, expected),
        "signed {signed} unsigned {unsigned}"
    );
}

#[test]
fn supplementary_angle_of_a_45_degree_angle_is_135() {
    let m = metadata(1000.0, 0);
    let p = prefs(TimecodeFormat::ClassicTime, ExportSpace::WorldSpace);
    let s = Scope {
        metadata: &m,
        prefs: &p,
    };
    let o = PointF::new(320.0, 240.0);
    let (a, b) = (PointF::new(420.0, 240.0), PointF::new(420.0, 140.0));
    assert!(close(
        s.angle(o, a, b, flags(false, true, false)).abs(),
        45.0
    ));
    assert!(close(
        s.angle(o, a, b, flags(false, true, true)).abs(),
        135.0
    ));
}

#[test]
fn degenerate_angle_has_no_value() {
    let m = metadata(1000.0, 0);
    let p = prefs(TimecodeFormat::ClassicTime, ExportSpace::WorldSpace);
    let s = Scope {
        metadata: &m,
        prefs: &p,
    };
    let o = PointF::new(320.0, 240.0);
    assert_eq!(
        s.angle(o, o, PointF::new(1.0, 1.0), flags(true, true, false)),
        0.0
    );
}

#[test]
fn angle_unit_radian_skips_the_degree_conversion() {
    let mut m = metadata(1000.0, 0);
    let mut units = m.calibration.units();
    units.angle = AngleUnit::Radian;
    m.calibration.set_units(units);
    let p = prefs(TimecodeFormat::ClassicTime, ExportSpace::WorldSpace);
    let s = Scope {
        metadata: &m,
        prefs: &p,
    };
    let o = PointF::new(320.0, 240.0);
    let v = s.angle(
        o,
        PointF::new(420.0, 240.0),
        PointF::new(320.0, 140.0),
        flags(false, true, false),
    );
    assert!(close(v.abs(), FRAC_PI_2), "got {v}");
}
