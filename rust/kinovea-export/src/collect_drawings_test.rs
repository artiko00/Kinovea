use crate::collect_drawings::collect_keyframe;
use crate::measure::Scope;
use crate::measured_data::ExportPrefs;
use kinovea_core::preferences::enums::ExportSpace;
use kinovea_core::timecode::TimecodeFormat;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Angle, CrossMark, Drawing, Line};
use kinovea_metadata::Metadata;

fn prefs() -> ExportPrefs {
    ExportPrefs {
        export_space: ExportSpace::ImageSpace,
        timecode_format: TimecodeFormat::ClassicTime,
        producer: String::new(),
    }
}

fn line(name: &str, b: PointF) -> Drawing {
    let mut l = Line::new(PointF::new(0.0, 0.0), b);
    l.header.name = name.to_owned();
    Drawing::Line(l)
}

#[test]
fn drawings_are_grouped_by_kind_and_sorted_by_name() {
    let m = Metadata::default();
    let p = prefs();
    let scope = Scope {
        metadata: &m,
        prefs: &p,
    };
    let mut marker = CrossMark::new(PointF::new(1.0, 2.0));
    marker.header.name = "Marker 1".to_owned();
    let mut angle = Angle::new(
        PointF::new(0.0, 0.0),
        PointF::new(1.0, 0.0),
        PointF::new(0.0, 1.0),
    );
    angle.header.name = "Angle 1".to_owned();
    let drawings = vec![
        line("Line 2", PointF::new(3.0, 4.0)),
        Drawing::CrossMark(marker),
        line("Line 1", PointF::new(0.0, 1.0)),
        Drawing::Angle(angle),
    ];
    let out = collect_keyframe(&scope, &drawings, 0.5);
    let names: Vec<&str> = out.distances.iter().map(|d| d.name.as_str()).collect();
    assert_eq!(names, ["Line 1", "Line 2"]);
    assert_eq!(out.distances[1].value, 5.0);
    assert_eq!(out.positions[0].name, "Marker 1");
    assert_eq!(out.positions[0].point, PointF::new(1.0, 2.0));
    assert_eq!(out.angles.len(), 1);
    assert!(out.distances.iter().all(|d| d.time == 0.5));
}
