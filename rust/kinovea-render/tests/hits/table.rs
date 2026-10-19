//! `hits.csv` of one case: the recorded `HitTest` results, grouped by drawing and scale.

use std::collections::HashMap;

/// Where a drawing lives in the oracle (`container`, `index`) and its element name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DrawingKey {
    pub container: String,
    pub index: usize,
    pub element: String,
}

/// Recorded results of one drawing at one scale. Probes not listed are misses (-1).
/// `exception` is set when the original threw: the drawing is not compared at that scale.
#[derive(Debug, Default, PartialEq)]
pub struct Expected {
    pub hits: HashMap<(i32, i32), i32>,
    pub exception: bool,
}

/// One data row of the CSV.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub key: DrawingKey,
    pub timestamp: i64,
    pub scale: f64,
    pub x: i32,
    pub y: i32,
    /// `None` for the `exception` marker.
    pub result: Option<i32>,
}

/// Parses `container,index,element,timestamp,scale,x,y,result` rows; the header line is skipped.
pub fn parse(text: &str) -> Vec<Row> {
    text.lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(parse_row)
        .collect()
}

fn parse_row(line: &str) -> Row {
    let f: Vec<&str> = line.split(',').collect();
    Row {
        key: DrawingKey {
            container: f[0].to_owned(),
            index: f[1].parse().expect("index"),
            element: f[2].to_owned(),
        },
        timestamp: f[3].parse().expect("timestamp"),
        scale: f[4].parse().expect("scale"),
        x: f[5].parse().expect("x"),
        y: f[6].parse().expect("y"),
        result: f[7].parse().ok(),
    }
}

/// The recorded results of `key` at `scale`.
pub fn expected(rows: &[Row], key: &DrawingKey, scale: f64) -> Expected {
    let mut e = Expected::default();
    for r in rows.iter().filter(|r| r.key == *key && r.scale == scale) {
        match r.result {
            Some(code) => {
                e.hits.insert((r.x, r.y), code);
            }
            None => e.exception = true,
        }
    }
    e
}

#[cfg(test)]
#[path = "table_test.rs"]
mod table_test;
