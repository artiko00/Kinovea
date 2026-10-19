//! Collects the measured data of a document in the original order (`Metadata.CollectMeasuredData`).

use crate::collect_drawings::collect_keyframe;
use crate::collect_series::{times, timeseries};
use crate::measure::{Scope, TimeBase};
use crate::measured_data::{keyframe_name, unit_symbols, ExportPrefs, KeyframeData, MeasuredData};
use kinovea_metadata::Metadata;
use std::path::Path;

pub fn collect_measured_data(metadata: &Metadata, prefs: &ExportPrefs) -> MeasuredData {
    let scope = Scope { metadata, prefs };
    let video = &metadata.video;
    let mut md = MeasuredData {
        producer: prefs.producer.clone(),
        full_path: video.video_path.clone(),
        original_filename: Path::new(&video.video_path)
            .file_stem()
            .map_or_else(String::new, |s| s.to_string_lossy().into_owned()),
        image_size: video.image_size,
        capture_framerate: metadata.calibration.capture_fps() as f32,
        video_framerate: (1000.0 / video.baseline_frame_interval_ms) as f32,
        units: unit_symbols(metadata, prefs),
        keyframes: Vec::new(),
        positions: Vec::new(),
        distances: Vec::new(),
        angles: Vec::new(),
        times: Vec::new(),
        timeseries: Vec::new(),
    };

    // Keyframes outside the selection are disabled in the original and skipped.
    let (start, end) = (video.selection_start, video.selection_end);
    for kf in metadata
        .keyframes
        .iter()
        .filter(|k| k.timestamp >= start && k.timestamp <= end)
    {
        let time = scope.time(kf.timestamp, TimeBase::UserOrigin);
        md.keyframes.push(KeyframeData {
            name: keyframe_name(kf, metadata.timecode),
            time,
            comment: kf.comments.clone(),
        });
        let measures = collect_keyframe(&scope, &kf.drawings, time);
        md.positions.extend(measures.positions);
        md.distances.extend(measures.distances);
        md.angles.extend(measures.angles);
    }

    md.times = times(&scope);
    md.timeseries = timeseries(&scope);
    md
}
