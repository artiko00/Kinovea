use crate::color::Color;
use crate::drawing::{Drawing, Line};
use crate::history::{HistoryStack, Memento};
use crate::history_test::{round_trip, sample};
use crate::kva_io::Filter;
use crate::metadata::edit::EditError;
use crate::metadata::{Manager, Metadata};
use crate::style::StyleValue;
use kinovea_geometry::PointF;
use uuid::Uuid;

fn named_line(name: &str) -> Drawing {
    let mut d = Drawing::Line(Line::new(PointF::new(3.0, 3.0), PointF::new(9.0, 4.0)));
    d.header_mut().name = name.to_owned();
    d
}

fn line_mut(m: &mut Metadata, kid: Uuid, id: Uuid) -> &mut Line {
    let Some(Drawing::Line(line)) = m.drawing_mut(Manager::Keyframe(kid), id) else {
        panic!("not a line");
    };
    line
}

#[test]
fn undo_of_add_drawing_removes_it_and_redo_restores_it() {
    let mut m = sample();
    let kid = m.keyframes[0].id;
    let line = named_line("Line 2");
    let memento = Memento::add_drawing(Manager::Keyframe(kid), &line).unwrap();

    let stack = round_trip(&mut m, memento, |m| {
        m.add_drawing(Manager::Keyframe(kid), line.clone()).unwrap();
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
    let memento = Memento::delete_drawing(&m, Manager::Keyframe(kid), &line).unwrap();

    let stack = round_trip(&mut m, memento, |m| {
        m.delete_drawing(Manager::Keyframe(kid), id).unwrap();
    });

    assert_eq!(stack.undo_action_name(), "Delete drawing (Line 1)");
}

#[test]
fn undo_of_delete_chrono_restores_it_in_the_chrono_list() {
    let mut m = sample();
    let chrono = m.chronos[0].clone();
    let id = chrono.id();
    let mut stack = HistoryStack::default();
    stack.push(Memento::delete_drawing(&m, Manager::Chronos, &chrono).unwrap());
    m.delete_drawing(Manager::Chronos, id).unwrap();
    assert!(m.chronos.is_empty());

    stack.step_backward(&mut m);

    assert_eq!(m.chronos.iter().map(|c| c.id()).collect::<Vec<_>>(), [id]);
}

#[test]
fn undo_of_modify_drawing_restores_the_previous_state() {
    let mut m = sample();
    let kid = m.keyframes[0].id;
    let id = m.keyframes[0].drawings[0].id();
    let memento = Memento::modify_drawing(&m, Manager::Keyframe(kid), id, Filter::KVA);

    let stack = round_trip(&mut m, memento, |m| {
        if let Some(d) = m.drawing_mut(Manager::Keyframe(kid), id) {
            d.header_mut().name = "renamed".to_owned();
        }
    });

    assert_eq!(stack.undo_action_name(), "Modify drawing (Line 1)");
}

#[test]
fn undo_of_a_style_memento_keeps_the_current_core_data() {
    let mut m = sample();
    let kid = m.keyframes[0].id;
    let id = m.keyframes[0].drawings[0].id();
    let style_only = Filter { core: false, style: true, fading: false };
    let memento = Memento::modify_drawing(&m, Manager::Keyframe(kid), id, style_only);
    let original = m.drawing(Manager::Keyframe(kid), id).and_then(|d| d.header().style.get("color").cloned());
    let mut stack = HistoryStack::default();
    stack.push(memento);

    line_mut(&mut m, kid, id).header.style.insert("color", StyleValue::Color(Color::rgb(9, 9, 9)));
    line_mut(&mut m, kid, id).b = PointF::new(7.0, 7.0);
    // The writer takes trackable points from their registered references, which the player updates
    // when a point moves.
    m.register_trackables();
    stack.step_backward(&mut m);

    let line = line_mut(&mut m, kid, id);
    assert_eq!(line.header.style.get("color"), original.as_ref());
    assert_eq!(line.b, PointF::new(7.0, 7.0));
}

#[test]
fn is_same_state_tells_a_modification_that_changed_nothing() {
    let mut m = sample();
    let chrono = m.chronos[0].clone();
    let memento = Memento::modify_drawing(&m, Manager::Chronos, chrono.id(), Filter::KVA);
    assert!(memento.is_same_state(&m));

    m.chronos[0].header_mut().name = "renamed".to_owned();

    assert!(!memento.is_same_state(&m));
}

#[test]
fn undo_of_modify_coordinate_system_restores_its_flags() {
    let mut m = sample();
    let id = m.coordinate_system.header.id;
    let memento = Memento::modify_drawing(&m, Manager::Singletons, id, Filter::KVA);

    let stack = round_trip(&mut m, memento, |m| m.coordinate_system.show_grid = !m.coordinate_system.show_grid);

    assert!(stack.undo_action_name().starts_with("Modify drawing ("));
}

#[test]
fn singletons_have_no_add_or_delete_mementos() {
    let line = named_line("x");
    assert_eq!(Memento::add_drawing(Manager::Singletons, &line).err(), Some(EditError::Singleton));
    let m = sample();
    assert_eq!(Memento::delete_drawing(&m, Manager::Singletons, &line).err(), Some(EditError::Singleton));
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
