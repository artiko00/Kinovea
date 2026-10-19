//! The render cases with hit tables: `hitScales`, `hitStep`, `hitTimestamp` and `kva` are read from
//! `tests/cases/render/<name>.json`. The JSON is scanned as text: the test crate has no JSON parser.

use super::load::repo;

/// One case: its KVA, the viewport scales probed, the grid step and the time of detached drawings.
#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub name: &'static str,
    pub kva: String,
    pub scales: Vec<f64>,
    pub step: i32,
    pub detached_timestamp: i64,
}

/// The cases whose oracle tables are compared.
pub const HIT_CASES: [&str; 4] = ["geometry", "simple", "complete", "all-drawings"];

pub fn load(name: &'static str) -> Case {
    let path = repo().join(format!("tests/cases/render/{name}.json"));
    let text = std::fs::read_to_string(path).expect("case json");
    Case {
        name,
        kva: string_value(&text, "kva"),
        scales: numbers(raw(&text, "hitScales")).into_iter().collect(),
        step: numbers(raw(&text, "hitStep"))[0] as i32,
        detached_timestamp: numbers(raw(&text, "hitTimestamp"))[0] as i64,
    }
}

/// The text after `"key":`, up to the end of an array, string, or scalar.
fn raw<'a>(text: &'a str, key: &str) -> &'a str {
    let start = text.find(&format!("\"{key}\":")).expect("key") + key.len() + 3;
    let rest = text[start..].trim_start();
    let end = match rest.as_bytes()[0] {
        b'[' => rest.find(']').map(|i| i + 1),
        b'"' => rest[1..].find('"').map(|i| i + 2),
        _ => rest.find([',', '}', '\n']),
    };
    &rest[..end.unwrap_or(rest.len())]
}

fn string_value(text: &str, key: &str) -> String {
    raw(text, key).trim_matches('"').to_owned()
}

fn numbers(text: &str) -> Vec<f64> {
    text.trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|t| t.trim().parse().expect("number"))
        .collect()
}

#[cfg(test)]
#[path = "cases_test.rs"]
mod cases_test;
