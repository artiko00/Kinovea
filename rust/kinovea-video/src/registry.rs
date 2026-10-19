//! Port of `Kinovea.Video/VideoTypeManager.cs`: maps extensions to reader factories. The C#
//! reflection scan becomes explicit `register` calls. Keys are lower-cased and keep their dot
//! (`.gif`), `*` is the wildcard reader, and the first registration of an extension wins as in
//! `ProcessType`.

use crate::reader::VideoReader;
use crate::wildcard::wildcard_match;
use std::cmp::Reverse;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Builds a fresh reader. The C# registry stores the type and instantiates it on demand.
pub type ReaderFactory = Box<dyn Fn() -> Box<dyn VideoReader>>;

/// Splits a `SupportedExtensions` attribute value on `;`, tab and space. Entries are lower-cased.
pub fn parse_extensions(list: &str) -> Vec<String> {
    list.split([';', '\t', ' '])
        .filter(|ext| !ext.is_empty())
        .map(str::to_lowercase)
        .collect()
}

#[derive(Default)]
pub struct ReaderRegistry {
    entries: Vec<(Vec<String>, ReaderFactory)>,
}

impl ReaderRegistry {
    /// `ProcessType`: registers `factory` for the listed extensions.
    pub fn register(&mut self, extensions: &str, factory: ReaderFactory) {
        self.entries.push((parse_extensions(extensions), factory));
    }

    /// The factory registered for `extension` itself, ignoring the wildcard.
    fn exact(&self, extension: &str) -> Option<&ReaderFactory> {
        let key = extension.to_lowercase();
        self.entries
            .iter()
            .find(|(extensions, _)| extensions.contains(&key))
            .map(|(_, factory)| factory)
    }

    /// `IsSupported`: true when the extension is a key, the wildcard key included (`ContainsKey`).
    pub fn is_supported(&self, extension: &str) -> bool {
        !extension.trim().is_empty() && self.exact(extension).is_some()
    }

    /// `GetVideoReader`: the reader for the extension, or the wildcard reader.
    pub fn reader_for(&self, extension: &str) -> Option<Box<dyn VideoReader>> {
        self.exact(extension)
            .or_else(|| self.exact("*"))
            .map(|factory| factory())
    }

    /// `GetImageSequenceReader`: the wildcard reader.
    pub fn image_sequence_reader(&self) -> Option<Box<dyn VideoReader>> {
        self.exact("*").map(|factory| factory())
    }

    /// `GetMostRecentSupportedVideo`: the newest supported file whose name matches the file-name
    /// part of `pattern`, looked up in the directory part. The result is absolute, as `FullName`.
    /// `GetFiles` promises no order, so the candidates are sorted by name first: with equal write
    /// times the first name wins, as `OrderByDescending(...).FirstOrDefault()` does.
    pub fn most_recent_supported(&self, pattern: &str) -> Option<PathBuf> {
        let pattern = Path::new(pattern);
        let name_pattern = pattern.file_name()?.to_string_lossy().into_owned();
        let dir = match pattern.parent() {
            Some(dir) if !dir.as_os_str().is_empty() => dir,
            _ => Path::new("."),
        };

        let mut candidates: Vec<(SystemTime, PathBuf)> = fs::read_dir(dir)
            .ok()?
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
            .filter(|entry| wildcard_match(&name_pattern, &entry.file_name().to_string_lossy()))
            .filter(|entry| self.is_supported(&dotted_extension(&entry.path())))
            .filter_map(|entry| Some((entry.metadata().ok()?.modified().ok()?, entry.path())))
            .collect();
        candidates.sort_by(|a, b| a.1.cmp(&b.1));
        let (_, path) = candidates
            .into_iter()
            .min_by_key(|(modified, _)| Reverse(*modified))?;
        std::path::absolute(path).ok()
    }
}

/// The extension with its dot, as `FileInfo.Extension` reports it; empty when there is none.
fn dotted_extension(path: &Path) -> String {
    path.extension()
        .map(|ext| format!(".{}", ext.to_string_lossy()))
        .unwrap_or_default()
}
