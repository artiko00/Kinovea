use crate::case::load_case;
use crate::collect::collect_measured_data;
use crate::json::write_json;
use crate::rtf::rtf_to_text;
use kinovea_fixtures::{approx_eq, fixtures_root};
use serde_json::Value;

/// Compares the writer output with the oracle `measured.json` of `name`:
/// the same key sequence in document order, and the same values with numbers at 1e-4.
fn check(name: &str) {
    let loaded = load_case(name);
    let md = collect_measured_data(&loaded.metadata, &loaded.prefs);
    let text = write_json(&md, rtf_to_text);
    let fixture = std::fs::read_to_string(
        fixtures_root()
            .join("export")
            .join(name)
            .join("measured.json"),
    )
    .unwrap();
    assert_eq!(keys(&text), keys(&fixture), "{name}: key order");

    let got: Value = serde_json::from_str(&text).unwrap();
    let want: Value = serde_json::from_str(&fixture).unwrap();
    assert_eq!(
        got["metadata"]["producer"], want["metadata"]["producer"],
        "{name}: producer"
    );
    if let Some(diff) = first_difference(&got, &want, String::new()) {
        panic!("{name}: {diff}");
    }
}

/// Object keys in document order, read from the text so that order and duplicates are kept.
fn keys(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'"' {
            i += 1;
            continue;
        }
        let start = i + 1;
        i = start;
        while bytes[i] != b'"' {
            i += if bytes[i] == b'\\' { 2 } else { 1 };
        }
        let token = &text[start..i];
        i += 1;
        if text[i..].trim_start().starts_with(':') {
            out.push(token.to_owned());
        }
    }
    out
}

fn first_difference(a: &Value, b: &Value, path: String) -> Option<String> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            let (x, y) = (x.as_f64()?, y.as_f64()?);
            (!approx_eq(x, y, 1e-4)).then(|| format!("{path}: {x} != {y}"))
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => x
            .iter()
            .zip(y)
            .enumerate()
            .find_map(|(i, (p, q))| first_difference(p, q, format!("{path}[{i}]"))),
        (Value::Object(x), Value::Object(y)) if x.len() == y.len() => {
            x.iter().find_map(|(k, v)| match y.get(k) {
                Some(w) => first_difference(v, w, format!("{path}.{k}")),
                None => Some(format!("{path}.{k}: missing")),
            })
        }
        _ => (a != b).then(|| format!("{path}: {a} != {b}")),
    }
}

#[test]
fn minimal_sample_matches_the_oracle() {
    check("minimal");
}

#[test]
fn simple_sample_matches_the_oracle() {
    check("simple");
}

#[test]
fn complete_sample_matches_the_oracle() {
    check("complete");
}

#[test]
fn keys_keep_document_order_and_duplicates() {
    assert_eq!(
        keys(r#"{"a": 1, "b": {"a": "x:y"}, "a": 2}"#),
        ["a", "b", "a", "a"]
    );
}
