//! SRT timecodes (`ParseSrtTimecode`): `h:mm:ss` with an optional `,` or `.` fraction, in milliseconds.

/// `ParseSrtTimecode`: the first `h:mm:ss` with an optional `,` or `.` fraction, in milliseconds
/// (truncated). `-1` when there is none or it is out of range.
pub(crate) fn parse_timecode(text: &str) -> i64 {
    let bytes = text.as_bytes();
    (0..bytes.len())
        .filter(|&i| bytes[i].is_ascii_digit())
        .find_map(|i| timecode_end(bytes, i).map(|end| &text[i..end]))
        .map_or(-1, to_millis)
}

/// End of `h+:m+:s+([,.]f+)?` starting at `start`, if it matches.
fn timecode_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = digits_end(bytes, start)?;
    for _ in 0..2 {
        if bytes.get(i) != Some(&b':') {
            return None;
        }
        i = digits_end(bytes, i + 1)?;
    }
    if matches!(bytes.get(i), Some(b',') | Some(b'.')) {
        if let Some(end) = digits_end(bytes, i + 1) {
            i = end;
        }
    }
    Some(i)
}

fn digits_end(bytes: &[u8], start: usize) -> Option<usize> {
    let end = start
        + bytes[start.min(bytes.len())..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count();
    (end > start).then_some(end)
}

/// `TimeSpan.TryParse` of `h:mm:ss[.fraction]` (up to 7 fraction digits), truncated to milliseconds.
fn to_millis(text: &str) -> i64 {
    let normalized = text.replace(',', ".");
    let (clock, fraction) = normalized.split_once('.').unwrap_or((&normalized, ""));
    if fraction.len() > 7 {
        return -1;
    }
    let parts: Vec<i64> = match clock
        .split(':')
        .map(|p| p.parse::<i64>())
        .collect::<Result<_, _>>()
    {
        Ok(parts) => parts,
        Err(_) => return -1,
    };
    let [h, mi, s] = parts[..] else {
        return -1;
    };
    if mi > 59 || s > 59 {
        return -1;
    }
    let fraction_ticks: i64 = format!("{fraction:0<7}").parse().unwrap_or(0);
    let seconds = (h * 60 + mi) * 60 + s;
    (seconds * 10_000_000 + fraction_ticks) / 10_000
}
