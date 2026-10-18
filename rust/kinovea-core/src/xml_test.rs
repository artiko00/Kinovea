use crate::video_section::VideoSection;
use crate::xml::*;

#[test]
fn floats_parse_with_fallback_and_write_with_seven_significant_digits() {
    assert_eq!(parse_float("1.5"), 1.5);
    assert_eq!(parse_float(" -2E3 "), -2000.0);
    assert_eq!(parse_float("abc"), 0.0);
    assert_eq!(write_float(0.1), "0.1");
    assert_eq!(write_float(100.0), "100");
    assert_eq!(write_float(1234.5678), "1234.568");
    assert_eq!(write_float(-0.000_012_5), "-1.25E-05");
    assert_eq!(format_f32_invariant(960.0), "960");
}

#[test]
fn points_and_sizes_round_trip() {
    assert_eq!(parse_point_f("12.5;-3"), (12.5, -3.0));
    assert_eq!(parse_point_f("12.5"), (0.0, 0.0));
    assert_eq!(parse_point_f("x;3"), (0.0, 0.0));
    assert_eq!(parse_size_f("4;3"), (4.0, 3.0));
    assert_eq!(parse_size("1920;1080"), (1920, 1080));
    assert_eq!(parse_size("1920.5;1080"), (0, 0));
    assert_eq!(write_point_f(12.5, -3.0), "12.5;-3");
    assert_eq!(write_size_f(4.0, 3.0), "4;3");
    assert_eq!(write_size(1920, 1080), "1920;1080");
    assert_eq!(write_rectangle_f(1.0, 2.0, 3.5, 4.0), "1;2;3.5;4");
}

#[test]
fn booleans_lists_and_references() {
    assert!(
        parse_boolean("true") && parse_boolean("True") && parse_boolean("1") && parse_boolean("")
    );
    assert!(!parse_boolean("false") && !parse_boolean("False") && !parse_boolean("0"));
    assert_eq!(write_boolean(true), "true");
    assert_eq!(write_boolean(false), "false");
    assert_eq!(parse_int_list("1;2;3"), vec![1, 2, 3]);
    assert_eq!(parse_int_list("1;x;3"), vec![1]);
    assert_eq!(parse_point_reference("12"), 12);
    assert_eq!(parse_point_reference("$3"), -4);
    assert_eq!(parse_point_reference("$x"), 0);
    assert_eq!(parse_point_reference("nope"), 0);
}

#[test]
fn video_sections_and_timestamps() {
    assert_eq!(write_video_section(&VideoSection::new(10, 20)), "10;20");
    assert_eq!(
        write_video_section(&VideoSection::new(i64::MAX, i64::MAX)),
        "-1;-1"
    );
    assert_eq!(write_timestamp(-7), "-7");
}
