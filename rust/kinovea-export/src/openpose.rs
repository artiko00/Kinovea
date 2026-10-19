//! OpenPose BODY_25 importer (`MetadataImporterOpenPose`): one `OpenPoseBody25` posture per person
//! per frame, one keyframe per frame file. Persons of other models (COCO, MPI) are skipped.

use crate::import::{posture_style, posture_tool, ImportError, ImportReport};
use crate::openpose_files::{frame_files, frame_number, keypoints_prefix};
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::metadata::register_drawing;
use kinovea_metadata::posture::GenericPosture;
use kinovea_metadata::{Drawing, Fading, Keyframe, Metadata};
use serde_json::Value;
use std::path::Path;

const BODY25_POINTS: usize = 25;
const CONFIDENCE_THRESHOLD: f32 = 0.25;

/// Option that shows each BODY_25 joint, by joint index.
const OPTION_KEYS: [&str; BODY25_POINTS] = [
    "showNose",
    "showNeck",
    "showRShoulder",
    "showRElbow",
    "showRWrist",
    "showLShoulder",
    "showLElbow",
    "showLWrist",
    "showMidHip",
    "showRHip",
    "showRKnee",
    "showRAnkle",
    "showLHip",
    "showLKnee",
    "showLAnkle",
    "showREye",
    "showLEye",
    "showREar",
    "showLEar",
    "showLBigToe",
    "showLSmallToe",
    "showLHeel",
    "showRBigToe",
    "showRSmallToe",
    "showRHeel",
];

/// Imports the frame file `source` and every frame file of its folder that shares its prefix.
pub fn import_openpose(m: &mut Metadata, source: &Path) -> Result<ImportReport, ImportError> {
    let name = source
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    let prefix = keypoints_prefix(name).ok_or_else(|| {
        ImportError::InvalidData(format!("{name}: not an OpenPose keypoints file"))
    })?;
    let folder = match source.parent() {
        Some(p) if !p.as_os_str().is_empty() => p,
        _ => Path::new("."),
    };
    let files = frame_files(folder, prefix)?;

    let template = posture_tool("OpenPoseBody25")?;
    let mut report = ImportReport::default();
    for file in files {
        // The original stops at the first file whose frame token is not a number.
        let Some(frame) = frame_number(&file) else {
            report.skipped.push(file);
            break;
        };
        let text = std::fs::read_to_string(folder.join(&file))?;
        let Some(people) = read_people(&text) else {
            report.skipped.push(file);
            continue;
        };
        let timestamp = (m.video.first_timestamp as f64
            + frame as f64 * m.video.average_timestamps_per_frame)
            .round_ties_even() as i64;
        let mut keyframe = Keyframe::new(timestamp, "");
        for keypoints in people
            .into_iter()
            .flatten()
            .filter(|k| k.len() == 3 * BODY25_POINTS)
        {
            let drawing =
                Drawing::GenericPostureDrawing(body25_drawing(&template, &keypoints, timestamp, m));
            register_drawing(&mut m.trackability, &drawing);
            keyframe.drawings.push(drawing);
            report.drawings += 1;
        }
        m.merge_insert_keyframe(keyframe);
        report.keyframes += 1;
    }
    Ok(report)
}

/// One person as a posture drawing, with onion skinning off (`ParsePosture` and the fading settings).
fn body25_drawing(
    template: &GenericPosture,
    keypoints: &[f32],
    timestamp: i64,
    m: &Metadata,
) -> GenericPostureDrawing {
    let mut posture = template.clone();
    for (index, joint) in keypoints.chunks_exact(3).enumerate() {
        if let Some(point) = posture.point_list.get_mut(index) {
            *point = PointF::new(joint[0], joint[1]);
        }
        let option = OPTION_KEYS
            .get(index)
            .and_then(|key| posture.options.iter_mut().find(|o| o.key == *key));
        if let Some(option) = option {
            option.value = joint[2] >= CONFIDENCE_THRESHOLD;
        }
    }
    let mut d = GenericPostureDrawing::new(template.id, Some(posture));
    d.header.name = "OpenPose".to_owned();
    d.header.style = posture_style();
    d.header.fading = Fading {
        use_default: false,
        reference_timestamp: timestamp,
        average_timestamps_per_frame: m.video.average_timestamps_per_frame,
        always_visible: false,
        opaque_frames: 1,
        fading_frames: 0,
        ..Fading::default()
    };
    d
}

/// Keypoints of every person (`None` when the person has no `pose_keypoints_2d`), or `None` when the
/// file is not a valid OpenPose frame.
fn read_people(text: &str) -> Option<Vec<Option<Vec<f32>>>> {
    let root: Value = serde_json::from_str(text).ok()?;
    root.get("people")?
        .as_array()?
        .iter()
        .map(|person| match person.get("pose_keypoints_2d") {
            None | Some(Value::Null) => Some(None),
            Some(values) => values
                .as_array()?
                .iter()
                .map(|v| v.as_f64().map(|x| x as f32))
                .collect::<Option<Vec<f32>>>()
                .map(Some),
        })
        .collect()
}
