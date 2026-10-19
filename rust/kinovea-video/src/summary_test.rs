//! Tests for `summary.rs`: thumbnail selection, thumbnail size and the companion KVA lookup.

use crate::summary::{has_companion_kva, thumbnail_indices, thumbnail_size};
use crate::temp_dir::TempDir;
use kinovea_geometry::Size;
use std::fs;

#[test]
fn thumbnails_are_spaced_by_the_ceiling_step() {
    assert_eq!(thumbnail_indices(10, 4), vec![0, 3, 6, 9]);
    assert_eq!(thumbnail_indices(250, 3), vec![0, 84, 168]);
    assert_eq!(thumbnail_indices(3, 5), vec![0, 1, 2]);
    assert_eq!(thumbnail_indices(10, 3), vec![0, 4, 8]);
    assert_eq!(thumbnail_indices(7, 2), vec![0, 4]);
    assert_eq!(thumbnail_indices(1, 1), vec![0]);
}

#[test]
fn no_thumbnails_when_none_requested() {
    assert!(thumbnail_indices(10, 0).is_empty());
}

#[test]
fn thumbnail_fits_the_maximum_size() {
    assert_eq!(
        thumbnail_size(Size::new(320, 240), Size::new(100, 100)),
        Size::new(100, 75)
    );
}

#[test]
fn companion_kva_is_found_next_to_the_video() {
    let dir = TempDir::new("kva");
    fs::write(dir.join("clip.avi"), b"").expect("write video");
    fs::write(dir.join("clip.kva"), b"").expect("write kva");
    fs::write(dir.join("other.avi"), b"").expect("write video");
    assert!(has_companion_kva(&dir.join("clip.avi")));
    assert!(!has_companion_kva(&dir.join("other.avi")));
}
