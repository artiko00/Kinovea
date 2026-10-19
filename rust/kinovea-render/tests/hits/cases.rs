//! The render cases with hit tables: `hitScales`, `hitStep`, `hitTimestamp` and `kva` are read from
//! the `inputs` of `tests/cases/render/<name>.json`. `hitStep` and `hitTimestamp` default to 4 and 0
//! when absent, as `RenderArea.Hits.cs` does.

use super::load::repo;
use serde_json::Value;

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
    parse(name, &text)
}

/// The case described by the JSON `text`.
pub fn parse(name: &'static str, text: &str) -> Case {
    let json: Value = serde_json::from_str(text).expect("case json parses");
    let inputs = &json["inputs"];
    Case {
        name,
        kva: inputs["kva"].as_str().expect("kva").to_owned(),
        scales: inputs["hitScales"]
            .as_array()
            .expect("hitScales")
            .iter()
            .map(|v| v.as_f64().expect("scale"))
            .collect(),
        step: inputs["hitStep"].as_i64().unwrap_or(4) as i32,
        detached_timestamp: inputs["hitTimestamp"].as_i64().unwrap_or(0),
    }
}

#[cfg(test)]
#[path = "cases_test.rs"]
mod cases_test;
