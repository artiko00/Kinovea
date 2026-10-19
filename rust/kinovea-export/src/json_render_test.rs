use crate::json_render::{render, Json};

#[test]
fn nested_containers_use_two_space_indentation() {
    let v = Json::Object(vec![
        (
            "a".into(),
            Json::Array(vec![Json::Object(vec![("x".into(), Json::Int(1))])]),
        ),
        ("b".into(), Json::Array(vec![])),
    ]);
    assert_eq!(
        render(&v),
        "{\n  \"a\": [\n    {\n      \"x\": 1\n    }\n  ],\n  \"b\": []\n}"
    );
}

#[test]
fn empty_object_is_written_on_one_line() {
    assert_eq!(render(&Json::Object(vec![])), "{}");
}

#[test]
fn floats_keep_a_decimal_point_like_the_original_writer() {
    assert_eq!(render(&Json::Float(25.0)), "25.0");
    assert_eq!(render(&Json::Float(0.52)), "0.52");
    assert_eq!(render(&Json::Float(-90.0)), "-90.0");
}

#[test]
fn non_finite_floats_are_quoted() {
    assert_eq!(render(&Json::Float(f32::NAN)), "\"NaN\"");
    assert_eq!(render(&Json::Float(f32::INFINITY)), "\"Infinity\"");
    assert_eq!(render(&Json::Float(f32::NEG_INFINITY)), "\"-Infinity\"");
}

#[test]
fn strings_are_escaped() {
    assert_eq!(render(&Json::String("a\"b\nc".into())), "\"a\\\"b\\nc\"");
}
