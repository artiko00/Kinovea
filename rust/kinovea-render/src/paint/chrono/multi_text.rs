//! Owner: P2. Port of `ChronoStringBuilder.Build`: one row per started section (marker, name,
//! duration, cumulated time, tag) with the visible columns padded to a common width.

use kinovea_metadata::drawing::{ChronoColumn, ChronoSection};

/// Cells of one section at a timestamp. Sections not started yet keep empty cells.
#[derive(Default)]
struct Row {
    current: bool,
    name: String,
    duration: String,
    cumul: String,
}

/// Text of the multi-section stopwatch at `timestamp`. Rows end with `\n` (`Environment.NewLine`
/// on the oracle's platform, rendered as a line break). Before the first section, the text is the
/// zero timecode; on the first section only its duration is shown, as in the original.
pub fn build_text(
    sections: &[ChronoSection],
    timestamp: i64,
    visible: &[ChronoColumn],
    timecode: fn(i64) -> String,
) -> String {
    if sections.first().is_none_or(|s| timestamp < s.section.start) {
        return timecode(0);
    }
    let rows = rows(sections, timestamp, timecode);
    if sections.len() == 1 || timestamp < sections[1].section.start {
        return rows[0].duration.clone();
    }
    let name_w = rows
        .iter()
        .map(|r| r.name.chars().count())
        .max()
        .unwrap_or(0);
    let duration_w = rows
        .iter()
        .map(|r| r.duration.chars().count())
        .max()
        .unwrap_or(0);
    let cumul_w = rows
        .iter()
        .map(|r| r.cumul.chars().count())
        .max()
        .unwrap_or(0);
    let tag_w = sections
        .iter()
        .map(|s| s.tag.chars().count())
        .max()
        .unwrap_or(0);

    let mut out = String::new();
    for (row, section) in rows
        .iter()
        .zip(sections)
        .take_while(|(_, s)| timestamp >= s.section.start)
    {
        out.push_str(if row.current { "▶ " } else { "  " });
        if visible.contains(&ChronoColumn::Name) {
            out.push_str(&format!("{}: ", pad(&row.name, name_w)));
        }
        let mut cells = Vec::new();
        if visible.contains(&ChronoColumn::Duration) {
            cells.push(pad(&row.duration, duration_w));
        }
        if visible.contains(&ChronoColumn::Cumul) {
            cells.push(pad(&row.cumul, cumul_w));
        }
        if visible.contains(&ChronoColumn::Tag) && tag_w > 0 {
            cells.push(pad(&section.tag, tag_w));
        }
        out.push_str(&cells.join(" | "));
        out.push('\n');
    }
    out
}

/// `UpdateTimecodes`: elapsed time of each started section (up to its end), cumulated over the
/// started sections.
fn rows(sections: &[ChronoSection], timestamp: i64, timecode: fn(i64) -> String) -> Vec<Row> {
    let mut cumul_timestamps = 0;
    let mut rows = Vec::with_capacity(sections.len());
    for (i, s) in sections.iter().enumerate() {
        if timestamp < s.section.start {
            rows.push(Row::default());
            continue;
        }
        let elapsed = if timestamp <= s.section.end {
            timestamp - s.section.start
        } else {
            s.section.end - s.section.start
        };
        cumul_timestamps += elapsed;
        rows.push(Row {
            current: s.section.contains(timestamp),
            name: if s.name.is_empty() {
                (i + 1).to_string()
            } else {
                s.name.clone()
            },
            duration: timecode(elapsed),
            cumul: timecode(cumul_timestamps),
        });
    }
    rows
}

/// `{0,N}` of the original: right-aligned in `width` characters.
fn pad(text: &str, width: usize) -> String {
    format!("{text:>width$}")
}
