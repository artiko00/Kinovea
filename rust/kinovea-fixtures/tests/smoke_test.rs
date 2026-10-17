//! Every committed fixture must match its input case byte for byte (via the manifest checksum)
//! and the fixtures directory must be readable offline.

use kinovea_fixtures::{fixtures_root, manifests, sha256_hex};
use std::path::Path;

/// `tests/fixtures/<area>/<case>/manifest.json` -> `tests/cases/<area>/<case>.json`
fn case_for(manifest: &Path, root: &Path) -> std::path::PathBuf {
    let rel = manifest
        .parent()
        .and_then(|p| p.strip_prefix(root).ok())
        .expect("manifest inside fixtures root");
    root.parent()
        .expect("tests dir")
        .join("cases")
        .join(rel)
        .with_extension("json")
}

#[test]
fn committed_fixtures_match_their_cases() {
    let root = fixtures_root();
    let entries = manifests(&root).expect("fixtures readable");
    assert!(
        !entries.is_empty(),
        "no fixtures under {}: run tools/oracle/fixtures.sh",
        root.display()
    );
    for (path, manifest) in &entries {
        let case = case_for(path, &root);
        let bytes = std::fs::read(&case)
            .unwrap_or_else(|e| panic!("case {} for {}: {e}", case.display(), path.display()));
        assert_eq!(
            sha256_hex(&bytes),
            manifest.case_sha256,
            "fixture {} was generated from a different version of {}",
            path.display(),
            case.display()
        );
        assert_ne!(manifest.commit, "unknown", "{}", path.display());
        assert_ne!(manifest.ffmpeg_version, "unknown", "{}", path.display());
    }
}

#[test]
fn every_case_has_a_fixture() {
    let root = fixtures_root();
    let cases_root = root.parent().unwrap().join("cases");
    let mut missing = Vec::new();
    let mut stack = vec![cases_root.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "json") {
                let rel = path.strip_prefix(&cases_root).unwrap().with_extension("");
                if !root.join(rel).join("manifest.json").exists() {
                    missing.push(path);
                }
            }
        }
    }
    assert!(missing.is_empty(), "cases without fixture: {missing:?}");
}
