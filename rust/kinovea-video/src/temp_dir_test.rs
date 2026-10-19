//! Tests for `temp_dir.rs`: unique directories that are removed on drop.

use crate::temp_dir::TempDir;

#[test]
fn directory_exists_until_dropped() {
    let dir = TempDir::new("guard");
    let path = dir.path().to_path_buf();
    assert!(path.is_dir());
    drop(dir);
    assert!(!path.exists());
}

#[test]
fn directories_with_the_same_name_differ() {
    let first = TempDir::new("same");
    let second = TempDir::new("same");
    assert_ne!(first.path(), second.path());
}
