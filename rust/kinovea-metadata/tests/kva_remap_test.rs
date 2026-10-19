//! A file from another video (other image size, other time base) is rescaled and its
//! timestamps remapped onto the current video, as the oracle did with the `remap` case.

use kinovea_fixtures::fixtures_root;
use kinovea_geometry::Size;
use kinovea_metadata::ids::normalize_ids;
use kinovea_metadata::{read_kva, write_kva, ExportFlags, Metadata};

#[test]
fn other_video_context_is_remapped_like_the_original() {
    let repo = fixtures_root().join("../..");
    let xml = std::fs::read_to_string(repo.join("tests/cases/kva/inputs/remap.kva")).unwrap();
    let mut m = Metadata::new(Size::new(640, 360));
    m.video.average_timestamps_per_frame = 400.0;
    m.video.average_timestamps_per_second = 10000.0;
    m.video.baseline_frame_interval_ms = 40.0;
    m.video.first_timestamp = 100;
    m.video.time_origin = 300;
    // The oracle used its case file as the "current video": it exists, so the import is
    // judged against it (different path, no byte length known → different context).
    m.video.video_path = "C:\\oracle\\Kinovea\\tests\\cases\\kva\\remap.json".into();
    let ctx = read_kva(&mut m, &xml).unwrap();
    assert_eq!(ctx.image_size, Size::new(1280, 720));
    assert!(!ctx.is_same_context(&m.video));
    assert_eq!(
        m.keyframes[0].timestamp, 300,
        "time origin maps onto the current origin"
    );
    assert_eq!(
        m.keyframes[1].timestamp, 1900,
        "4 frames of 1000 become 4 frames of 400"
    );
    let got = normalize_ids(&write_kva(&m, "0.0", false, ExportFlags::FULL).to_document());
    let want = std::fs::read_to_string(fixtures_root().join("kva/remap/roundtrip.kva")).unwrap();
    if got != want {
        let first = got
            .lines()
            .zip(want.lines())
            .position(|(g, w)| g != w)
            .unwrap_or(0);
        let g: Vec<&str> = got.lines().skip(first.saturating_sub(2)).take(6).collect();
        let w: Vec<&str> = want.lines().skip(first.saturating_sub(2)).take(6).collect();
        panic!(
            "remap: first difference near line {}\n got: {g:#?}\nwant: {w:#?}",
            first + 1
        );
    }
}
