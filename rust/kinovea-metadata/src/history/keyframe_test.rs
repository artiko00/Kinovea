use crate::history::Memento;
use crate::history_test::{round_trip, sample};
use crate::keyframe::Keyframe;

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
