//! Loads a render case's KVA the way `kva_roundtrip_test` does.

use kinovea_fixtures::fixtures_root;
use kinovea_metadata::{load_kva, Metadata};
use std::path::PathBuf;

/// Repository root (the fixtures live at `<repo>/tests/fixtures`).
pub fn repo() -> PathBuf {
    fixtures_root().join("../..")
}

/// The metadata of a KVA given relative to the repository root.
pub fn metadata(kva: &str) -> Metadata {
    let xml = std::fs::read_to_string(repo().join(kva)).expect("kva");
    let xml = xml.trim_start_matches('\u{feff}');
    load_kva(xml).expect("kva parses")
}

#[cfg(test)]
#[path = "load_test.rs"]
mod load_test;
