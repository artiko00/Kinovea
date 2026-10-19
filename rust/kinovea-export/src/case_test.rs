use crate::case::load_case;
use kinovea_core::preferences::enums::ExportSpace;
use kinovea_geometry::Size;

#[test]
fn complete_case_loads_the_kva_with_its_context() {
    let loaded = load_case("complete");
    assert_eq!(loaded.metadata.video.image_size, Size::new(848, 480));
    assert_eq!(loaded.metadata.video.video_path, "complete.mp4");
    assert_eq!(loaded.metadata.keyframes.len(), 9);
    assert_eq!(loaded.prefs.export_space, ExportSpace::WorldSpace);
    assert_eq!(loaded.prefs.producer, "Kinovea.0.0");
}

#[test]
fn selection_covers_the_whole_document() {
    let loaded = load_case("minimal");
    assert_eq!(loaded.metadata.video.selection_start, i64::MIN);
    assert_eq!(loaded.metadata.video.selection_end, i64::MAX);
}
