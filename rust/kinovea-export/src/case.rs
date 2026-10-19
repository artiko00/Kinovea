//! Loads an export case the way the oracle's `ExportArea` does: the KVA read with the case
//! context, the whole selection, and the case's export preferences. Test support only.

use crate::collect::collect_measured_data;
use crate::measured_data::{ExportPrefs, MeasuredData};
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

/// The measured data of an export case, collected as the oracle does.
pub fn measured(name: &str) -> MeasuredData {
    let loaded = load_case(name);
    collect_measured_data(&loaded.metadata, &loaded.prefs)
}

/// Text of a file of an export case's fixture folder.
pub fn fixture_text(name: &str, file: &str) -> String {
    read(&fixtures_root().join("export").join(name).join(file))
}

/// Checks `write` against the fixture `file` of every case that has it, and against `None` for
/// the cases with measured data but no `file` (the original writes no file then).
pub fn check_optional_file(file: &str, write: impl Fn(&MeasuredData) -> Option<String>) {
    let written = cases_with_fixture(file);
    assert!(!written.is_empty());
    for name in &written {
        assert_eq!(
            write(&measured(name)),
            Some(fixture_text(name, file)),
            "{name}"
        );
    }
    let without: Vec<String> = cases_with_fixture("measured.json")
        .into_iter()
        .filter(|name| !written.contains(name))
        .collect();
    assert!(!without.is_empty());
    for name in &without {
        assert_eq!(write(&measured(name)), None, "{name}");
    }
}

/// Names of the export cases whose fixture folder holds `file`, in name order.
pub fn cases_with_fixture(file: &str) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(fixtures_root().join("export"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.join(file).is_file())
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

fn read(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

/// `"848;480"` as written in the case files.
fn parse_size(text: &str) -> Size {
    let (w, h) = text.split_once(';').unwrap();
    Size::new(w.parse().unwrap(), h.parse().unwrap())
}
