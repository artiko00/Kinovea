use super::memento::{named, parse};
use crate::history::{Change, Memento};
use crate::metadata::Metadata;
use uuid::Uuid;

#[test]
fn named_formats_action_and_subject_like_the_original() {
    assert_eq!(named("Add drawing", "Line 1"), "Add drawing (Line 1)");
}

#[test]
fn parse_rejects_empty_and_malformed_data() {
    assert!(parse("").is_none());
    assert!(parse("<Broken").is_none());
    assert!(parse("<Root/>").is_some());
}

#[test]
fn the_inverse_of_an_undo_keeps_the_command_name() {
    let mut m = Metadata::default();
    let memento = Memento { name: "Add a key image".to_owned(), change: Change::AddKeyframe { id: Uuid::nil() } };

    let inverse = memento.perform_undo(&mut m);

    assert_eq!(inverse.command_name(), "Add a key image");
    assert!(matches!(inverse.change, Change::DeleteKeyframe { .. }));
}
