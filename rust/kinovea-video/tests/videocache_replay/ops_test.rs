use super::{replay, Row};
use serde_json::json;

fn rows(inputs: serde_json::Value) -> Vec<Row> {
    replay(&inputs)
}

#[test]
fn blocked_add_reports_blocked_and_leaves_the_buffer_unchanged() {
    let out = rows(json!({
        "container": "prebuffer", "capacity": 1, "ops": ["add 0", "add 1"]
    }));
    assert_eq!(out[1][1], "Blocked");
    assert_eq!(out[1][4], "0");
    assert_eq!(out[1][3], "1|[0;0]");
}

#[test]
fn shutdown_on_the_dense_cache_reports_the_original_exception_and_keeps_state() {
    let out = rows(json!({ "ops": ["add 0", "shutdown"] }));
    assert_eq!(out[1][1], "exception:NotImplementedException");
    assert_eq!(out[1][3], "1|[0;0]");
}

#[test]
fn clear_on_the_prebuffer_reports_the_original_exception_and_keeps_state() {
    // PreBuffer2.Clear throws NotImplementedException (PreBuffer2.cs:522-525).
    let out = rows(json!({
        "container": "prebuffer", "ops": ["add 0", "clear"]
    }));
    assert_eq!(out[1][1], "exception:NotImplementedException");
    assert_eq!(out[1][3], "1|[0;0]");
    assert_eq!(out[1][4], "0");
}

#[test]
fn clear_on_the_dense_cache_empties_it() {
    let out = rows(json!({ "ops": ["add 0", "clear"] }));
    assert_eq!(out[1][1], "");
    assert_eq!(out[1][4], "");
}

#[test]
fn prebuffer_only_ops_fail_on_the_dense_cache_like_a_null_reference() {
    let out = rows(json!({ "ops": ["relate 5", "purge"] }));
    assert_eq!(out[0][1], "exception:NullReferenceException");
    assert_eq!(out[1][1], "exception:NullReferenceException");
}

#[test]
fn unknown_op_reports_argument_exception() {
    let out = rows(json!({ "ops": ["explode 1"] }));
    assert_eq!(out[0][1], "exception:ArgumentException");
}

#[test]
fn add_without_previous_uses_the_frame_step() {
    let out = rows(json!({ "container": "prebuffer", "tsPerFrame": 10, "ops": ["add 20"] }));
    assert_eq!(out[0][1], "Added");
    assert_eq!(out[0][3], "1|[20;20]");
}
