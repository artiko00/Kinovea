use super::*;

#[test]
fn reads_the_probe_settings_of_the_geometry_case() {
    let c = load("geometry");
    assert_eq!(c.kva, "tests/cases/render/inputs/geometry.kva");
    assert_eq!(c.scales, vec![0.5, 1.0, 2.0]);
    assert_eq!(c.step, 2);
    assert_eq!(c.detached_timestamp, 0);
}

#[test]
fn reads_the_detached_timestamp_of_all_drawings() {
    let c = load("all-drawings");
    assert_eq!(c.step, 4);
    assert_eq!(c.detached_timestamp, 1000);
}

#[test]
fn missing_step_and_timestamp_take_the_oracle_defaults() {
    let c = parse("sample", r#"{"inputs":{"kva":"a.kva","hitScales":[1]}}"#);
    assert_eq!(c.step, 4);
    assert_eq!(c.detached_timestamp, 0);
}

#[test]
fn a_key_quoted_inside_another_value_is_not_read() {
    let text = r#"{"inputs":{"note":"\"hitStep\": 9","kva":"a.kva","hitScales":[1],"hitStep":2}}"#;
    assert_eq!(parse("sample", text).step, 2);
}
