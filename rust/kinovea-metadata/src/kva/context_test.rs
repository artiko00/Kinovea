use crate::kva::ImportContext;
use crate::kva_io::Scale;
use crate::metadata::VideoContext;
use kinovea_geometry::Size;

fn current() -> VideoContext {
    VideoContext { video_path: "/videos/run.mp4".into(), video_byte_length: 1000, image_size: Size::new(640, 360), average_timestamps_per_second: 50.0, baseline_frame_interval_ms: 20.0, first_timestamp: 100, time_origin: 300, ..VideoContext::default() }
}

#[test]
fn scaling_is_the_size_ratio() {
    let ctx = ImportContext { image_size: Size::new(1280, 720), ..ImportContext::default() };
    assert_eq!(ctx.scaling(&current()), Scale { x: 0.5, y: 0.5 });
    assert_eq!(ctx.scaling(&VideoContext::default()), Scale::IDENTITY);
    assert_eq!(ImportContext::default().scaling(&current()), Scale::IDENTITY);
}

#[test]
fn same_context_detection() {
    let mut c = current();
    assert!(ImportContext { path: "/videos/run.mp4".into(), ..ImportContext::default() }.is_same_context(&c));
    assert!(ImportContext { byte_length: 1000, ..ImportContext::default() }.is_same_context(&c));
    assert!(!ImportContext { byte_length: 999, ..ImportContext::default() }.is_same_context(&c));
    assert!(ImportContext { path: "C:\\other\\run.mp4".replace('\\', "/"), ..ImportContext::default() }.is_same_context(&c));
    assert!(ImportContext { file_name: "run".into(), ..ImportContext::default() }.is_same_context(&c));
    assert!(!ImportContext { file_name: "walk".into(), ..ImportContext::default() }.is_same_context(&c));
    assert!(!ImportContext::default().is_same_context(&c));
    c.video_path.clear();
    assert!(ImportContext::default().is_same_context(&c));
}

#[test]
fn remapping_uses_frames_and_the_output_time_base() {
    let ctx = ImportContext { average_timestamps_per_frame: 10.0, first_timestamp: 1000, time_origin: 2000, ..ImportContext::default() };
    assert_eq!(ctx.remap(2050, &current(), true), 2050);
    // 5 frames after the origin; output has 1 timestamp per frame.
    assert_eq!(ctx.remap(2050, &current(), false), 305);
    let capture = ImportContext { is_capture_recording: true, ..ctx.clone() };
    assert_eq!(capture.remap(1020, &current(), false), 102);
    assert_eq!(ImportContext::default().remap(7, &current(), false), 7);
}

#[test]
fn remapping_rounds_exact_halves_to_even() {
    // Output has one timestamp per frame from 300: 0.5 and 1.5 frames land on 300.5 and 301.5.
    let ctx = ImportContext { average_timestamps_per_frame: 2.0, ..ImportContext::default() };
    assert_eq!(ctx.remap(1, &current(), false), 300);
    assert_eq!(ctx.remap(3, &current(), false), 302);
}

#[test]
fn header_metadata_mirrors_image_size_and_timing() {
    let m = crate::kva::header_metadata("<KinoveaVideoAnalysis><ImageSize>320;200</ImageSize><AverageTimeStampsPerFrame>40</AverageTimeStampsPerFrame></KinoveaVideoAnalysis>");
    assert_eq!(m.video.image_size, kinovea_geometry::Size::new(320, 200));
    assert_eq!(m.video.average_timestamps_per_frame, 40.0);
    assert_eq!(crate::kva::header_metadata("<x/>").video.average_timestamps_per_frame, 1.0);
}

#[test]
fn file_stem_accepts_both_separators() {
    use crate::kva::context::file_stem;
    assert_eq!(file_stem("C:\\videos\\run.mp4"), "run");
    assert_eq!(file_stem("/clips/jump.tar.gz"), "jump.tar");
    assert_eq!(file_stem(".hidden"), ".hidden");
    assert_eq!(file_stem(""), "");
}
