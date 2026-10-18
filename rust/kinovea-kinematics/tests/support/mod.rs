//! Fixture readers shared by the kinematics parity tests.

// Each integration test compiles this module separately and uses a subset of it.
#![allow(dead_code)]

mod mod_test;

use kinovea_fixtures::fixtures_root;
use kinovea_kinematics::Kinematics;
use std::path::PathBuf;

pub fn case_dir(name: &str) -> PathBuf {
    fixtures_root().join("kinematics").join(name)
}

/// `linear.csv` or `angular.csv` as header names and parsed rows.
pub fn series(name: &str, file: &str) -> (Vec<Kinematics>, Vec<Vec<f64>>) {
    let path = case_dir(name).join(file);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut lines = text.lines().filter(|l| !l.is_empty());
    let header: Vec<Kinematics> = lines
        .next()
        .unwrap()
        .split(',')
        .skip(1)
        .map(|h| h.parse().unwrap())
        .collect();
    let rows = lines
        .map(|l| {
            l.split(',')
                .skip(1)
                .map(|v| v.parse::<f64>().unwrap())
                .collect()
        })
        .collect();
    (header, rows)
}

/// Column of a parsed table.
pub fn column(header: &[Kinematics], rows: &[Vec<f64>], key: Kinematics) -> Vec<f64> {
    let i = header
        .iter()
        .position(|k| *k == key)
        .unwrap_or_else(|| panic!("{key} missing"));
    rows.iter().map(|r| r[i]).collect()
}

/// Numeric or boolean value of a flat JSON key (`"fps": 60`, `"can_filter": true`, ...).
pub fn json_value(text: &str, key: &str) -> String {
    let pattern = format!("\"{key}\":");
    let start = text
        .find(&pattern)
        .unwrap_or_else(|| panic!("{key} missing"))
        + pattern.len();
    text[start..]
        .trim_start()
        .chars()
        .take_while(|c| !matches!(c, ',' | '}' | '\n'))
        .collect::<String>()
        .trim()
        .to_owned()
}

/// Flag from a case file (`tests/cases/kinematics/<name>.json`), default `true`.
pub fn case_flag(name: &str, key: &str) -> bool {
    let path = fixtures_root()
        .join("../cases/kinematics")
        .join(format!("{name}.json"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    !text.contains(&format!("\"{key}\": false"))
}

pub fn filtering_json(name: &str) -> String {
    std::fs::read_to_string(case_dir(name).join("filtering.json")).unwrap()
}

/// Relative comparison with a floor of 1 for values near zero; NaN must coincide.
pub fn assert_cell(got: f64, want: f64, ctx: &str) {
    if want.is_nan() || got.is_nan() {
        assert!(
            want.is_nan() && got.is_nan(),
            "{ctx}: got {got}, want {want}"
        );
        return;
    }
    let tol = 1e-6 * want.abs().max(1.0);
    assert!(
        kinovea_fixtures::approx_eq(got, want, tol),
        "{ctx}: got {got}, want {want}"
    );
}
