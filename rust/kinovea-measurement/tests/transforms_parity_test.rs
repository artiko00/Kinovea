//! `CalibrationHelper` value computers against `tests/fixtures/calibration/transforms/*.csv`.

mod support;
use support::calibrations::{build, NAMES};
use support::*;

#[test]
fn transforms_match_within_1e_3() {
    for name in NAMES {
        let h = build(name);
        let (header, rows) = table(&format!("transforms/{name}.csv"));
        assert_eq!(rows.len(), 25, "{name}");
        for (i, cells) in rows.iter().enumerate() {
            let r = Row {
                header: &header,
                cells,
            };
            let ctx = format!("{name}[{i}]");
            let (ip, q) = (r.point("ix", "iy"), r.point("qx", "qy"));
            let w = h.point(ip);
            assert_pt(w, r.point("wx", "wy"), 1e-3, &format!("{ctx} world"));
            assert_pt(
                h.image_point(w),
                r.point("bx", "by"),
                1e-2,
                &format!("{ctx} image"),
            );
            let s = r.f("s") as f32;
            assert_f(
                f64::from(h.scalar(s)),
                r.f("gs"),
                1e-3,
                &format!("{ctx} scalar"),
            );
            assert_f(
                f64::from(h.image_scalar(s)),
                r.f("gis"),
                1e-2,
                &format!("{ctx} image scalar"),
            );
            assert_f(
                f64::from(h.image_length(w, h.point(q))),
                r.f("ilen"),
                1e-2,
                &format!("{ctx} image length"),
            );
            assert_f(
                f64::from(h.time(r.f("frames") as i32)),
                r.f("time"),
                1e-9,
                &format!("{ctx} time"),
            );
            assert_pt(
                h.origin(),
                r.point("ox", "oy"),
                1e-3,
                &format!("{ctx} origin"),
            );
            let conversions = [
                ("cs", h.convert_speed(s)),
                ("ca", h.convert_acceleration(s)),
                ("cav", h.convert_acceleration_from_velocity(s)),
                ("cang", h.convert_angle(s / 100.0)),
                ("cangv", h.convert_angular_velocity(s / 100.0)),
                ("cangacc", h.convert_angular_acceleration(s / 100.0)),
            ];
            for (col, got) in conversions {
                assert_f(f64::from(got), r.f(col), 1e-3, &format!("{ctx} {col}"));
            }
        }
    }
}
