use crate::history::Memento;
use crate::history_test::{round_trip, sample};

#[test]
fn undo_of_modify_video_filter_restores_the_filter_parameters() {
    let mut m = sample();
    let memento = Memento::modify_video_filter(&m, "Brightness");

    let stack = round_trip(&mut m, memento, |m| {
        m.video_filters.push_leaf("Brightness", "42");
    });

    assert_eq!(stack.undo_action_name(), "Modify Brightness");
}
