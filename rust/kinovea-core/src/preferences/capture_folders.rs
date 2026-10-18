//! Capture destinations (ports of `CaptureFolder`, `CapturePathConfiguration`).

use super::dom::Element;
use super::enums::*;
use super::fields::xml_fields;
use super::helpers::XmlField;
use uuid::Uuid;

/// A destination folder for recordings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureFolder {
    pub id: Uuid,
    pub short_name: String,
    pub path: String,
}

xml_fields!(CaptureFolder { id => "Id", short_name => "ShortName", path => "Path" });

impl CaptureFolder {
    pub fn new(short_name: &str, path: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            short_name: short_name.to_owned(),
            path: path.to_owned(),
        }
    }
}

/// `~/Documents`, the base of the default capture folders (`SpecialFolder.MyDocuments`).
pub fn documents_dir() -> String {
    std::env::var("HOME")
        .map(|h| format!("{h}/Documents"))
        .unwrap_or_else(|_| "Documents".to_owned())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePathConfiguration {
    pub capture_folders: Vec<CaptureFolder>,
    pub default_file_name: String,
    pub image_format: ImageFormat,
    pub video_format: VideoContainer,
    pub capture_codec: CaptureCodec,
    pub encoding_quality: EncodingQuality,
}

impl CapturePathConfiguration {
    /// Defaults with the two capture folders rooted at `documents`.
    pub fn default_in(documents: &str) -> Self {
        let sep = std::path::MAIN_SEPARATOR;
        let root = format!("{documents}{sep}Capture");
        Self {
            capture_folders: vec![
                CaptureFolder::new("Capture A", &format!("{root}{sep}Capture A")),
                CaptureFolder::new("Capture B", &format!("{root}{sep}Capture B")),
            ],
            default_file_name: "%dateb%-%time%".to_owned(),
            image_format: ImageFormat::JPG,
            video_format: VideoContainer::MP4,
            capture_codec: CaptureCodec::MJPEG,
            encoding_quality: EncodingQuality::Good,
        }
    }
}

impl Default for CapturePathConfiguration {
    fn default() -> Self {
        Self::default_in(&documents_dir())
    }
}

xml_fields!(CapturePathConfiguration {
    default_file_name => "DefaultFileName", image_format => "ImageFormat", video_format => "VideoFormat",
    capture_codec => "CaptureCodec", encoding_quality => "EncodingQuality",
});

impl XmlField for CapturePathConfiguration {
    fn write(&self, name: &str) -> Option<Element> {
        let mut e = Element::new(name);
        let mut folders = Element::new("CaptureFolders");
        for f in &self.capture_folders {
            let mut cf = Element::new("CaptureFolder");
            f.write_fields(&mut cf);
            folders.push(cf);
        }
        e.push(folders);
        self.write_fields(&mut e);
        Some(e)
    }
    fn read(&mut self, element: &Element) {
        if let Some(list) = element.child("CaptureFolders") {
            self.capture_folders = list
                .children
                .iter()
                .filter(|c| c.name == "CaptureFolder")
                .map(|c| {
                    let mut f = CaptureFolder {
                        id: Uuid::nil(),
                        short_name: String::new(),
                        path: String::new(),
                    };
                    f.read_fields(c);
                    f
                })
                .collect();
        }
        self.read_fields(element);
    }
}
