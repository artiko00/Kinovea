use super::capture_folders::*;
use super::helpers::XmlField;

#[test]
fn default_capture_folders_are_rooted_in_documents() {
    let cfg = CapturePathConfiguration::default_in("/home/u/Documents");
    assert_eq!(cfg.capture_folders.len(), 2);
    assert_eq!(cfg.capture_folders[0].short_name, "Capture A");
    assert!(cfg.capture_folders[1].path.ends_with("Capture/Capture B"));
    assert_ne!(cfg.capture_folders[0].id, cfg.capture_folders[1].id);
    let e = cfg.write("CapturePathConfiguration").unwrap();
    assert_eq!(e.children[0].name, "CaptureFolders");
    assert_eq!(e.child_text("DefaultFileName"), Some("%dateb%-%time%"));
    assert!(documents_dir().ends_with("Documents"));
}
