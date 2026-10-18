use crate::drawing::{Chrono, Drawing};
use crate::keyframe::Keyframe;
use crate::kva::{write_kva, ExportFlags};
use crate::metadata::Metadata;
use kinovea_geometry::{PointF, Size};

fn names(root: &kinovea_core::preferences::dom::Element) -> Vec<&str> {
    root.children.iter().map(|c| c.name.as_str()).collect()
}

#[test]
fn empty_metadata_writes_the_fixed_sections_in_order() {
    let m = Metadata::new(Size::new(512, 384));
    let root = write_kva(&m, "0.0", false, ExportFlags::FULL);
    let want = ["FormatVersion", "Producer", "OriginalFilename", "FullPath", "ByteLength", "ImageSize", "Aspect", "Rotation", "Mirror", "Demosaicing", "Deinterlacing", "StabilizationTrack", "BackgroundColor", "AverageTimeStampsPerFrame", "CaptureFramerate", "VideoFramerate", "FirstTimeStamp", "SelectionStart", "SelectionEnd", "TimeOrigin", "Calibration", "CoordinateSystem", "TestGrid", "Trackability", "VideoFilters"];
    assert_eq!(names(&root), want);
    assert_eq!(root.child_text("Producer"), Some("Kinovea.0.0"));
    assert_eq!(root.child_text("VideoFramerate"), Some("25"));
    assert_eq!(root.child_text("BackgroundColor"), Some("0;255;255;255"));
    let cal = root.child("Calibration").unwrap();
    assert_eq!(cal.child("CalibrationLine").unwrap().child("Segment").unwrap().child_text("A"), Some("256;192"));
}

#[test]
fn optional_sections_and_capture_recording_flags() {
    let mut m = Metadata::new(Size::new(100, 100));
    m.video.video_path = "/clips/jump.mp4".into();
    m.global_title = "Title".into();
    m.merge_insert_keyframe(Keyframe::new(5, "k"));
    m.chronos.push(Drawing::Chrono(Chrono::new(PointF::EMPTY)));
    let full = write_kva(&m, "1.2", false, ExportFlags::FULL);
    assert_eq!(full.child_text("OriginalFilename"), Some("jump"));
    assert!(full.child("GlobalTitle").is_some() && full.child("Keyframes").is_some() && full.child("Chronos").is_some());
    assert!(full.child("Tracks").is_none() && full.child("Spotlights").is_none());
    let capture = write_kva(&m, "1.2", true, ExportFlags::CAPTURE_RECORDING);
    assert_eq!(capture.child_text("CaptureRecording"), Some("true"));
    assert!(capture.child("FullPath").is_none() && capture.child("FirstTimeStamp").is_none());
    assert!(capture.child("Chronos").is_none() && capture.child("Trackability").is_none() && capture.child("VideoFilters").is_none());
    assert!(capture.child("Keyframes").is_some() && capture.child("Calibration").is_some() && capture.child("TestGrid").is_some());
}
