use crate::backend::recording::{Call, RecordingRenderer};
use crate::renderer::{Brush, Renderer, Transform};
use kinovea_geometry::RectangleF;
use kinovea_metadata::Color;

#[test]
fn reset_transform_is_undone_by_pop() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    let shift = Transform::translate(5.0, 5.0);
    let brush = Brush::solid(Color::BLACK);
    r.push_transform(shift);
    r.push_transform(Transform::scale(2.0, 2.0));
    r.reset_transform();
    r.fill_ellipse(RectangleF::EMPTY, &brush);
    r.pop_transform();
    r.fill_ellipse(RectangleF::EMPTY, &brush);
    let transforms: Vec<Transform> = r
        .calls
        .iter()
        .filter_map(|c| match c {
            Call::FillEllipse(_, _, transform) => Some(*transform),
            _ => None,
        })
        .collect();
    assert_eq!(transforms, vec![Transform::identity(), shift]);
}

#[test]
fn push_clip_intersects_and_pop_restores() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    r.push_clip(RectangleF::new(10.0, 10.0, 50.0, 50.0));
    r.push_clip(RectangleF::new(0.0, 0.0, 30.0, 30.0));
    assert_eq!(r.clip_bounds(), RectangleF::new(10.0, 10.0, 20.0, 20.0));
    r.pop_clip();
    assert_eq!(r.clip_bounds(), RectangleF::new(10.0, 10.0, 50.0, 50.0));
    r.pop_clip();
    assert_eq!(r.clip_bounds(), RectangleF::new(0.0, 0.0, 100.0, 100.0));
}

#[test]
fn state_changes_are_recorded_in_issue_order() {
    let mut r = RecordingRenderer::new(100.0, 100.0);
    let shift = Transform::translate(1.0, 2.0);
    let rect = RectangleF::new(0.0, 0.0, 4.0, 4.0);
    r.push_transform(shift);
    r.push_clip(rect);
    r.pop_clip();
    r.pop_transform();
    assert_eq!(
        r.calls,
        vec![
            Call::PushTransform(shift),
            Call::PushClip(rect),
            Call::PopClip,
            Call::PopTransform
        ]
    );
}
