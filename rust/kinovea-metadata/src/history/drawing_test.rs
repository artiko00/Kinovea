use crate::drawing::{Drawing, Line};
use crate::history::Memento;
use crate::history_test::{round_trip, sample};
use crate::metadata::Manager;
use kinovea_geometry::PointF;

fn named_line(name: &str) -> Drawing {
    let mut d = Drawing::Line(Line::new(PointF::new(3.0, 3.0), PointF::new(9.0, 4.0)));
    d.header_mut().name = name.to_owned();
    d
}

#[test]
fn undo_of_add_drawing_removes_it_and_redo_restores_it() {
    let mut m = sample();
    let kid = m.keyframes[0].id;
    let line = named_line("Line 2");
    let memento = Memento::add_drawing(Manager::Keyframe(kid), &line);

    let stack = round_trip(&mut m, memento, |m| {
        m.add_drawing(Manager::Keyframe(kid), line.clone());
        m.register_trackables();
    });

    assert_eq!(stack.undo_action_name(), "Add drawing (Line 2)");
}

#[test]
fn undo_of_delete_drawing_restores_the_drawing_and_its_tracker() {
    let mut m = sample();
    let kid = m.keyframes[0].id;
    let line = m.keyframes[0].drawings[0].clone();
    let id = line.id();
    let memento = Memento::delete_drawing(&m, Manager::Keyframe(kid), &line);

    let stack = round_trip(&mut m, memento, |m| {
        m.delete_drawing(Manager::Keyframe(kid), id);
    });

    assert_eq!(stack.undo_action_name(), "Delete drawing (Line 1)");
}

#[test]
fn undo_of_delete_chrono_restores_it_in_the_chrono_list() {
    let mut m = sample();
    let chrono = m.chronos[0].clone();
    let id = chrono.id();
    let memento = Memento::delete_drawing(&m, Manager::Chronos, &chrono);

    round_trip(&mut m, memento, |m| {
        m.delete_drawing(Manager::Chronos, id);
    });

    assert!(m.chronos.is_empty());
}

#[test]
fn undo_of_modify_drawing_restores_the_previous_state() {
    let mut m = sample();
    let kid = m.keyframes[0].id;
    let line = m.keyframes[0].drawings[0].clone();
    let id = line.id();
    let memento = Memento::modify_drawing(&m, Manager::Keyframe(kid), &line);

    let stack = round_trip(&mut m, memento, |m| {
        if let Some(d) = m.drawing_mut(Manager::Keyframe(kid), id) {
            d.header_mut().name = "renamed".to_owned();
        }
    });

    assert_eq!(stack.undo_action_name(), "Modify drawing (Line 1)");
}
