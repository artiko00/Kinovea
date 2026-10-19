//! Tests for `wildcard.rs`: `*` and `?` matching of file names, ignoring case.

use crate::wildcard::wildcard_match;

#[test]
fn star_matches_any_run_of_characters() {
    assert!(wildcard_match("*.avi", "clip.avi"));
    assert!(wildcard_match("*", ""));
    assert!(wildcard_match("a*b*c", "aXbYc"));
    assert!(!wildcard_match("*.avi", "clip.avi.txt"));
}

#[test]
fn question_mark_matches_one_character() {
    assert!(wildcard_match("clip?.avi", "clip1.avi"));
    assert!(!wildcard_match("clip?.avi", "clip12.avi"));
    assert!(!wildcard_match("clip?.avi", "clip.avi"));
}

#[test]
fn matching_ignores_case() {
    assert!(wildcard_match("*.AVI", "Clip.avi"));
}
