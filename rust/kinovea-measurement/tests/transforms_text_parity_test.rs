//! Value texts and circle ellipses of `CalibrationHelper` against the transforms fixtures.

mod support;
use support::calibrations::{build, NAMES};
use support::*;

use kinovea_geometry::shapes::Circle;

#[test]
fn texts_match_exactly_and_ellipses_within_1e_2() {
    for name in NAMES {
        let h = build(name);
        let (header, rows) = table(&format!("transforms/{name}.csv"));
        for (i, cells) in rows.iter().enumerate() {
            let r = Row {
                header: &header,
                cells,
            };
            let ctx = format!("{name}[{i}]");
            let (ip, q) = (r.point("ix", "iy"), r.point("qx", "qy"));
            assert_eq!(
                h.point_text(ip, true, 0),
                r.text("point_text"),
                "{ctx} point text"
            );
            assert_eq!(
                h.length_text(ip, q, true),
                r.text("length_text"),
                "{ctx} length text"
            );
            assert_eq!(
                h.circumference_text(ip, q, true),
                r.text("circ_text"),
                "{ctx} circumference text"
            );
            let w = h.point(ip);
            let e = h.ellipse_from_circle(Circle::new(w, h.scalar(r.f("s") as f32).abs()));
            assert_pt(
                e.center,
                r.point("ecx", "ecy"),
                1e-2,
                &format!("{ctx} ellipse centre"),
            );
            assert_f(
                f64::from(e.semi_major_axis),
                r.f("ea"),
                1e-2,
                &format!("{ctx} ellipse a"),
            );
            assert_f(
                f64::from(e.semi_minor_axis),
                r.f("eb"),
                1e-2,
                &format!("{ctx} ellipse b"),
            );
            assert_f(
                f64::from(e.rotation),
                r.f("erot"),
                1e-4,
                &format!("{ctx} ellipse rotation"),
            );
        }
    }
}
