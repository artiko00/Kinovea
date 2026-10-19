use crate::history::{HistoryStack, Memento};
use crate::history_test::{round_trip, sample};
use crate::metadata::Metadata;

fn text_of<'a>(m: &'a Metadata, name: &str) -> Option<&'a str> {
    m.video_filters.child_text(name)
}

fn set_text(m: &mut Metadata, name: &str, text: &str) {
    if let Some(c) = m.video_filters.children.iter_mut().find(|c| c.name == name) {
        c.text = text.to_owned();
    }
}

#[test]
fn undo_of_modify_video_filter_restores_the_filter_parameters() {
    let mut m = sample();
    let memento = Memento::modify_video_filter(&m, "Brightness");

    let stack = round_trip(&mut m, memento, |m| {
        m.video_filters.push_leaf("Brightness", "42");
    });

    assert_eq!(stack.undo_action_name(), "Modify Brightness");
}

#[test]
fn undo_of_modify_video_filter_leaves_the_other_filters_as_they_are() {
    let mut m = sample();
    m.video_filters.push_leaf("Brightness", "1");
    m.video_filters.push_leaf("Contrast", "2");
    let mut stack = HistoryStack::default();
    stack.push(Memento::modify_video_filter(&m, "Brightness"));

    set_text(&mut m, "Brightness", "42");
    set_text(&mut m, "Contrast", "9");
    stack.step_backward(&mut m);

    assert_eq!(text_of(&m, "Brightness"), Some("1"));
    assert_eq!(text_of(&m, "Contrast"), Some("9"));
}
