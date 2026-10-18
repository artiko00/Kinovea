use super::capture_automation::*;
use super::helpers::XmlField;

#[test]
fn defaults_match_the_original_file() {
    let auto = CaptureAutomationConfiguration::default()
        .write("A")
        .unwrap();
    assert_eq!(auto.child_text("AudioTriggerThreshold"), Some("0.900"));
    assert_eq!(
        auto.child_text("AudioInputDevice"),
        Some("00000000-0000-0000-0000-000000000000")
    );
    assert_eq!(auto.child_text("UDPPort"), Some("8875"));
    let photo = PhotofinishConfiguration::default().write("P").unwrap();
    assert_eq!(photo.child_text("OutputHeight"), Some("1000"));
}
