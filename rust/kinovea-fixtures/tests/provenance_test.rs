//! Every committed fixture must come from the same generator commit, otherwise
//! parity tests would compare against a moving target.

use kinovea_fixtures::{fixtures_root, manifests, Manifest};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn write_manifest(dir: &Path, commit: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join("manifest.json"),
        format!(r#"{{"commit":"{commit}","ffmpeg_version":"8.1.2","case_sha256":"00"}}"#),
    )
    .unwrap();
}

/// Returns the manifests whose commit differs from the most common one.
fn stale(entries: &[(std::path::PathBuf, Manifest)]) -> Vec<&Path> {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, m) in entries {
        *counts.entry(m.commit.as_str()).or_default() += 1;
    }
    let Some((majority, _)) = counts
        .iter()
        .max_by_key(|(c, n)| (**n, std::cmp::Reverse(**c)))
    else {
        return Vec::new();
    };
    entries
        .iter()
        .filter(|(_, m)| m.commit != *majority)
        .map(|(p, _)| p.as_path())
        .collect()
}

#[test]
fn committed_fixtures_share_one_commit() {
    let entries = manifests(&fixtures_root()).expect("fixtures readable");
    let bad = stale(&entries);
    assert!(bad.is_empty(), "fixtures from a different commit: {bad:?}");
}

#[test]
fn zero_fixtures_is_valid() {
    let dir = tempfile::tempdir().unwrap();
    assert!(stale(&manifests(dir.path()).unwrap()).is_empty());
}

#[test]
fn mixed_commits_are_reported() {
    let dir = tempfile::tempdir().unwrap();
    write_manifest(&dir.path().join("kva/a"), "aaa");
    write_manifest(&dir.path().join("kva/b"), "aaa");
    write_manifest(&dir.path().join("kinematics/c"), "bbb");
    let entries = manifests(dir.path()).unwrap();
    let bad = stale(&entries);
    assert_eq!(bad.len(), 1);
    assert!(bad[0].ends_with("kinematics/c/manifest.json"));
}
