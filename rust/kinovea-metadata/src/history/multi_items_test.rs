use crate::history::Memento;
use crate::history_test::{round_trip, sample};
use crate::metadata::edit::MultiItem;
use crate::multi::{AutoNumber, Spotlight};
use kinovea_geometry::PointF;

#[test]
fn undo_of_add_spotlight_removes_it_and_redo_restores_it() {
    let mut m = sample();
    let item = MultiItem::Spotlight(Spotlight::new(120, PointF::new(10.0, 10.0), 20));
    let memento = Memento::add_multi_item(&item);

    let stack = round_trip(&mut m, memento, |m| {
        m.add_multi_item(item.clone());
    });

    assert_eq!(stack.undo_action_name(), "Add drawing (Spotlight)");
}

#[test]
fn undo_of_delete_auto_number_restores_it() {
    let mut m = sample();
    let item = MultiItem::AutoNumber(AutoNumber::new(130, PointF::new(1.0, 2.0), 4));
    m.add_multi_item(item.clone());
    let id = item.id();
    let memento = Memento::delete_multi_item(&item);

    let stack = round_trip(&mut m, memento, |m| {
        m.delete_multi_item(id);
    });

    assert_eq!(stack.undo_action_name(), "Delete drawing (Auto numbers)");
}
