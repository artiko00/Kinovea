use super::{assert_cell, json_value};

#[test]
fn json_value_reads_flat_keys() {
    let text = "{\n  \"fps\": 60,\n  \"can_filter\": true,\n  \"x_cutoff_index\": -1\n}\n";
    assert_eq!(json_value(text, "fps"), "60");
    assert_eq!(json_value(text, "can_filter"), "true");
    assert_eq!(json_value(text, "x_cutoff_index"), "-1");
}

#[test]
fn cells_compare_relatively_and_nan_must_coincide() {
    assert_cell(1_000_000.5, 1_000_000.0, "relative");
    assert_cell(f64::NAN, f64::NAN, "nan");
}

#[test]
#[should_panic(expected = "nan")]
fn a_number_against_nan_fails() {
    assert_cell(1.0, f64::NAN, "nan");
}
