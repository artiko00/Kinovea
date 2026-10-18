//! `FileExplorer` preferences group (port of `FileExplorerPreferences.cs`).

use super::dom::Element;
use super::enums::{ExplorerThumbSize, FileSortAxis};
pub use super::explorer_items::{FavoriteFolder, FilePropertyVisibility, Recents};
use super::helpers::XmlField;

#[derive(Debug, Clone, PartialEq)]
pub struct FileExplorerPreferences {
    pub max_recent_files: i32,
    pub max_recent_captured_files: i32,
    pub recent_files: Recents,
    pub recent_watchers: Recents,
    pub recent_captured_files: Recents,
    pub favorite_folders: Vec<FavoriteFolder>,
    pub thumbnail_size: ExplorerThumbSize,
    pub file_property_visibility: FilePropertyVisibility,
    pub last_replay_folder: String,
    pub file_sort_axis: FileSortAxis,
    pub file_sort_ascending: bool,
    pub use_drive_icons: bool,
}

impl Default for FileExplorerPreferences {
    fn default() -> Self {
        Self {
            max_recent_files: 10,
            max_recent_captured_files: 10,
            recent_files: Recents::default(),
            recent_watchers: Recents::default(),
            recent_captured_files: Recents::default(),
            favorite_folders: Vec::new(),
            thumbnail_size: ExplorerThumbSize::Medium,
            file_property_visibility: FilePropertyVisibility::default(),
            last_replay_folder: String::new(),
            file_sort_axis: FileSortAxis::Name,
            file_sort_ascending: true,
            use_drive_icons: true,
        }
    }
}

impl XmlField for FileExplorerPreferences {
    fn write(&self, name: &str) -> Option<Element> {
        let mut e = Element::new(name);
        let max = self.max_recent_files;
        e.children
            .extend(self.max_recent_files.write("MaxRecentFiles"));
        e.children
            .extend(self.recent_files.write("RecentFiles", "RecentFile", max));
        e.children.extend(
            self.recent_watchers
                .write("RecentWatchers", "RecentWatcher", max),
        );
        e.children.extend(
            self.max_recent_captured_files
                .write("MaxRecentCapturedFiles"),
        );
        e.children.extend(self.recent_captured_files.write(
            "RecentCapturedFiles",
            "RecentCapturedFile",
            self.max_recent_captured_files,
        ));
        if !self.favorite_folders.is_empty() {
            let mut shortcuts = Element::new("Shortcuts");
            for f in &self.favorite_folders {
                let mut s = Element::new("Shortcut");
                f.write_fields(&mut s);
                shortcuts.push(s);
            }
            e.push(shortcuts);
        }
        e.children
            .extend(self.thumbnail_size.write("ThumbnailSize"));
        e.children.extend(
            self.file_property_visibility
                .write("FilePropertyVisibility"),
        );
        e.children
            .extend(self.last_replay_folder.write("LastReplayFolder"));
        e.children.extend(self.file_sort_axis.write("FileSortAxis"));
        e.children
            .extend(self.file_sort_ascending.write("FileSortAscending"));
        e.children
            .extend(self.use_drive_icons.write("UseDriveIcons"));
        Some(e)
    }

    fn read(&mut self, element: &Element) {
        for c in &element.children {
            match c.name.as_str() {
                "MaxRecentFiles" => self.max_recent_files.read(c),
                "RecentFiles" => self.recent_files.read(c, "RecentFile"),
                "RecentWatchers" => self.recent_watchers.read(c, "RecentWatcher"),
                "MaxRecentCapturedFiles" => self.max_recent_captured_files.read(c),
                "RecentCapturedFiles" => self.recent_captured_files.read(c, "RecentCapturedFile"),
                "Shortcuts" => {
                    self.favorite_folders = c
                        .children
                        .iter()
                        .filter(|s| s.name == "Shortcut")
                        .map(|s| {
                            let mut f = FavoriteFolder::default();
                            f.read_fields(s);
                            f
                        })
                        .collect();
                }
                "ThumbnailSize" => self.thumbnail_size.read(c),
                "FilePropertyVisibility" => self.file_property_visibility.read(c),
                "LastReplayFolder" => self.last_replay_folder.read(c),
                "FileSortAxis" => self.file_sort_axis.read(c),
                "FileSortAscending" => self.file_sort_ascending.read(c),
                "UseDriveIcons" => self.use_drive_icons.read(c),
                _ => {}
            }
        }
    }
}
