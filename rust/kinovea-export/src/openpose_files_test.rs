use crate::import_test::repo;
use crate::openpose_files::{frame_files, frame_number, keypoints_prefix};

#[test]
fn prefix_is_the_name_before_the_frame_token() {
    assert_eq!(
        keypoints_prefix("clip_000000000000_keypoints.json"),
        Some("clip")
    );
    assert_eq!(
        keypoints_prefix("my_clip_000000000001_keypoints.json"),
        Some("my_clip")
    );
}

#[test]
fn names_without_the_keypoints_suffix_have_no_prefix() {
    assert_eq!(keypoints_prefix("clip_keypoints.json"), None);
    assert_eq!(keypoints_prefix("clip_000000000000_pose.json"), None);
}

#[test]
fn frame_number_is_the_token_before_the_suffix() {
    assert_eq!(frame_number("clip_000000000012_keypoints.json"), Some(12));
    assert_eq!(frame_number("clip_abc_keypoints.json"), None);
}

#[test]
fn frame_files_match_the_prefix_and_sort_by_frame() {
    let folder = repo().join("tests/data/import/openpose");
    let names = frame_files(&folder, "clip").unwrap();
    assert_eq!(
        names,
        vec![
            "clip_000000000000_keypoints.json",
            "clip_000000000001_keypoints.json",
            "clip_000000000002_keypoints.json"
        ]
    );
    assert!(frame_files(&folder, "other").unwrap().is_empty());
}
