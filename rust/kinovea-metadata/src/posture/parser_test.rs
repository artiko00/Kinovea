use crate::posture::catalog::CUSTOM_XML;
use crate::posture::model::*;
use crate::posture::parser::parse_posture;
use kinovea_geometry::PointLinePosition;

fn by_name(name: &str) -> GenericPosture {
    CUSTOM_XML.iter().filter_map(|x| parse_posture(x, false)).find(|p| p.name == name).unwrap()
}

#[test]
fn goniometer_structure() {
    let g = by_name("Goniometer");
    assert_eq!(g.display_name, "DrawingName_Goniometer");
    assert_eq!((g.points.len(), g.segments.len(), g.angles.len(), g.handles.len()), (4, 3, 1, 4));
    assert_eq!(g.segments[1].stroke.style, SegmentLineStyle::Dash);
    assert!(g.segments[2].stroke.arrow_end);
    assert_eq!((g.angles[0].radius, g.angles[0].text_distance, g.angles[0].tenth), (-25, 40, true));
    assert_eq!(g.trackable_indices, vec![0, 1, 3]);
    assert!(g.flip_horizontal && g.flip_vertical);
    match &g.handles[2].constraint {
        Some(Constraint { kind: ConstraintKind::RotationSteps { origin, leg1, step, keep_distance }, option_group }) => {
            assert_eq!((*origin, *leg1, *step, *keep_distance), (1, 0, 45, false));
            assert!(option_group.is_empty());
        }
        other => panic!("{other:?}"),
    }
    assert!(matches!(g.handles[0].impacts[0], Impact::KeepAngle { origin: 1, leg1: 0, leg2: 2, .. }));
}

#[test]
fn bike_fit_options_are_consolidated_from_groups() {
    let b = by_name("Bikefit");
    assert_eq!(b.options.len(), 1);
    assert_eq!(b.options[0].key, "Lock segments lengths");
    assert!(!b.options[0].value && !b.options[0].hidden && b.has_non_hidden_options);
    assert_eq!(b.points[0].name, "tip of head");
    assert_eq!(b.points[0].color, crate::color::Color::rgb(0xdc, 0x32, 0x2f));
    assert!(b.flip_horizontal && !b.flip_vertical);
}

#[test]
fn openpose_has_hidden_options_and_the_human_model_computed_points() {
    let o = by_name("OpenPoseBody25");
    assert_eq!(o.point_list.len(), 25);
    assert_eq!(o.points[0].name, "Nose");
    assert_eq!(o.points[8].name, "MidHip");
    assert!(!o.options.is_empty() && !o.has_non_hidden_options);
    let all: Vec<GenericPosture> = CUSTOM_XML.iter().filter_map(|x| parse_posture(x, false)).collect();
    let with_zone = all.iter().find(|p| !p.hit_zones.is_empty()).expect("one tool has a hit zone");
    assert_eq!(with_zone.hit_zones[0].points.len(), 4);
    let with_com = all.iter().find(|p| !p.computed_points.is_empty()).expect("one tool has computed points");
    let com = &with_com.computed_points[0];
    assert_eq!(com.name, "Center of mass");
    assert!(matches!(&com.children[1], WeightedPoint::Computed(trunk) if trunk.name == "Trunk" && trunk.children.len() == 2));
    let location = com.location(&with_com.point_list);
    assert!(location.x > 0.0 && location.y > 0.0);
}

#[test]
fn rejects_other_roots_and_reads_line_slides() {
    assert!(parse_posture("<KinoveaDrawingTool/>", false).is_none());
    let xml = "<KinoveaPostureTool><FormatVersion>1.1</FormatVersion><Points><Point value=\"0;0\"/><Point value=\"10;0\"/><Point value=\"5;5\"/></Points><Handles><Handle type=\"Point\" reference=\"2\"><Constraint type=\"LineSlide\" optionGroup=\"a|b\"><LineSlide point1=\"0\" point2=\"1\" position=\"OnSegment\"/></Constraint></Handle></Handles><Options><Option key=\"a\" label=\"A\" default=\"true\"/><Option key=\"a\" label=\"dup\"/></Options></KinoveaPostureTool>";
    let p = parse_posture(xml, false).unwrap();
    assert_eq!(p.options.len(), 2, "duplicate key skipped, group key b added");
    assert_eq!((p.options[0].label.as_str(), p.options[0].value, p.options[1].key.as_str()), ("A", true, "b"));
    match &p.handles[0].constraint.as_ref().unwrap().kind {
        ConstraintKind::LineSlide { start, end, allowed, margin } => assert_eq!((*start, *end, *allowed, *margin), (0, 1, PointLinePosition::OnSegment, 10)),
        other => panic!("{other:?}"),
    }
}

fn field<'a>(json: &'a str, key: &str) -> &'a str {
    let start = json.find(&format!("\"{key}\": ")).unwrap_or_else(|| panic!("{key}")) + key.len() + 4;
    let rest = &json[start..];
    let end = if rest.starts_with('[') { rest.find(']').unwrap() + 1 } else { rest.find([',', '\n']).unwrap() };
    rest[..end].trim_matches('"')
}

fn list(json: &str, key: &str) -> Vec<String> {
    field(json, key).trim_matches(['[', ']']).split(',').map(|s| s.trim().trim_matches('"').to_owned()).filter(|s| !s.is_empty()).collect()
}

/// Oracle parity of the parsed description of every shipped tool.
#[test]
fn summaries_match_the_oracle() {
    for xml in CUSTOM_XML {
        let p = parse_posture(xml, false).unwrap();
        let slug = super::engine_test::slug_of(&p.name);
        let json = std::fs::read_to_string(kinovea_fixtures::fixtures_root().join("posture").join(&slug).join("summary.json")).unwrap();
        let counts = [("points", p.point_list.len()), ("segments", p.segments.len()), ("circles", p.circles.len()), ("angles", p.angles.len()), ("polylines", p.polylines.len()), ("handles", p.handles.len()), ("hit_zones", p.hit_zones.len()), ("distances", p.distances.len()), ("positions", p.positions.len()), ("computed_points", p.computed_points.len()), ("options", p.options.len())];
        for (key, got) in counts {
            assert_eq!(field(&json, key), got.to_string(), "{slug} {key}");
        }
        assert_eq!(field(&json, "id"), p.id.to_string(), "{slug}");
        assert_eq!(field(&json, "display_name"), p.display_name, "{slug}");
        assert_eq!(field(&json, "has_non_hidden_options"), p.has_non_hidden_options.to_string(), "{slug}");
        assert_eq!(field(&json, "trackable"), (!p.trackable_indices.is_empty()).to_string(), "{slug}");
        assert_eq!(field(&json, "flip_horizontal"), p.flip_horizontal.to_string(), "{slug}");
        assert_eq!(field(&json, "flip_vertical"), p.flip_vertical.to_string(), "{slug}");
        assert_eq!(list(&json, "trackable_points"), p.trackable_indices.iter().map(|i| i.to_string()).collect::<Vec<_>>(), "{slug}");
        assert_eq!(list(&json, "option_keys"), p.options.iter().map(|o| o.key.clone()).collect::<Vec<_>>(), "{slug}");
        let names: Vec<String> = field(&json, "point_names").trim_matches(['[', ']']).split(", ").map(|s| s.trim_matches('"').to_owned()).collect();
        assert_eq!(names, p.points.iter().map(|pt| pt.name.clone()).collect::<Vec<_>>(), "{slug}");
        for (want, got) in list(&json, "point_list").iter().zip(&p.point_list) {
            let (x, y) = kinovea_core::xml::parse_point_f(want);
            assert_eq!((x, y), (got.x, got.y), "{slug}");
        }
        for (want, c) in list(&json, "computed").iter().zip(&p.computed_points) {
            let (x, y) = kinovea_core::xml::parse_point_f(want);
            let got = c.location(&p.point_list);
            assert!((x - got.x).abs() < 1e-3 && (y - got.y).abs() < 1e-3, "{slug} computed {want} vs {got:?}");
        }
    }
}
