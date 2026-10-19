use crate::case::load_case;
use crate::collect::collect_measured_data;
use kinovea_geometry::PointF;

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

#[test]
fn complete_sample_has_the_oracle_counts() {
    let loaded = load_case("complete");
    let md = collect_measured_data(&loaded.metadata, &loaded.prefs);
    assert_eq!(md.keyframes.len(), 9);
    assert_eq!(md.positions.len(), 4);
    assert_eq!(md.distances.len(), 3);
    assert_eq!(md.angles.len(), 3);
    assert_eq!(md.times.len(), 2);
    assert_eq!(md.timeseries.len(), 1);
}

#[test]
fn complete_sample_values_match_the_oracle() {
    let loaded = load_case("complete");
    let md = collect_measured_data(&loaded.metadata, &loaded.prefs);
    assert_eq!(md.keyframes[0].name, "comments");
    assert!(close(md.keyframes[0].time, 1.28));
    assert_eq!(md.positions[0].name, "Marker 1");
    assert_eq!(md.positions[0].point, PointF::new(-165.0, 161.0));
    assert!(close(md.distances[0].value, 97.308_784));
    assert!(close(md.distances[0].time, 5.48));
    assert!(close(md.angles[0].value, -90.0));
    assert_eq!(md.times[1].name, "countdown");
    assert!(close(md.times[1].sections[0].duration, 0.52));
    assert!(close(md.times[1].sections[0].start, 11.16));
    assert_eq!(md.timeseries[0].name, "Trajectory 1");
    assert_eq!(md.timeseries[0].data[0].1[0], PointF::new(-181.0, -113.0));
}

#[test]
fn units_and_header_come_from_the_document() {
    let loaded = load_case("complete");
    let md = collect_measured_data(&loaded.metadata, &loaded.prefs);
    assert_eq!(md.full_path, "complete.mp4");
    assert_eq!(md.original_filename, "complete");
    assert_eq!((md.image_size.width, md.image_size.height), (848, 480));
    assert!(close(md.capture_framerate, 25.0) && close(md.video_framerate, 25.0));
    assert_eq!(
        (
            md.units.time_symbol,
            md.units.length_symbol,
            md.units.angle_symbol
        ),
        ("s", "px", "°")
    );
}

#[test]
fn keyframes_outside_the_selection_contribute_nothing() {
    let mut loaded = load_case("complete");
    loaded.metadata.video.selection_start = 0;
    loaded.metadata.video.selection_end = 0;
    let md = collect_measured_data(&loaded.metadata, &loaded.prefs);
    assert!(md.keyframes.is_empty());
    assert!(md.positions.is_empty() && md.distances.is_empty() && md.angles.is_empty());
    assert_eq!(
        md.times.len(),
        2,
        "stopwatches do not depend on the selection"
    );
}

#[test]
fn unnamed_keyframe_uses_the_timecode_builder() {
    let mut loaded = load_case("simple");
    loaded.metadata.keyframes[0].name.clear();
    loaded.metadata.timecode = |t| format!("tc{t}");
    let md = collect_measured_data(&loaded.metadata, &loaded.prefs);
    assert_eq!(
        md.keyframes[0].name,
        format!("tc{}", loaded.metadata.keyframes[0].timestamp)
    );
}
