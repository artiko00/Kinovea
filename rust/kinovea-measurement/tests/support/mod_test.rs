use super::{assert_f, Row};

#[test]
fn rows_resolve_columns_by_name() {
    let header = vec!["a".to_string(), "bx".to_string(), "by".to_string()];
    let cells = vec!["1.5".to_string(), "2".to_string(), "-3".to_string()];
    let row = Row {
        header: &header,
        cells: &cells,
    };
    assert_eq!(row.f("a"), 1.5);
    assert_eq!(row.point("bx", "by").y, -3.0);
    assert_eq!(row.text("a"), "1.5");
}

#[test]
#[should_panic(expected = "missing")]
fn unknown_column_panics() {
    let header = vec!["a".to_string()];
    let cells = vec!["1".to_string()];
    Row {
        header: &header,
        cells: &cells,
    }
    .f("zzz");
}

#[test]
fn nan_expectations() {
    assert_f(f64::NAN, f64::NAN, 0.0, "nan");
    assert_f(1.0, 1.0001, 1e-3, "close");
}
