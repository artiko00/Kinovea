use crate::drawing::{Chrono, Drawing, Line};
use crate::keyframe::Keyframe;
use crate::metadata::{Manager, Metadata};
use kinovea_geometry::PointF;
use uuid::Uuid;

fn kf(t: i64, name: &str) -> Keyframe {
    let mut k = Keyframe::new(t, name);
    k.add_drawing(Drawing::Line(Line::new(PointF::EMPTY, PointF::new(1.0, t as f32))));
    k
}

#[test]
fn keyframes_stay_sorted_and_merge_by_id_or_time() {
    let mut m = Metadata::default();
    m.merge_insert_keyframe(kf(300, "c"));
    m.merge_insert_keyframe(kf(100, "a"));
    m.merge_insert_keyframe(kf(200, "b"));
    assert_eq!(m.keyframes.iter().map(|k| k.timestamp).collect::<Vec<_>>(), [100, 200, 300]);
    assert_eq!(m.keyframe_index(200), Some(1));
    assert_eq!(m.keyframe_index(250), None);
    let same_time = kf(200, "b2");
    m.merge_insert_keyframe(same_time);
    assert_eq!(m.keyframes.len(), 3);
    assert_eq!(m.keyframes[1].drawings.len(), 2);
    let mut same_id = kf(400, "renamed");
    same_id.id = m.keyframes[0].id;
    let existing = m.keyframes[0].drawings[0].clone();
    same_id.drawings.push(existing);
    m.merge_insert_keyframe(same_id);
    assert_eq!(m.keyframes[0].name, "renamed");
    assert_eq!(m.keyframes[0].drawings.len(), 2);
    assert!(m.keyframe(m.keyframes[0].id).is_some());
}

#[test]
fn find_drawing_and_trackable_registration() {
    let mut m = Metadata::default();
    m.merge_insert_keyframe(kf(10, "k"));
    let chrono = Drawing::Chrono(Chrono::new(PointF::EMPTY));
    let chrono_id = chrono.id();
    m.chronos.push(chrono);
    let line_id = m.keyframes[0].drawings[0].id();
    assert!(matches!(m.find_drawing(line_id), Some((Manager::Keyframe(_), _))));
    assert!(matches!(m.find_drawing(chrono_id), Some((Manager::Chronos, _))));
    assert!(m.find_drawing(Uuid::new_v4()).is_none());
    assert_eq!(m.attached_drawings().count(), 1);
    m.register_trackables();
    assert_eq!(m.trackability.reference_value(line_id, "b"), PointF::new(1.0, 10.0));
    assert_eq!(m.trackability.trackers[0].points[0].1.reference_timestamp, m.keyframes[0].drawings[0].header().reference_timestamp);
}
