//! `Capture` preferences group (port of `CapturePreferences.cs`). Camera descriptions
//! (`Cameras`) are kept as opaque XML and written back unchanged.

pub use super::capture_automation::{CaptureAutomationConfiguration, PhotofinishConfiguration};
pub use super::capture_folders::{documents_dir, CaptureFolder, CapturePathConfiguration};
use super::dom::Element;
use super::enums::*;
use super::fields::xml_struct;
use super::helpers::{Fixed3, XmlField};

/// Camera descriptions, kept verbatim (they embed icons and driver-specific blocks).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cameras(pub Vec<Element>);

impl XmlField for Cameras {
    fn write(&self, name: &str) -> Option<Element> {
        (!self.0.is_empty()).then(|| Element {
            name: name.to_owned(),
            children: self.0.clone(),
            ..Element::default()
        })
    }
    fn read(&mut self, element: &Element) {
        self.0 = element
            .children
            .iter()
            .filter(|c| c.name == "Camera")
            .cloned()
            .collect();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CapturePreferences {
    pub capture_path_configuration: CapturePathConfiguration,
    pub display_synchronization_framerate: Fixed3,
    pub recording_mode: CaptureRecordingMode,
    pub verbose_stats: bool,
    pub save_uncompressed_video: bool,
    pub encoding_quality: EncodingQuality,
    pub memory_buffer: i32,
    pub cameras: Cameras,
    pub photofinish: PhotofinishConfiguration,
    pub automation: CaptureAutomationConfiguration,
    pub enable_framerate_replacement: bool,
    pub highspeed_recording_framerate_threshold: Fixed3,
    pub highspeed_recording_framerate_output: Fixed3,
    pub slowspeed_recording_framerate_threshold: Fixed3,
    pub slowspeed_recording_framerate_output: Fixed3,
    pub export_flags: KvaExportFlags,
    pub capture_kva: String,
    pub context_enabled: bool,
    pub context_string: String,
}

impl Default for CapturePreferences {
    fn default() -> Self {
        Self {
            capture_path_configuration: CapturePathConfiguration::default(),
            display_synchronization_framerate: Fixed3(25.0),
            recording_mode: CaptureRecordingMode::Delay,
            verbose_stats: false,
            save_uncompressed_video: false,
            encoding_quality: EncodingQuality::Good,
            memory_buffer: 768,
            cameras: Cameras::default(),
            photofinish: PhotofinishConfiguration::default(),
            automation: CaptureAutomationConfiguration::default(),
            enable_framerate_replacement: true,
            highspeed_recording_framerate_threshold: Fixed3(150.0),
            highspeed_recording_framerate_output: Fixed3(30.0),
            slowspeed_recording_framerate_threshold: Fixed3(1.0),
            slowspeed_recording_framerate_output: Fixed3(30.0),
            export_flags: KvaExportFlags::DefaultCaptureRecording,
            capture_kva: String::new(),
            context_enabled: true,
            context_string: String::new(),
        }
    }
}

xml_struct!(CapturePreferences {
    capture_path_configuration => "CapturePathConfiguration",
    display_synchronization_framerate => "DisplaySynchronizationFramerate",
    recording_mode => "CaptureRecordingMode", verbose_stats => "VerboseStats",
    save_uncompressed_video => "SaveUncompressedVideo", encoding_quality => "EncodingQuality",
    memory_buffer => "MemoryBuffer", cameras => "Cameras", photofinish => "PhotofinishConfiguration",
    automation => "CaptureAutomationConfiguration", enable_framerate_replacement => "EnableFramerateReplacement",
    highspeed_recording_framerate_threshold => "HighspeedRecordingFramerateThreshold",
    highspeed_recording_framerate_output => "HighspeedRecordingFramerateOutput",
    slowspeed_recording_framerate_threshold => "SlowspeedRecordingFramerateThreshold",
    slowspeed_recording_framerate_output => "SlowspeedRecordingFramerateOutput",
    export_flags => "ExportFlags", capture_kva => "CaptureKVA", context_enabled => "ContextEnabled",
    context_string => "ContextString",
});
