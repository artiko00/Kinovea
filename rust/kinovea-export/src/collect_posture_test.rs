use crate::collect_drawings::KeyframeMeasures;
use crate::collect_posture::collect_posture;
use crate::measure::Scope;
use crate::measured_data::ExportPrefs;
use kinovea_core::preferences::enums::ExportSpace;
use kinovea_core::timecode::TimecodeFormat;
use kinovea_geometry::PointF;
use kinovea_metadata::color::Color;
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::{
    posture_by_id, ComputedPoint, GenericPosture, Measure, WeightedPoint,
};
use kinovea_metadata::Metadata;
use kinovea_metadata::Uuid;

const GONIOMETER: &str = "02ea93f9-ba00-424b-8c87-07e156e0252f";

fn prefs() -> ExportPrefs {
    ExportPrefs {
        export_space: ExportSpace::ImageSpace,
        timecode_format: TimecodeFormat::ClassicTime,
        producer: String::new(),
    }
}

fn goniometer() -> GenericPosture {
    posture_by_id(Uuid::parse_str(GONIOMETER).unwrap(), true).unwrap()
}

fn measure(name: &str, point1: i32, point2: i32) -> Measure {
    Measure {
        name: name.to_owned(),
        point1,
        point2,
        symbol: String::new(),
        color: Color::default(),
        option_group: String::new(),
    }
}

fn drawing(name: &str, posture: GenericPosture) -> GenericPostureDrawing {
    let mut d = GenericPostureDrawing::new(posture.id, Some(posture));
    d.header.name = name.to_owned();
    d
}

fn collect(d: &GenericPostureDrawing) -> KeyframeMeasures {
    let m = Metadata::default();
    let p = prefs();
    let mut out = KeyframeMeasures::default();
    let scope = Scope {
        metadata: &m,
        prefs: &p,
    };
    collect_posture(&scope, d, 1.5, &mut out);
    out
}

#[test]
fn unnamed_angle_takes_the_drawing_name() {
    let out = collect(&drawing("Goniometer 1", goniometer()));
    assert_eq!(out.angles.len(), 1);
    assert_eq!(out.angles[0].name, "Goniometer 1");
    assert!((out.angles[0].value - 45.0).abs() < 1e-4);
    assert_eq!(out.angles[0].time, 1.5);
}

#[test]
fn named_parts_are_appended_to_the_drawing_name() {
    let mut p = goniometer();
    p.positions = vec![measure("Hip", 1, 1), measure("", -1, -1)];
    p.distances = vec![measure("Span", 0, 2)];
    let out = collect(&drawing("Goniometer 1", p));
    assert_eq!(out.positions.len(), 1, "a negative point index is skipped");
    assert_eq!(out.positions[0].name, "Goniometer 1 - Hip");
    assert_eq!(out.positions[0].point, PointF::new(150.0, 100.0));
    assert_eq!(out.distances[0].name, "Goniometer 1 - Span");
    assert!((out.distances[0].value - 100.0).abs() < 1e-4);
}

#[test]
fn computed_points_are_positions_at_their_blended_location() {
    let mut p = goniometer();
    p.computed_points = vec![ComputedPoint {
        name: "Centre".to_owned(),
        weight: 0.0,
        color: Color::default(),
        symbol: String::new(),
        option_group: String::new(),
        display_coordinates: false,
        children: vec![
            WeightedPoint::Reference {
                reference: 0,
                weight: 0.5,
            },
            WeightedPoint::Reference {
                reference: 2,
                weight: 0.5,
            },
        ],
    }];
    let out = collect(&drawing("Goniometer 1", p));
    let centre = out.positions.last().unwrap();
    assert_eq!(centre.name, "Goniometer 1 - Centre");
    assert_eq!(centre.point, PointF::new(150.0, 100.0));
}

#[test]
fn a_drawing_without_a_shipped_tool_contributes_nothing() {
    let out = collect(&GenericPostureDrawing::new(Uuid::nil(), None));
    assert_eq!(out, KeyframeMeasures::default());
}
