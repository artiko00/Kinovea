//! Sports2D TRC importer (`MetadataImporterSports2D`): one `BodyWithFeet` posture named `Human` on the
//! first row, one track per marker and a tracker binding the posture points to those tracks.
//! The header's DataRate and CameraRate are not used, as in the original.

use crate::import::{posture_style, posture_tool, ImportError, ImportReport};
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{GenericPostureDrawing, Track};
use kinovea_metadata::measure_label::MeasureLabel;
use kinovea_metadata::metadata::Manager;
use kinovea_metadata::posture::GenericPosture;
use kinovea_metadata::style::{StyleElements, StyleValue, TrackShape};
use kinovea_metadata::trackability::{DrawingTracker, TrackablePoint};
use kinovea_metadata::{Color, Drawing, Keyframe, Metadata};
use std::str::FromStr;

const FIRST_DATA_LINE: usize = 5;

/// Imports TRC `text` (tab-separated, `.` decimal point) into `m`.
pub fn import_trc(m: &mut Metadata, text: &str) -> Result<ImportReport, ImportError> {
    let lines: Vec<&str> = text.lines().collect();
    let head3: Vec<&str> = line(&lines, 2)?.split('\t').collect();
    let num_frames: usize = field(&head3, 2, "line 3")?;
    let num_markers: usize = field(&head3, 3, "line 3")?;

    let head4: Vec<&str> = line(&lines, 3)?
        .split('\t')
        .filter(|f| !f.is_empty())
        .collect();
    if head4.len() != num_markers + 2 {
        return Err(ImportError::InvalidData(format!(
            "line 4: expected {} labels, found {}",
            num_markers + 2,
            head4.len()
        )));
    }
    let markers: Vec<&str> = head4[2..].to_vec();

    let template = posture_tool("BodyWithFeet")?;
    let mut timelines: Vec<Vec<TimedPoint>> = vec![Vec::new(); markers.len()];
    let mut first: Option<GenericPosture> = None;
    let mut first_timestamp = 0;
    for line_index in FIRST_DATA_LINE..num_frames {
        let frame_index = (line_index - FIRST_DATA_LINE) as f64;
        let timestamp = (m.video.first_timestamp as f64
            + frame_index * m.video.average_timestamps_per_frame)
            .round_ties_even() as i64;
        let posture = parse_row(
            &template,
            line(&lines, line_index)?,
            &markers,
            line_index + 1,
        )?;
        for (j, marker) in markers.iter().enumerate() {
            let p = get_value(&posture, marker);
            timelines[j].push(TimedPoint::new(p.x, p.y, timestamp));
        }
        if first.is_none() {
            first = Some(posture);
            first_timestamp = timestamp;
        }
    }
    let Some(first) = first else {
        return Err(ImportError::InvalidData("no data rows".to_owned()));
    };

    let mut d = GenericPostureDrawing::new(template.id, Some(first.clone()));
    d.header.name = "Human".to_owned();
    d.header.style = posture_style();
    d.header.reference_timestamp = first_timestamp;
    let drawing = Drawing::GenericPostureDrawing(d);
    let (drawing_id, tracker_points) = (drawing.id(), drawing.trackable_points());

    let mut track_ids = Vec::with_capacity(markers.len());
    for (j, marker) in markers.iter().enumerate() {
        let track = build_track(marker, &timelines[j], first.points[j].color);
        track_ids.push(track.header.id);
        m.add_drawing(Manager::Tracks, Drawing::Track(track))
            .expect("the tracks container takes any drawing");
    }

    let points = tracker_points
        .into_iter()
        .map(|(key, value)| {
            let track_id = key
                .parse::<usize>()
                .ok()
                .and_then(|j| track_ids.get(j).copied());
            (
                key,
                TrackablePoint {
                    reference_timestamp: first_timestamp,
                    reference_value: value,
                    track_id,
                },
            )
        })
        .collect();
    m.trackability.import_tracker(DrawingTracker {
        drawing_id,
        name: "Human".to_owned(),
        points,
        object_tracking_initialized: true,
    });

    let mut keyframe = Keyframe::new(first_timestamp, "");
    keyframe.drawings.push(drawing);
    m.merge_insert_keyframe(keyframe);

    Ok(ImportReport {
        keyframes: 1,
        drawings: 1,
        tracks: markers.len(),
        skipped: Vec::new(),
    })
}

/// `DrawingTrack(name, points)`: the first point is added twice, then every timed position.
fn build_track(marker: &str, timeline: &[TimedPoint], color: Color) -> Track {
    let first = timeline[0];
    let mut positions = vec![first];
    positions.extend_from_slice(timeline);
    let mut track = Track::new(positions);
    track.header.name = format!("Human.{marker}");
    track.header.style = track_style(color);
    track.main_label = MeasureLabel::new(PointF::new(first.x, first.y));
    track
}

/// Style of the imported tracks: solid, line size 1, point size 3, label size 8.
fn track_style(color: Color) -> StyleElements {
    StyleElements::new(vec![
        ("color", StyleValue::Color(color)),
        ("track shape", StyleValue::TrackShape(TrackShape::SOLID)),
        ("line size", StyleValue::LineSize(1)),
        ("TrackPointSize", StyleValue::PenSize(3)),
        ("label size", StyleValue::FontSize(8)),
    ])
}

/// One data row: a frame, a time, then x, y, z per marker. Coordinates are scaled to pixels with the
/// y axis inverted, as Sports2D writes them in metres.
fn parse_row(
    template: &GenericPosture,
    row: &str,
    markers: &[&str],
    line_number: usize,
) -> Result<GenericPosture, ImportError> {
    let values: Vec<&str> = row.split('\t').collect();
    if values.len() != 2 + 3 * markers.len() {
        return Err(ImportError::InvalidData(format!(
            "line {line_number}: expected {} values, found {}",
            2 + 3 * markers.len(),
            values.len()
        )));
    }
    let mut posture = template.clone();
    for (j, marker) in markers.iter().enumerate() {
        let x: f32 = number(values[2 + 3 * j], line_number)?;
        let y: f32 = number(values[3 + 3 * j], line_number)?;
        assign_value(&mut posture, marker, PointF::new(x * 1000.0, y * -1000.0));
    }
    Ok(posture)
}

fn line<'a>(lines: &[&'a str], index: usize) -> Result<&'a str, ImportError> {
    lines
        .get(index)
        .copied()
        .ok_or_else(|| ImportError::InvalidData(format!("line {}: missing", index + 1)))
}

fn field<T: FromStr>(fields: &[&str], index: usize, at: &str) -> Result<T, ImportError> {
    fields
        .get(index)
        .and_then(|f| f.trim().parse().ok())
        .ok_or_else(|| {
            ImportError::InvalidData(format!("{at}: field {} is not a number", index + 1))
        })
}

fn number(text: &str, line_number: usize) -> Result<f32, ImportError> {
    text.parse().map_err(|_| {
        ImportError::InvalidData(format!("line {line_number}: '{text}' is not a number"))
    })
}

/// Index of the point of the handle called `name`.
fn handle_index(posture: &GenericPosture, name: &str) -> Option<usize> {
    let handle = posture.handles.iter().find(|h| h.name == name)?;
    usize::try_from(handle.reference).ok()
}

/// `AssignValue`: sets the point of the handle called `name`; unknown names are ignored.
fn assign_value(posture: &mut GenericPosture, name: &str, value: PointF) {
    if let Some(point) = handle_index(posture, name).and_then(|i| posture.point_list.get_mut(i)) {
        *point = value;
    }
}

/// `GetValue`: the point of the handle called `name`, or empty when unknown.
fn get_value(posture: &GenericPosture, name: &str) -> PointF {
    handle_index(posture, name)
        .and_then(|i| posture.point_list.get(i).copied())
        .unwrap_or(PointF::EMPTY)
}
