use crate::drawing::{Drawing, Track};
use crate::history::{HistoryStack, Memento};
use crate::history_test::{round_trip, sample};
use crate::keyframe::Keyframe;
use crate::metadata::Metadata;
use kinovea_core::timed_point::TimedPoint;

/// A track from 0 to 200 with the label of a keyframe at 100.
fn push_track_with_label(m: &mut Metadata) {
    let mut track = Track::new(vec![TimedPoint::new(0.0, 0.0, 0), TimedPoint::new(1.0, 1.0, 200)]);
    track.update_keyframe_labels(&[100]);
    m.tracks.push(Drawing::Track(track));
}

fn label_count(m: &Metadata) -> usize {
    match &m.tracks[0] {
        Drawing::Track(t) => t.keyframe_labels.len(),
        _ => 0,
    }
}

#[test]
fn undo_of_add_keyframe_removes_it_and_redo_restores_it() {
    let mut m = sample();
    let kf = Keyframe::new(300, "added");
    let id = kf.id;

    let stack = round_trip(&mut m, Memento::add_keyframe(id), |m| {
        m.merge_insert_keyframe(kf);
    });

    assert_eq!(stack.undo_action_name(), "Add a key image");
}

#[test]
fn undo_of_delete_keyframe_restores_its_drawings_and_trackers() {
    let mut m = sample();
    let id = m.keyframes[0].id;
    let memento = Memento::delete_keyframe(&m, id);

    let stack = round_trip(&mut m, memento, |m| {
        m.delete_keyframe(id);
    });

    assert_eq!(stack.undo_action_name(), "Remove a key image");
}

#[test]
fn deleting_a_keyframe_updates_track_labels_and_undo_restores_them() {
    let mut m = sample();
    let id = m.keyframes[0].id;
    push_track_with_label(&mut m);
    let mut stack = HistoryStack::default();
    stack.push(Memento::delete_keyframe(&m, id));

    m.delete_keyframe(id);
    assert_eq!(label_count(&m), 0);

    stack.step_backward(&mut m);
    assert_eq!(label_count(&m), 1);
}

#[test]
fn undo_of_modify_keyframe_restores_its_core_data() {
    let mut m = sample();
    let id = m.keyframes[0].id;
    let memento = Memento::modify_keyframe(&m, id);

    let stack = round_trip(&mut m, memento, |m| {
        m.keyframes[0].name = "renamed".to_owned();
        m.keyframes[0].comments = "note".to_owned();
    });

    assert_eq!(stack.undo_action_name(), "Modify keyframe");
}

#[test]
fn undo_of_modify_keyframe_does_not_recreate_a_deleted_keyframe() {
    let mut m = sample();
    let id = m.keyframes[0].id;
    let mut stack = HistoryStack::default();
    stack.push(Memento::modify_keyframe(&m, id));

    m.delete_keyframe(id);
    stack.step_backward(&mut m);

    assert!(m.keyframes.is_empty());
}
