//! Loads a render case's KVA the way `kva_roundtrip_test` does, and resolves oracle drawing keys.

use super::table::DrawingKey;
use kinovea_fixtures::fixtures_root;
use kinovea_metadata::{header_metadata, read_kva, Drawing, Metadata};
use std::path::PathBuf;

/// Repository root (the fixtures live at `<repo>/tests/fixtures`).
pub fn repo() -> PathBuf {
    fixtures_root().join("../..")
}

/// The metadata of a KVA given relative to the repository root.
pub fn metadata(kva: &str) -> Metadata {
    let xml = std::fs::read_to_string(repo().join(kva)).expect("kva");
    let xml = xml.trim_start_matches('\u{feff}');
    let mut m = header_metadata(xml);
    read_kva(&mut m, xml).expect("kva parses");
    m
}

/// The drawing an oracle key names: `kN` is keyframe N, `chronos` and `tracks` the detached lists.
/// Panics when the key does not exist, which would mean the table and the KVA disagree.
pub fn drawing<'m>(m: &'m Metadata, key: &DrawingKey) -> &'m Drawing {
    let list = match key.container.as_str() {
        "chronos" => &m.chronos,
        "tracks" => &m.tracks,
        k => {
            let n: usize = k.trim_start_matches('k').parse().expect("keyframe index");
            &m.keyframes[n].drawings
        }
    };
    &list[key.index]
}

#[cfg(test)]
#[path = "load_test.rs"]
mod load_test;
