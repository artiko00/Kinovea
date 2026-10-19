use super::common::testing::Scene;
use super::posture::hit_posture;
use super::Hit;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::model::{
    GenericPosture, Handle, HandleType, PostureOption, TRANSPARENT,
};
use uuid::Uuid;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

pub(super) fn handle(kind: HandleType, reference: i32, option_group: &str) -> Handle {
    Handle {
        name: String::new(),
        kind,
        color: TRANSPARENT,
        reference,
        trackable: false,
        option_group: option_group.to_owned(),
        grab_point: PointF::EMPTY,
        constraint: None,
        impacts: Vec::new(),
    }
}

fn drawing(posture: GenericPosture) -> GenericPostureDrawing {
    GenericPostureDrawing::new(Uuid::nil(), Some(posture))
}

// DrawingGenericPosture.cs:236-238: a point handle is its point box, numbered from 1.
#[test]
fn point_handle_is_numbered_from_one() {
    let scene = Scene::at_scale(1.0);
    let posture = GenericPosture {
        point_list: vec![p(100.0, 100.0)],
        handles: vec![handle(HandleType::Point, 0, "")],
        ..Default::default()
    };
    assert_eq!(
        hit_posture(&drawing(posture), p(100.0, 100.0), &scene.ctx(0)),
        Hit::Handle(1)
    );
}

// DrawingGenericPosture.cs:1047-1067 (IsActive): a handle whose option is off is skipped.
#[test]
fn handle_of_an_inactive_option_group_is_skipped() {
    let scene = Scene::at_scale(1.0);
    let posture = GenericPosture {
        point_list: vec![p(100.0, 100.0)],
        handles: vec![handle(HandleType::Point, 0, "wrist")],
        options: vec![PostureOption {
            key: "wrist".into(),
            label: String::new(),
            default_value: false,
            value: false,
            hidden: false,
        }],
        ..Default::default()
    };
    assert_eq!(
        hit_posture(&drawing(posture), p(100.0, 100.0), &scene.ctx(0)),
        Hit::Miss
    );
}

// DrawingGenericPosture.cs:242-244: a negative reference skips the handle.
#[test]
fn handle_with_negative_reference_is_skipped() {
    let scene = Scene::at_scale(1.0);
    let posture = GenericPosture {
        point_list: vec![p(100.0, 100.0)],
        handles: vec![handle(HandleType::Point, -1, "")],
        ..Default::default()
    };
    assert_eq!(
        hit_posture(&drawing(posture), p(100.0, 100.0), &scene.ctx(0)),
        Hit::Miss
    );
}
