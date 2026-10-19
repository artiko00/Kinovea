use super::common::testing::Scene;
use super::posture::hit_posture;
use super::posture_test::handle;
use super::Hit;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::model::{
    ComputedPoint, GenericPosture, HandleType, HitZone, PostureCircle, Segment, Stroke,
    WeightedPoint, TRANSPARENT,
};
use uuid::Uuid;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn drawing(posture: GenericPosture) -> GenericPostureDrawing {
    GenericPostureDrawing::new(Uuid::nil(), Some(posture))
}

// DrawingGenericPosture.cs:1081-1090 and 1172-1185: a computed point (index -1) is the weighted
// middle of its children, here of (0,0) and (100,0); the segment from it to (50,100) is the body.
#[test]
fn segment_from_a_computed_point_is_body() {
    let scene = Scene::at_scale(1.0);
    let middle = ComputedPoint {
        name: "mid".into(),
        weight: 1.0,
        color: TRANSPARENT,
        symbol: String::new(),
        option_group: String::new(),
        display_coordinates: false,
        children: vec![
            WeightedPoint::Reference {
                reference: 0,
                weight: 0.5,
            },
            WeightedPoint::Reference {
                reference: 1,
                weight: 0.5,
            },
        ],
    };
    let segment = Segment {
        name: String::new(),
        start: -1,
        end: 2,
        stroke: Stroke::default(),
        option_group: String::new(),
    };
    let posture = GenericPosture {
        point_list: vec![p(0.0, 0.0), p(100.0, 0.0), p(50.0, 100.0)],
        computed_points: vec![middle],
        segments: vec![segment],
        ..Default::default()
    };
    assert_eq!(
        hit_posture(&drawing(posture), p(50.0, 50.0), &scene.ctx(0)),
        Hit::Body
    );
}

// DrawingGenericPosture.cs:1204-1212 (HitArc) and 1195-1203 (HitDisc): the circle outline is handle 1,
// its disc is body.
#[test]
fn circle_outline_is_handle_and_disc_is_body() {
    let scene = Scene::at_scale(1.0);
    let circle = PostureCircle {
        name: String::new(),
        center: 0,
        radius: 20,
        stroke: Stroke::default(),
        option_group: String::new(),
    };
    let posture = GenericPosture {
        point_list: vec![p(100.0, 100.0)],
        circles: vec![circle],
        handles: vec![handle(HandleType::Circle, 0, "")],
        ..Default::default()
    };
    let d = drawing(posture);
    assert_eq!(
        hit_posture(&d, p(120.0, 100.0), &scene.ctx(0)),
        Hit::Handle(1)
    );
    assert_eq!(hit_posture(&d, p(105.0, 100.0), &scene.ctx(0)), Hit::Body);
}

// DrawingGenericPosture.cs:1123-1131 (HitPolygon): a hit zone is body over its polygon.
#[test]
fn hit_zone_polygon_is_body() {
    let scene = Scene::at_scale(1.0);
    let posture = GenericPosture {
        point_list: vec![p(0.0, 0.0), p(100.0, 0.0), p(100.0, 100.0), p(0.0, 100.0)],
        hit_zones: vec![HitZone {
            points: vec![0, 1, 2, 3],
        }],
        ..Default::default()
    };
    let d = drawing(posture);
    assert_eq!(hit_posture(&d, p(50.0, 50.0), &scene.ctx(0)), Hit::Body);
    assert_eq!(hit_posture(&d, p(200.0, 50.0), &scene.ctx(0)), Hit::Miss);
}
