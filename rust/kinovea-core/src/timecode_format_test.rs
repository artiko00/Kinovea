use crate::timecode_format::{timecode_type, TimecodeFormat, TimecodeType};

#[test]
fn classification_matches_original() {
    use TimecodeFormat::*;
    let numbers = [
        Frames,
        Milliseconds,
        Microseconds,
        HundredthOfMinutes,
        TenThousandthOfHours,
        Timestamps,
    ];
    for f in TimecodeFormat::ALL {
        let expected = if numbers.contains(&f) {
            TimecodeType::Number
        } else {
            TimecodeType::String
        };
        assert_eq!(timecode_type(f), expected, "{f}");
    }
}

#[test]
fn names_round_trip() {
    assert_eq!(TimecodeFormat::ALL.len(), 9);
    for f in TimecodeFormat::ALL {
        let parsed: TimecodeFormat = f.to_string().parse().unwrap();
        assert_eq!(parsed, f);
    }
    assert_eq!(TimecodeFormat::ClassicTime.to_string(), "ClassicTime");
    assert!("Unknown".parse::<TimecodeFormat>().is_err());
}
