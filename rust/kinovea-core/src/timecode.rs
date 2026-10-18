//! Time-to-text rendering (port of `TimeHelper.cs`).

pub use crate::timecode_format::{
    timecode_type, TimecodeFormat, TimecodeType, UnknownTimecodeFormat,
};

/// `(int)Math.Round(value)` as .NET x64 evaluates it: banker's rounding, then a
/// double-to-int conversion that yields `int.MinValue` when out of range.
fn round_to_int32(value: f64) -> i32 {
    let rounded = value.round_ties_even();
    if (-2_147_483_648.0..2_147_483_648.0).contains(&rounded) {
        rounded as i32
    } else {
        i32::MIN
    }
}

/// Renders a time span as text (`TimeHelper.GetTimestring`).
///
/// `framerate` drives the decimal precision, `frames`/`milliseconds`/`timestamps`
/// describe the span, `duration_timestamps` and `total_frames` describe the
/// enclosing segment (for `Normalized`), `symbol` appends the unit suffix.
#[allow(clippy::too_many_arguments)]
pub fn timestring(
    framerate: f64,
    frames: i64,
    milliseconds: f64,
    timestamps: i64,
    duration_timestamps: f64,
    total_frames: f64,
    format: TimecodeFormat,
    symbol: bool,
) -> String {
    let precision = framerate.log10().ceil() as i32;
    match format {
        TimecodeFormat::ClassicTime => ms_to_timecode(milliseconds, precision),
        TimecodeFormat::Frames => frames.to_string(),
        TimecodeFormat::Milliseconds => {
            let v = round_to_int32(milliseconds).to_string();
            if symbol {
                v + " ms"
            } else {
                v
            }
        }
        TimecodeFormat::Microseconds => {
            let v = round_to_int32(milliseconds * 1000.0).to_string();
            if symbol {
                v + " µs"
            } else {
                v
            }
        }
        TimecodeFormat::TenThousandthOfHours => split_hundredths(milliseconds / 360.0),
        TimecodeFormat::HundredthOfMinutes => split_hundredths(milliseconds / 600.0),
        TimecodeFormat::TimeAndFrames => {
            format!("{} ({frames})", ms_to_timecode(milliseconds, precision))
        }
        TimecodeFormat::Normalized => {
            let magnitude = total_frames.log10().ceil().max(0.0) as usize;
            format!("{:.*}", magnitude, timestamps as f64 / duration_timestamps)
        }
        TimecodeFormat::Timestamps => (timestamps as i32).to_string(),
    }
}

/// `"{0}:{1:00}"` of the integer part and the floored hundredths.
fn split_hundredths(value: f64) -> String {
    let whole = value as i64;
    let hundredths = ((value - whole as f64) * 100.0).floor() as i64;
    format!("{whole}:{hundredths:02}")
}

/// Converts signed milliseconds to `[h:][mm:]ss.xx[x]` (`TimeHelper.MillisecondsToTimecode`).
pub fn ms_to_timecode(total_milliseconds: f64, precision: i32) -> String {
    const MS_PER_MINUTE: f64 = 60_000.0;
    const MS_PER_HOUR: f64 = 3_600_000.0;

    let mut hours = (total_milliseconds / MS_PER_HOUR) as i64;
    let mut remainder = total_milliseconds % MS_PER_HOUR;
    let mut minutes = (remainder / MS_PER_MINUTE) as i64;
    remainder %= MS_PER_MINUTE;
    let mut seconds = (remainder / 1000.0) as i64;
    let mut milliseconds = remainder % 1000.0;
    let mut centiseconds = i64::from(round_to_int32(milliseconds / 10.0));

    let negative = total_milliseconds < 0.0;
    if negative {
        hours = -hours;
        minutes = -minutes;
        seconds = -seconds;
        centiseconds = -centiseconds;
        milliseconds = -milliseconds;
    }
    let sign = if negative { "- " } else { "" };

    if precision > 2 {
        let fraction = (milliseconds * 10f64.powi(precision - 3)) as i64;
        let width = precision as usize;
        if hours > 0 {
            format!("{sign}{hours}:{minutes:02}:{seconds:02}.{fraction:0width$}")
        } else if minutes > 0 {
            format!("{sign}{minutes}:{seconds:02}.{fraction:0width$}")
        } else {
            format!("{sign}{seconds}.{fraction:0width$}")
        }
    } else {
        if centiseconds == 100 {
            centiseconds = 0;
            seconds += 1;
        }
        if seconds == 60 {
            seconds = 0;
            minutes += 1;
        }
        if minutes == 60 {
            minutes = 0;
            hours += 1;
        }
        if hours > 0 {
            format!("{sign}{hours}:{minutes:02}:{seconds:02}.{centiseconds:02}")
        } else if minutes > 0 {
            format!("{sign}{minutes}:{seconds:02}.{centiseconds:02}")
        } else {
            format!("{sign}{seconds}.{centiseconds:02}")
        }
    }
}
