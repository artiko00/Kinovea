use crate::drawing::{Chrono, Drawing, Line};
use crate::history::{HistoryStack, Memento};
use crate::keyframe::Keyframe;
use crate::kva::{write_kva, ExportFlags};
use crate::kva_io::Filter;
use crate::metadata::{Manager, Metadata};
use kinovea_geometry::PointF;

/// Document as the KVA writer produces it, the reference for undo and redo.
pub(crate) fn document(m: &Metadata) -> String {
    write_kva(m, "0.0", false, ExportFlags::FULL).to_document()
}

/// One keyframe with a named line (and its tracker) and a named stopwatch.
pub(crate) fn sample() -> Metadata {
    let mut m = Metadata::default();
    m.merge_insert_keyframe(Keyframe::new(100, "first"));
    let mut line = Drawing::Line(Line::new(PointF::EMPTY, PointF::new(1.0, 2.0)));
    line.header_mut().name = "Line 1".to_owned();
    let kid = m.keyframes[0].id;
    m.add_drawing(Manager::Keyframe(kid), line).unwrap();
    let mut chrono = Drawing::Chrono(Chrono::new(PointF::new(5.0, 5.0)));
    chrono.header_mut().name = "Stopwatch 1".to_owned();
    m.chronos.push(chrono);
    m.register_trackables();
    m
}

/// Pushes `memento`, applies `change`, then checks that undo restores the document and redo
/// applies it again. Returns the stack for further checks.
pub(crate) fn round_trip(m: &mut Metadata, memento: Memento, change: impl FnOnce(&mut Metadata)) -> HistoryStack {
    let before = document(m);
    let mut stack = HistoryStack::default();
    stack.push(memento);
    change(m);
    let after = document(m);
    assert_ne!(before, after, "the change must modify the document");
    stack.step_backward(m);
    assert_eq!(document(m), before);
    stack.step_forward(m);
    assert_eq!(document(m), after);
    stack
}

#[test]
fn push_clears_the_redo_stack() {
    let mut m = sample();
    let id = m.keyframes[0].id;
    let mut stack = HistoryStack::default();
    stack.push(Memento::add_keyframe(id));
    stack.step_backward(&mut m);
    assert!(stack.can_redo());

    stack.push(Memento::add_keyframe(id));

    assert!(!stack.can_redo());
    assert!(stack.can_undo());
}

#[test]
fn identical_modify_drawing_mementos_are_not_stacked() {
    let mut m = sample();
    let id = m.chronos[0].id();
    let mut stack = HistoryStack::default();
    stack.push(Memento::modify_drawing(&m, Manager::Chronos, id, Filter::KVA));
    stack.push(Memento::modify_drawing(&m, Manager::Chronos, id, Filter::KVA));

    stack.step_backward(&mut m);

    assert!(!stack.can_undo());
}

#[test]
fn modify_drawing_mementos_with_other_states_are_stacked() {
    let mut m = sample();
    let mut stack = HistoryStack::default();
    let id = m.chronos[0].id();
    stack.push(Memento::modify_drawing(&m, Manager::Chronos, id, Filter::KVA));
    m.chronos[0].header_mut().name = "renamed".to_owned();
    stack.push(Memento::modify_drawing(&m, Manager::Chronos, id, Filter::KVA));

    stack.step_backward(&mut m);

    assert!(stack.can_undo());
}

#[test]
fn command_names_follow_the_undo_and_redo_stacks() {
    let mut m = sample();
    let chrono = m.chronos[0].clone();
    let memento = Memento::delete_drawing(&m, Manager::Chronos, &chrono).unwrap();
    let mut stack = round_trip(&mut m, memento, |m| {
        m.delete_drawing(Manager::Chronos, chrono.id()).unwrap();
    });
    assert_eq!(stack.undo_action_name(), "Delete drawing (Stopwatch 1)");
    assert_eq!(stack.redo_action_name(), "");

    stack.step_backward(&mut m);

    assert_eq!(stack.undo_action_name(), "");
    assert_eq!(stack.redo_action_name(), "Delete drawing (Stopwatch 1)");
}
