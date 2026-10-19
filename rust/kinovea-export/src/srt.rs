//! SubRip importer (`MetadataImporterSRT`): each cue becomes a `Subtitle` text drawing on a keyframe
//! at its start, snapped to a video frame, visible until its end. Formatting tags are removed.

use crate::import::{ImportError, ImportReport};
use crate::srt_timecode::parse_timecode;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Label;
use kinovea_metadata::{Drawing, Fading, Keyframe, Metadata};

const ARROWS: [&str; 3] = ["-->", "- >", "->"];

/// Imports SRT `text` into `m`: one keyframe and one text drawing per cue.
pub fn import_srt(m: &mut Metadata, text: &str) -> Result<ImportReport, ImportError> {
    let mut report = ImportReport::default();
    for block in blocks(text) {
        let Some((start, end, content)) = parse_block(&block) else {
            continue;
        };
        add_cue(m, start, end, &content);
        report.keyframes += 1;
        report.drawings += 1;
    }
    Ok(report)
}

/// Blocks of consecutive non-blank lines, each line trimmed.
fn blocks(text: &str) -> Vec<Vec<&str>> {
    let mut out: Vec<Vec<&str>> = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !current.is_empty() {
                out.push(std::mem::take(&mut current));
            }
        } else {
            current.push(line);
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

/// `start`, `end` in milliseconds and the text, one line each with a final newline. `None` when the
/// block has no text. The first line that splits into two timecodes sets the times; later lines are text.
fn parse_block(lines: &[&str]) -> Option<(i64, i64, String)> {
    let (mut start, mut end) = (-1, -1);
    let mut text = String::new();
    for line in lines {
        if start == -1 && end == -1 {
            if let Some((s, e)) = split_timecode(line) {
                (start, end) = (s, e);
            }
        } else {
            text.push_str(&strip_tags(line));
            text.push('\n');
        }
    }
    ((start != -1 || end != -1) && !text.is_empty()).then_some((start, end, text))
}

/// `TryParseTimecodeLine`: two timecodes around an arrow, or `None` when the line has no arrow.
fn split_timecode(line: &str) -> Option<(i64, i64)> {
    let mut parts = Vec::new();
    let mut part_start = 0;
    let mut i = 0;
    while i < line.len() {
        match ARROWS.iter().find(|a| line[i..].starts_with(**a)) {
            Some(arrow) => {
                parts.push(&line[part_start..i]);
                i += arrow.len();
                part_start = i;
            }
            None => i += line[i..].chars().next().map_or(1, char::len_utf8),
        }
    }
    parts.push(&line[part_start..]);
    (parts.len() == 2).then(|| (parse_timecode(parts[0]), parse_timecode(parts[1])))
}

/// Removes `<...>` formatting tags; a `<` without a closing `>` is kept.
pub(crate) fn strip_tags(line: &str) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(open) = rest.find('<') {
        let Some(close) = rest[open..].find('>') else {
            break;
        };
        out.push_str(&rest[..open]);
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    out
}

/// `AddSubtitle`: start and end snapped to frames, a label at 75% of the image height.
fn add_cue(m: &mut Metadata, start: i64, end: i64, content: &str) {
    let atps = m.video.average_timestamps_per_second;
    let atpf = m.video.average_timestamps_per_frame;
    let start_ts = snap(start, atps, atpf);
    let end_ts = snap(end, atps, atpf);

    let top = m.video.image_size.height as f32 * 0.75;
    let mut label = Label::new(content, PointF::new(200.0, top));
    label.header.name = "Subtitle".to_owned();
    label.header.reference_timestamp = start_ts;
    label.header.fading = Fading {
        use_default: false,
        reference_timestamp: start_ts,
        average_timestamps_per_frame: atpf,
        always_visible: false,
        opaque_frames: ((end_ts - start_ts) as f32 / atpf as f32) as i32,
        fading_frames: 10,
        ..Fading::default()
    };
    let mut keyframe = Keyframe::new(start_ts, "");
    keyframe.drawings.push(Drawing::Label(label));
    m.merge_insert_keyframe(keyframe);
}

/// Milliseconds to timestamps, then to the nearest frame boundary (`Math.Round` rounds halves to even).
fn snap(ms: i64, atps: f64, atpf: f64) -> i64 {
    let ts = (ms as f64 * atps / 1000.0).round_ties_even() as i64;
    let frame = (ts as f64 / atpf).round_ties_even() as i64;
    (frame as f64 * atpf).round_ties_even() as i64
}
