use super::dom::Element;
use super::enums::{ExplorerThumbSize, FileProperty, FileSortAxis};
use super::explorer::FileExplorerPreferences;
use super::helpers::XmlField;

#[test]
fn defaults_match_the_original() {
    let p = FileExplorerPreferences::default();
    assert_eq!(p.max_recent_files, 10);
    assert_eq!(p.max_recent_captured_files, 10);
    assert_eq!(p.thumbnail_size, ExplorerThumbSize::Medium);
    assert_eq!(p.file_sort_axis, FileSortAxis::Name);
    assert!(p.file_sort_ascending && p.use_drive_icons);
    let names: Vec<_> = p
        .file_property_visibility
        .visible
        .iter()
        .map(|(k, _)| *k)
        .collect();
    assert_eq!(
        names,
        [
            FileProperty::Size,
            FileProperty::Framerate,
            FileProperty::Duration,
            FileProperty::CreationTime,
            FileProperty::HasKva
        ]
    );
    assert!(!p.file_property_visibility.visible[3].1);
}

#[test]
fn empty_lists_are_omitted_and_recents_are_capped() {
    let mut p = FileExplorerPreferences::default();
    let written = p.write("FileExplorer").unwrap();
    assert!(written.child("RecentFiles").is_none());
    assert!(written.child("Shortcuts").is_none());
    assert_eq!(written.child("LastReplayFolder").unwrap().text, "");

    p.max_recent_files = 2;
    p.recent_files.items = vec!["a".into(), "b".into(), "c".into()];
    let written = p.write("FileExplorer").unwrap();
    assert_eq!(written.child("RecentFiles").unwrap().children.len(), 2);
}

#[test]
fn reads_lists_and_shortcuts() {
    let xml = "<FileExplorer><RecentFiles><RecentFile>x.mp4</RecentFile><Other/></RecentFiles>\
<Shortcuts><Shortcut><FriendlyName>Vids</FriendlyName><Location>/v</Location></Shortcut></Shortcuts>\
<FilePropertyVisibility><CreationTime>true</CreationTime><IsImage>true</IsImage></FilePropertyVisibility></FileExplorer>";
    let mut p = FileExplorerPreferences::default();
    p.read(&Element::parse(xml).unwrap());
    assert_eq!(p.recent_files.items, ["x.mp4"]);
    assert_eq!(p.favorite_folders[0].location, "/v");
    assert!(p.file_property_visibility.visible[3].1);
    assert_eq!(p.file_property_visibility.visible.len(), 5);
}
