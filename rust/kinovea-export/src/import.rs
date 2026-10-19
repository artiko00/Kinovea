//! Import entry point: picks the importer from the file name and extension (`MetadataSerializer.Load`),
//! and the types every importer shares: the report, the error and the posture tool lookup.

use crate::{openpose, srt, trc};
use kinovea_metadata::posture::catalog::{custom_postures, posture_by_id};
use kinovea_metadata::posture::GenericPosture;
use kinovea_metadata::style::{StyleElements, StyleValue};
use kinovea_metadata::{read_kva, Color, KvaError, Metadata};
use std::fmt;
use std::path::Path;

/// What an import merged into the document, and what it left out.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ImportReport {
    /// Keyframes merged into the document, including frames that only held skipped persons.
    pub keyframes: usize,
    pub drawings: usize,
    pub tracks: usize,
    /// File names or items that were not imported, in the order they were met.
    pub skipped: Vec<String>,
}

#[derive(Debug)]
pub enum ImportError {
    Io(std::io::Error),
    /// Malformed content; the message names the line or the file.
    InvalidData(String),
    /// The file name or extension is not a supported format.
    Unsupported(String),
    Kva(KvaError),
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::Io(e) => write!(f, "{e}"),
            ImportError::InvalidData(msg) => write!(f, "invalid data: {msg}"),
            ImportError::Unsupported(name) => write!(f, "unsupported file: {name}"),
            ImportError::Kva(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ImportError {}

impl From<std::io::Error> for ImportError {
    fn from(e: std::io::Error) -> Self {
        ImportError::Io(e)
    }
}

/// Imports `path` into `m`, choosing the importer by file name and extension (case-insensitive).
/// `.kva` and `.xml` files are read as KVA documents.
pub fn import_file(m: &mut Metadata, path: &Path) -> Result<ImportReport, ImportError> {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    if name.ends_with("_keypoints.json") {
        return openpose::import_openpose(m, path);
    }
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match extension.as_str() {
        "trc" => trc::import_trc(m, &read_text(path)?),
        "srt" => srt::import_srt(m, &read_text(path)?),
        "kva" | "xml" => {
            read_kva(m, &read_text(path)?).map_err(ImportError::Kva)?;
            Ok(ImportReport::default())
        }
        _ => Err(ImportError::Unsupported(name.to_owned())),
    }
}

/// UTF-8 text without its byte order mark.
fn read_text(path: &Path) -> Result<String, ImportError> {
    Ok(std::fs::read_to_string(path)?
        .trim_start_matches('\u{feff}')
        .to_owned())
}

/// A fresh instance of the shipped posture tool called `name` (`GenericPostureManager.Instanciate`).
pub(crate) fn posture_tool(name: &str) -> Result<GenericPosture, ImportError> {
    let id = custom_postures()
        .into_iter()
        .find(|p| p.name == name)
        .map(|p| p.id)
        .ok_or_else(|| ImportError::InvalidData(format!("missing posture tool {name}")))?;
    posture_by_id(id, true)
        .ok_or_else(|| ImportError::InvalidData(format!("missing posture tool {name}")))
}

/// Default style of the generic posture tools (`DrawingToolGenericPosture`): the line colour.
pub(crate) fn posture_style() -> StyleElements {
    StyleElements::new(vec![(
        "line color",
        StyleValue::Color(Color::argb(255, 0, 153, 153)),
    )])
}
