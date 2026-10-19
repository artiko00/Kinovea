use super::replay;
use serde_json::json;

#[test]
fn each_op_yields_a_row_with_five_columns_echoing_the_op() {
    let out = replay(&json!({ "ops": ["acquireNext", "add 3"] }));
    assert_eq!(out.len(), 2);
    assert_eq!(
        out[0],
        ["acquireNext", "False", "-1", "-1|", ""].map(String::from)
    );
    assert_eq!(out[1][0], "add 3");
    assert_eq!(out[1][4], "3");
}
