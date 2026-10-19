//! Replays every `videocache` case on the Rust containers and checks each row against the
//! original's `steps.csv`, exactly.

mod videocache_replay;

use kinovea_fixtures::fixtures_root;
use serde_json::Value;
use videocache_replay::{replay, Row};

/// The `tests/cases/videocache` directory: one `<name>.json` per case.
fn cases_dir() -> std::path::PathBuf {
    fixtures_root().join("../cases/videocache")
}

/// The case names, from the `.json` files of `cases_dir`.
fn case_names() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(cases_dir())
        .expect("videocache cases")
        .map(|entry| entry.expect("case entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .map(|path| {
            path.file_stem()
                .expect("case name")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}

/// The fixture directories under `tests/fixtures/videocache`.
fn fixture_names() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(fixtures_root().join("videocache"))
        .expect("videocache fixtures")
        .map(|entry| {
            entry
                .expect("fixture entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}

/// The `inputs` object of `tests/cases/videocache/<name>.json`.
fn load_inputs(name: &str) -> Value {
    let path = cases_dir().join(format!("{name}.json"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let case: Value = serde_json::from_str(&text).expect("case json");
    case["inputs"].clone()
}

fn load_steps(name: &str) -> String {
    let path = fixtures_root()
        .join("videocache")
        .join(name)
        .join("steps.csv");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Describes every row that differs from `steps_csv`. Empty when the replay matches exactly.
fn compare(name: &str, steps_csv: &str, got: &[Row]) -> Vec<String> {
    let want: Vec<Vec<String>> = steps_csv
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',').map(str::to_owned).collect())
        .collect();
    if want.len() != got.len() {
        return vec![format!(
            "{name}: {} rows replayed, {} expected",
            got.len(),
            want.len()
        )];
    }
    got.iter()
        .zip(&want)
        .enumerate()
        .filter(|(_, (g, w))| g.as_slice() != w.as_slice())
        .map(|(i, (g, w))| format!("{name} row {i} ({}): got {g:?}, want {w:?}", w[0]))
        .collect()
}

#[test]
fn every_case_has_fixtures_and_no_fixture_is_orphaned() {
    assert_eq!(fixture_names(), case_names());
}

#[test]
fn every_case_replays_to_the_original_rows() {
    for name in case_names() {
        let got = replay(&load_inputs(&name));
        let problems = compare(&name, &load_steps(&name), &got);
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }
}

#[test]
fn an_edited_expected_row_is_reported() {
    let edited = load_steps("cache-basic").replacen("|[0;30]", "|[0;31]", 1);
    let got = replay(&load_inputs("cache-basic"));
    let problems = compare("cache-basic", &edited, &got);
    assert_eq!(problems.len(), 1, "{problems:?}");
    assert!(problems[0].contains("row 5 (add 30)"), "{problems:?}");
}
