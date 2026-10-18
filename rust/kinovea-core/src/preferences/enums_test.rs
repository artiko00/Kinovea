use super::enums::*;

#[test]
fn names_round_trip() {
    for v in <MeasureLabelType as NamedEnum>::ALL {
        assert_eq!(v.to_string().parse::<MeasureLabelType>().unwrap(), *v);
    }
    assert_eq!(ImageFormat::JPG.to_string(), "JPG");
    assert_eq!(
        "SnapPercent".parse::<TimelineJumpUnit>().unwrap(),
        TimelineJumpUnit::SnapPercent
    );
    assert_eq!(
        KvaExportFlags::DefaultCaptureRecording.name(),
        "DefaultCaptureRecording"
    );
}

#[test]
fn unknown_name_is_an_error() {
    let err = "Huge".parse::<ExplorerThumbSize>().unwrap_err();
    assert_eq!(err.to_string(), "unknown thumbnail size: Huge");
}
