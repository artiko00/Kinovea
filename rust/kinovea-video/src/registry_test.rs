//! Tests for `registry.rs`: extension lookup, wildcard fallback and the most recent supported file.

use crate::reader::VideoReader;
use crate::registry::{parse_extensions, ReaderFactory, ReaderRegistry};
use crate::results::OpenVideoResult;
use crate::synthetic::SyntheticReader;
use crate::temp_dir::TempDir;
use kinovea_geometry::Size;
use std::fs::File;
use std::path::Path;
use std::time::{Duration, SystemTime};

/// A factory whose readers report `fps` once opened, so a test can tell them apart.
fn factory(fps: f64) -> ReaderFactory {
    Box::new(move || -> Box<dyn VideoReader> {
        Box::new(SyntheticReader::new(Size::new(4, 2), fps, 5, 1))
    })
}

fn opened_fps(registry: &ReaderRegistry, extension: &str) -> f64 {
    let mut reader = registry.reader_for(extension).expect("a reader");
    assert_eq!(reader.open("clip"), OpenVideoResult::Success);
    reader.info().frames_per_second
}

fn touch(path: &Path, seconds_since_epoch: u64) {
    let file = File::create(path).expect("create file");
    let modified = SystemTime::UNIX_EPOCH + Duration::from_secs(seconds_since_epoch);
    file.set_modified(modified).expect("set modification time");
}

#[test]
fn lookup_is_case_insensitive() {
    let mut registry = ReaderRegistry::default();
    registry.register(".gif", factory(10.0));
    assert_eq!(opened_fps(&registry, ".GIF"), 10.0);
}

#[test]
fn unknown_extension_falls_back_to_the_wildcard() {
    let mut registry = ReaderRegistry::default();
    registry.register(".gif", factory(10.0));
    registry.register("*", factory(25.0));
    assert_eq!(opened_fps(&registry, ".xyz"), 25.0);
    assert_eq!(opened_fps(&registry, ".gif"), 10.0);
}

#[test]
fn unknown_extension_without_wildcard_has_no_reader() {
    let mut registry = ReaderRegistry::default();
    registry.register(".gif", factory(10.0));
    assert!(registry.reader_for(".xyz").is_none());
}

#[test]
fn first_registration_of_an_extension_wins() {
    let mut registry = ReaderRegistry::default();
    registry.register(".gif", factory(10.0));
    registry.register(".GIF;.apng", factory(30.0));
    assert_eq!(opened_fps(&registry, ".gif"), 10.0);
    assert_eq!(opened_fps(&registry, ".apng"), 30.0);
}

#[test]
fn extensions_split_on_semicolons_tabs_and_spaces() {
    assert_eq!(
        parse_extensions(".avi;.MOV\t.gif  ;"),
        vec![".avi", ".mov", ".gif"]
    );
}

#[test]
fn is_supported_counts_the_wildcard_key() {
    // IsSupported is ContainsKey: the wildcard reader is a key too.
    let mut registry = ReaderRegistry::default();
    registry.register("*", factory(25.0));
    registry.register(".gif", factory(10.0));
    assert!(registry.is_supported(".GIF"));
    assert!(registry.is_supported("*"));
    assert!(!registry.is_supported(".xyz"));
    assert!(!registry.is_supported("  "));
}

#[test]
fn image_sequence_reader_is_the_wildcard() {
    let mut registry = ReaderRegistry::default();
    registry.register(".gif", factory(10.0));
    assert!(registry.image_sequence_reader().is_none());
    registry.register("*", factory(25.0));
    let mut reader = registry.image_sequence_reader().expect("wildcard reader");
    assert_eq!(reader.open("frames"), OpenVideoResult::Success);
    assert_eq!(reader.info().frames_per_second, 25.0);
}

#[test]
fn most_recent_supported_file_matches_the_pattern() {
    let dir = TempDir::new("recent");
    let older = dir.join("a.avi");
    let newer = dir.join("b.avi");
    touch(&older, 1_000);
    touch(&newer, 2_000);
    touch(&dir.join("c.txt"), 3_000);
    let mut registry = ReaderRegistry::default();
    registry.register(".avi", factory(25.0));

    let everything = dir.join("*");
    assert_eq!(
        registry.most_recent_supported(&everything.to_string_lossy()),
        Some(newer)
    );
    let first_only = dir.join("a*");
    assert_eq!(
        registry.most_recent_supported(&first_only.to_string_lossy()),
        Some(older)
    );
}

#[test]
fn most_recent_breaks_a_tie_by_name_order() {
    // GetFiles promises no order: on equal write times the first name wins, as OrderByDescending does.
    let dir = TempDir::new("tie");
    touch(&dir.join("b.avi"), 1_000);
    touch(&dir.join("a.avi"), 1_000);
    let mut registry = ReaderRegistry::default();
    registry.register(".avi", factory(25.0));
    assert_eq!(
        registry.most_recent_supported(&dir.join("*").to_string_lossy()),
        Some(dir.join("a.avi"))
    );
}

#[test]
fn most_recent_returns_an_absolute_path_for_a_relative_pattern() {
    // FileInfo.FullName is absolute. Cargo runs tests from the crate directory.
    let mut registry = ReaderRegistry::default();
    registry.register(".rs", factory(25.0));
    let cwd = std::env::current_dir().expect("current dir");
    let found = registry.most_recent_supported("src/lib.rs");
    assert_eq!(found, Some(cwd.join("src/lib.rs")));
}

#[test]
fn missing_directory_has_no_recent_file() {
    let registry = ReaderRegistry::default();
    assert_eq!(
        registry.most_recent_supported("/nonexistent-kinovea-dir/*.avi"),
        None
    );
}
