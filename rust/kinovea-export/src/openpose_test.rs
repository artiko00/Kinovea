use crate::import_test::{assert_matches_oracle, copy_into, document, scratch_dir};
use crate::openpose::import_openpose;
use kinovea_metadata::drawing::Drawing;
use kinovea_metadata::Metadata;

fn frame_name(frame: u32) -> String {
    format!("clip_{frame:012}_keypoints.json")
}

/// A folder holding the three sample frames, plus `extra` files given as (name, content).
fn frames_folder(tag: &str, frames: &[u32], extra: &[(&str, &str)]) -> std::path::PathBuf {
    let dir = scratch_dir(tag);
    for frame in frames {
        copy_into(
            &dir,
            &format!("tests/data/import/openpose/{}", frame_name(*frame)),
            &frame_name(*frame),
        );
    }
    for (name, text) in extra {
        std::fs::write(dir.join(name), text).unwrap();
    }
    dir
}

fn timestamps(m: &Metadata) -> Vec<i64> {
    m.keyframes.iter().map(|k| k.timestamp).collect()
}

fn posture_option(m: &Metadata, timestamp: i64, key: &str) -> bool {
    let keyframe = m
        .keyframes
        .iter()
        .find(|k| k.timestamp == timestamp)
        .unwrap();
    let Drawing::GenericPostureDrawing(d) = &keyframe.drawings[0] else {
        panic!("expected a posture drawing");
    };
    let posture = d.posture.as_ref().unwrap();
    posture.options.iter().find(|o| o.key == key).unwrap().value
}

#[test]
fn folder_of_three_frames_matches_the_oracle() {
    let dir = frames_folder("openpose-frames", &[0, 1, 2], &[]);
    let mut m = document("openpose");
    let report = import_openpose(&mut m, &dir.join(frame_name(0))).unwrap();
    assert_eq!(timestamps(&m), vec![0, 400, 800]);
    assert_eq!(report.keyframes, 3);
    assert_eq!(report.drawings, 3);
    assert!(report.skipped.is_empty());
    assert_matches_oracle(&m, "openpose");
}

#[test]
fn drawings_are_named_and_use_the_body25_tool() {
    let dir = frames_folder("openpose-names", &[0], &[]);
    let mut m = document("openpose");
    import_openpose(&mut m, &dir.join(frame_name(0))).unwrap();
    let Drawing::GenericPostureDrawing(d) = &m.keyframes[0].drawings[0] else {
        panic!("expected a posture drawing");
    };
    assert_eq!(d.header.name, "OpenPose");
    assert_eq!(d.tool_name(), "OpenPoseBody25");
    assert_eq!(
        m.keyframes[0].drawings.len(),
        1,
        "the COCO person is skipped"
    );
}

#[test]
fn low_confidence_joint_hides_its_point_and_keeps_the_others() {
    let dir = frames_folder("openpose-confidence", &[1], &[]);
    let mut m = document("openpose");
    import_openpose(&mut m, &dir.join(frame_name(1))).unwrap();
    assert!(
        !posture_option(&m, 400, "showRWrist"),
        "confidence 0.1 is under 0.25"
    );
    assert!(
        posture_option(&m, 400, "showRShoulder"),
        "confidence 0.9 stays visible"
    );
}

#[test]
fn unparseable_frame_token_stops_the_import_and_reports_the_file() {
    // Sorts between frames 1 and 2, and its token before `keypoints.json` is not a number.
    let bad = "clip_000000000001x_keypoints.json";
    let dir = frames_folder("openpose-token", &[0, 1, 2], &[(bad, "{}")]);
    let mut m = document("openpose");
    let report = import_openpose(&mut m, &dir.join(frame_name(0))).unwrap();
    assert_eq!(
        timestamps(&m),
        vec![0, 400],
        "the frame after the bad file is not imported"
    );
    assert_eq!(report.skipped, vec![bad.to_owned()]);
}

#[test]
fn invalid_json_skips_only_that_frame() {
    let dir = frames_folder("openpose-json", &[0, 2], &[(&frame_name(1), "not json")]);
    let mut m = document("openpose");
    let report = import_openpose(&mut m, &dir.join(frame_name(0))).unwrap();
    assert_eq!(timestamps(&m), vec![0, 800]);
    assert_eq!(report.skipped, vec![frame_name(1)]);
}
