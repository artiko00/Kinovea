use crate::results::{OpenVideoResult, RecordingResult, VideoExportResult};

#[test]
fn result_enums_compare_by_variant() {
    assert_ne!(OpenVideoResult::Success, OpenVideoResult::Cancelled);
    assert_ne!(RecordingResult::Success, RecordingResult::UnknownError);
    assert_ne!(VideoExportResult::Success, VideoExportResult::FfmpegError);
}
