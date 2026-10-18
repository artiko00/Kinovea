//! `Preferences.xml` document reader and writer (port of `PreferencesManager.ReadXML/WriteXML`).

use super::dom::{Element, XmlError};
use super::helpers::XmlField;
use super::Preferences;
use std::path::Path;

/// The only file format version produced and understood.
pub const FORMAT_VERSION: &str = "3.0";

/// Errors raised while loading or saving the preferences file.
#[derive(Debug, thiserror::Error)]
pub enum PreferencesError {
    #[error("cannot access {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{path}: {source}")]
    Xml {
        path: String,
        #[source]
        source: XmlError,
    },
    #[error("{path}: root element is {found}, expected KinoveaPreferences")]
    NotPreferences { path: String, found: String },
}

impl Preferences {
    /// Builds the whole document. Unknown sections are not preserved; `Keyboard` is.
    pub fn to_element(&self) -> Element {
        let mut root = Element::new("KinoveaPreferences");
        root.push_leaf("FormatVersion", FORMAT_VERSION);
        root.children.extend(self.general.write("General"));
        root.children
            .extend(self.file_explorer.write("FileExplorer"));
        root.children.extend(self.player.write("Player"));
        root.children.extend(self.capture.write("Capture"));
        if let Some(k) = &self.keyboard {
            root.push(k.clone());
        }
        root
    }

    /// Reads the sections found under the root, keeping defaults for anything missing and
    /// ignoring unknown elements.
    pub fn from_element(root: &Element) -> Self {
        let mut prefs = Self::default();
        for c in &root.children {
            match c.name.as_str() {
                "General" => prefs.general.read(c),
                "FileExplorer" => prefs.file_explorer.read(c),
                "Player" => prefs.player.read(c),
                "Capture" => prefs.capture.read(c),
                "Keyboard" => prefs.keyboard = Some(c.clone()),
                _ => {}
            }
        }
        prefs
    }

    /// Parses a document; the root must be `KinoveaPreferences`.
    pub fn from_xml(xml: &str, path: &str) -> Result<Self, PreferencesError> {
        let root = Element::parse(xml).map_err(|source| PreferencesError::Xml {
            path: path.to_owned(),
            source,
        })?;
        if root.name != "KinoveaPreferences" {
            return Err(PreferencesError::NotPreferences {
                path: path.to_owned(),
                found: root.name,
            });
        }
        Ok(Self::from_element(&root))
    }

    /// Serializes the document text.
    pub fn to_xml(&self) -> String {
        self.to_element().to_document()
    }

    /// Loads the file, or returns the defaults when it does not exist.
    pub fn load(path: &Path) -> Result<Self, PreferencesError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path).map_err(|source| PreferencesError::Io {
            path: path.display().to_string(),
            source,
        })?;
        Self::from_xml(&text, &path.display().to_string())
    }

    /// Writes the file. This is the only operation that touches the disk.
    pub fn save(&self, path: &Path) -> Result<(), PreferencesError> {
        std::fs::write(path, self.to_xml()).map_err(|source| PreferencesError::Io {
            path: path.display().to_string(),
            source,
        })
    }
}
