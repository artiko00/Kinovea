//! Loads an export case the way the oracle's `ExportArea` does: the KVA read with the case
//! context, the whole selection, and the case's export preferences. Test support only.

use crate::measured_data::ExportPrefs;
use kinovea_fixtures::fixtures_root;
use kinovea_geometry::Size;
use kinovea_metadata::{read_kva, Metadata};
use serde_json::Value;

pub struct Loaded {
    pub metadata: Metadata,
    pub prefs: ExportPrefs,
}

pub fn load_case(name: &str) -> Loaded {
    let repo = fixtures_root().join("../..");
    let case: Value = serde_json::from_str(&read(
        &repo.join("tests/cases/export").join(format!("{name}.json")),
    ))
    .unwrap();
    let inputs = &case["inputs"];
    let context = &inputs["context"];
    let number = |key: &str| context[key].as_f64().unwrap();

    let mut metadata = Metadata::new(parse_size(context["imageSize"].as_str().unwrap()));
    metadata.video.average_timestamps_per_frame = number("averageTimeStampsPerFrame");
    metadata.video.average_timestamps_per_second = number("averageTimeStampsPerSecond");
    metadata.video.baseline_frame_interval_ms = number("baselineFrameInterval");
    metadata.video.first_timestamp = context["firstTimeStamp"].as_i64().unwrap();
    metadata.video.time_origin = context["timeOrigin"].as_i64().unwrap();
    metadata.video.video_path = format!("{name}.mp4");
    read_kva(
        &mut metadata,
        &read(&repo.join(inputs["kva"].as_str().unwrap())),
    )
    .unwrap();
    metadata.video.selection_start = i64::MIN;
    metadata.video.selection_end = i64::MAX;

    let prefs = ExportPrefs {
        export_space: inputs["exportSpace"].as_str().unwrap().parse().unwrap(),
        timecode_format: inputs["timecodeFormat"].as_str().unwrap().parse().unwrap(),
        producer: "Kinovea.0.0".to_owned(),
    };
    Loaded { metadata, prefs }
}

fn read(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

/// `"848;480"` as written in the case files.
fn parse_size(text: &str) -> Size {
    let (w, h) = text.split_once(';').unwrap();
    Size::new(w.parse().unwrap(), h.parse().unwrap())
}
