use crate::timecode::{ms_to_timecode, timestring, TimecodeFormat};

#[test]
fn precision_follows_framerate_magnitude() {
    let classic = TimecodeFormat::ClassicTime;
    assert_eq!(
        timestring(25.0, 37, 1500.0, 37, 900.0, 900.0, classic, false),
        "1.50"
    );
    assert_eq!(
        timestring(1000.0, 1500, 1500.0, 1500, 36_000.0, 36_000.0, classic, false),
        "1.500"
    );
    // 1 fps gives precision 0 and falls into the centisecond branch.
    assert_eq!(
        timestring(1.0, 1, 1500.0, 1, 36.0, 36.0, classic, false),
        "1.50"
    );
}

#[test]
fn centisecond_carry() {
    assert_eq!(ms_to_timecode(59_995.0, 2), "1:00.00");
    assert_eq!(ms_to_timecode(999.0, 2), "1.00");
    assert_eq!(ms_to_timecode(-1.0, 2), "- 0.00");
    assert_eq!(ms_to_timecode(60_999.0, 3), "1:00.999");
    assert_eq!(ms_to_timecode(-36_000_000.0, 3), "- 10:00:00.000");
}
