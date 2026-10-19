use crate::posture::catalog::{custom_postures, CUSTOM_XML};
use crate::posture::engine::{grab_segment_middle, move_handle, Space};
use crate::posture::model::*;
use crate::posture::parser::parse_posture;
use kinovea_fixtures::fixtures_root;
use kinovea_geometry::{PointF, Size};
use kinovea_measurement::{CalibrationHelper, Units};

fn goniometer() -> GenericPosture {
    custom_postures().into_iter().find(|p| p.name == "Goniometer").unwrap()
}

#[test]
fn keep_angle_impact_follows_the_moved_point() {
    let mut g = goniometer();
    let before = kinovea_geometry::helpers::angle_at(g.point_list[1], g.point_list[0], g.point_list[2]);
    move_handle(&mut g, Space(None), 0, PointF::new(100.0, 200.0), false);
    assert_eq!(g.point_list[0], PointF::new(100.0, 200.0));
    let after = kinovea_geometry::helpers::angle_at(g.point_list[1], g.point_list[0], g.point_list[2]);
    assert!((before.cos() - after.cos()).abs() < 1e-4 && (before.sin() - after.sin()).abs() < 1e-4, "{before} vs {after}");
    assert!((kinovea_geometry::helpers::distance(g.point_list[1], g.point_list[2]) - 50.0).abs() < 1e-3);
    assert_eq!(g.point_list[3], PointF::new(180.0, 70.0), "untouched point");
}

#[test]
fn locked_and_inactive_constraints() {
    let mut g = goniometer();
    g.handles[3].constraint = Some(Constraint { kind: ConstraintKind::LockedInPlace, option_group: String::new() });
    move_handle(&mut g, Space(None), 3, PointF::new(1.0, 1.0), false);
    assert_eq!(g.point_list[3], PointF::new(180.0, 70.0));
    g.handles[3].constraint = Some(Constraint { kind: ConstraintKind::LockedInPlace, option_group: "lock".into() });
    g.options.push(PostureOption { key: "lock".into(), label: "lock".into(), default_value: false, value: false, hidden: false });
    move_handle(&mut g, Space(None), 3, PointF::new(1.0, 1.0), false);
    assert_eq!(g.point_list[3], PointF::new(1.0, 1.0), "option off → constraint inactive → free move");
    move_handle(&mut g, Space(None), 99, PointF::new(5.0, 5.0), false);
}

/// Oracle parity: every handle of every shipped tool moved to the fixture targets.
#[test]
fn moves_match_the_oracle() {
    let root = fixtures_root().join("posture");
    let calibration = CalibrationHelper::without_tracking(Size::new(640, 480), Units::default(), 25.0);
    let mut checked = 0;
    for xml in CUSTOM_XML {
        let reference = parse_posture(xml, false).unwrap();
        let slug = slug_of(&reference.name);
        let csv = std::fs::read_to_string(root.join(&slug).join("moves.csv")).unwrap_or_else(|e| panic!("{slug}: {e}"));
        let mut cache: Option<((i32, i32, i32), GenericPosture)> = None;
        for line in csv.lines().skip(1) {
            let f: Vec<&str> = line.split(',').collect();
            let key = (f[0].parse().unwrap(), f[1].parse().unwrap(), f[2].parse().unwrap());
            if cache.as_ref().map(|c| c.0) != Some(key) {
                let mut p = parse_posture(xml, false).unwrap();
                for o in &mut p.options {
                    o.value = key.1 == 1;
                }
                let handle = key.0 as usize;
                if p.handles[handle].kind == HandleType::Segment {
                    grab_segment_middle(&mut p, handle);
                }
                move_handle(&mut p, Space(Some(&calibration)), handle, PointF::new(f[3].parse().unwrap(), f[4].parse().unwrap()), false);
                cache = Some((key, p));
            }
            let p = &cache.as_ref().unwrap().1;
            let ctx = format!("{slug} handle {} options {} move {} {}", key.0, key.1, key.2, f[5]);
            let (x, y): (f64, f64) = (f[6].parse().unwrap(), f[7].parse().unwrap());
            if let Some(index) = f[5].strip_prefix("circle") {
                kinovea_fixtures::assert_close(f64::from(p.circles[index.parse::<usize>().unwrap()].radius), x, 0.0, &ctx);
            } else {
                let pt = p.point_list[f[5].parse::<usize>().unwrap()];
                kinovea_fixtures::assert_close(f64::from(pt.x), x, 2e-3, &ctx);
                kinovea_fixtures::assert_close(f64::from(pt.y), y, 2e-3, &ctx);
            }
            checked += 1;
        }
    }
    assert!(checked > 1000, "{checked} rows checked");
}

/// Fixture directory of each shipped tool (case files were named after the tool files).
pub(super) const SLUGS: &[(&str, &str)] = &[
    ("Goniometer", "goniometer"),
    ("Posture", "posture"),
    ("Bikefit", "bike-fit"),
    ("Genu", "genu-flexum"),
    ("Archery", "archery-top-view"),
    ("AngleToVertical", "angle-to-vertical"),
    ("AngleToHorizontal", "angle-to-horizontal"),
    ("HorizontalDistance", "horizontal-distance"),
    ("HumanModel", "human-model"),
    ("HumanModel2", "human-model-2"),
    ("CSVL", "central-sacral-vertical-line"),
    ("BodyWithFeet", "body-with-feet"),
    ("Canis", "canis"),
    ("Equus", "equus"),
    ("OpenPoseBody25", "openpose-body25"),
    ("Profile", "profile"),
];

pub(super) fn slug_of(name: &str) -> String {
    SLUGS.iter().find(|(n, _)| *n == name).map(|(_, s)| (*s).to_owned()).unwrap_or_else(|| panic!("no fixture slug for {name}"))
}
