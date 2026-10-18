use super::capture::*;
use super::dom::Element;
use super::enums::{CaptureRecordingMode, KvaExportFlags};
use super::helpers::XmlField;

#[test]
fn defaults_match_the_original() {
    let c = CapturePreferences::default();
    assert_eq!(c.recording_mode, CaptureRecordingMode::Delay);
    assert_eq!(c.memory_buffer, 768);
    assert_eq!(c.export_flags, KvaExportFlags::DefaultCaptureRecording);
    assert_eq!(c.highspeed_recording_framerate_threshold.0, 150.0);
    let e = c.write("Capture").unwrap();
    assert_eq!(
        e.child_text("DisplaySynchronizationFramerate"),
        Some("25.000")
    );
    assert!(e.child("Cameras").is_none());
    let auto = e.child("CaptureAutomationConfiguration").unwrap();
    assert_eq!(auto.child_text("AudioTriggerThreshold"), Some("0.900"));
    assert_eq!(
        auto.child_text("AudioInputDevice"),
        Some("00000000-0000-0000-0000-000000000000")
    );
}

#[test]
fn cameras_are_preserved_verbatim() {
    let xml = "<Capture><Cameras><Camera><CameraType>x</CameraType><Specific><Any a=\"1\" /></Specific></Camera></Cameras><MemoryBuffer>99</MemoryBuffer></Capture>";
    let mut c = CapturePreferences::default();
    c.read(&Element::parse(xml).unwrap());
    assert_eq!(c.memory_buffer, 99);
    assert_eq!(c.cameras.0.len(), 1);
    let out = c.write("Capture").unwrap();
    assert_eq!(out.child("Cameras").unwrap().children[0], c.cameras.0[0]);
}
