use crate::openpose_sort::compare;
use std::cmp::Ordering::{Equal, Greater, Less};

#[test]
fn digit_runs_compare_by_value() {
    assert_eq!(
        compare("clip_2_keypoints.json", "clip_10_keypoints.json"),
        Less
    );
    assert_eq!(compare("a9", "a10"), Less);
}

#[test]
fn frame_names_sort_in_frame_order() {
    assert_eq!(
        compare(
            "clip_000000000001_keypoints.json",
            "clip_000000000002_keypoints.json"
        ),
        Less
    );
    assert_eq!(
        compare(
            "clip_000000000002_keypoints.json",
            "clip_000000000001_keypoints.json"
        ),
        Greater
    );
}

#[test]
fn extension_is_ignored_and_equal_stems_tie() {
    assert_eq!(compare("clip.json", "clip.txt"), Equal);
}

#[test]
fn shorter_name_sorts_first_when_it_is_a_prefix() {
    assert_eq!(compare("clip", "clip_0"), Less);
    assert_eq!(compare("clip_0", "clip"), Greater);
}

#[test]
fn letters_compare_ordinally() {
    assert_eq!(compare("abc", "abd"), Less);
    assert_eq!(compare("a1b", "a1c"), Less);
}

#[test]
fn digit_runs_of_different_length_compare_by_length_first() {
    // Same rule as the original comparator: the length decides before the digits do.
    assert_eq!(compare("a0001", "a10"), Greater);
}
