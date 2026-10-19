use crate::srt_timecode::parse_timecode;

#[test]
fn timecode_accepts_comma_and_dot_and_truncates_to_milliseconds() {
    assert_eq!(parse_timecode("00:00:01,250"), 1250);
    assert_eq!(parse_timecode("00:00:01.250"), 1250);
    assert_eq!(parse_timecode("00:00:01.1234"), 1123);
    assert_eq!(parse_timecode("1:02:03,5"), 3_723_500);
}

#[test]
fn invalid_timecode_is_minus_one() {
    assert_eq!(parse_timecode("00:61:00,000"), -1);
    assert_eq!(parse_timecode("nothing"), -1);
}
