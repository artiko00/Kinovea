//! The annotation document: video context plus every container of drawings (`Metadata`).

use crate::color::Color;
use crate::drawing::Drawing;
use crate::keyframe::Keyframe;
use crate::multi::{NumberSequence, Spotlights};
use crate::singletons::{CoordinateSystem, TestGrid};
use crate::trackability::TrackabilityManager;
use kinovea_measurement::{CalibrationHelper, Units};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::Size;
use uuid::Uuid;

pub mod edit;
#[cfg(test)]
mod edit_test;
#[cfg(test)]
mod multi_edit_test;
mod query;
#[cfg(test)]
mod query_test;

crate::text_enum!(ImageAspectRatio [Auto, Force43, Force169, ForcedSquarePixels]);
pub use kinovea_core::image_options::{Demosaicing, ImageRotation};

/// Where a drawing lives (`AbstractDrawingManager.Id`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manager {
    Keyframe(Uuid),
    Chronos,
    Tracks,
    Singletons,
}

/// Facts about the video the annotations belong to.
#[derive(Debug, Clone, PartialEq)]
pub struct VideoContext {
    pub video_path: String,
    pub video_byte_length: i64,
    pub image_size: Size,
    pub average_timestamps_per_frame: f64,
    /// Timestamps per second, used with the baseline frame interval to remap imported times.
    pub average_timestamps_per_second: f64,
    pub baseline_frame_interval_ms: f64,
    pub first_timestamp: i64,
    pub selection_start: i64,
    pub selection_end: i64,
    pub time_origin: i64,
    pub capture_fps: f64,
    /// Capture over video frame rate; 1 unless set (`Metadata.HighSpeedFactor`, never saved in KVA).
    pub high_speed_factor: f64,
}

impl Default for VideoContext {
    fn default() -> Self {
        VideoContext { video_path: String::new(), video_byte_length: 0, image_size: Size::default(), average_timestamps_per_frame: 1.0, average_timestamps_per_second: 25.0, baseline_frame_interval_ms: 40.0, first_timestamp: 0, selection_start: 0, selection_end: 0, time_origin: 0, capture_fps: 25.0, high_speed_factor: 1.0 }
    }
}

#[derive(Debug)]
pub struct Metadata {
    pub video: VideoContext,
    pub global_title: String,
    pub image_aspect: ImageAspectRatio,
    pub image_rotation: ImageRotation,
    pub mirrored: bool,
    pub demosaicing: Demosaicing,
    pub deinterlacing: bool,
    pub stabilization_track: Uuid,
    pub background_color: Color,
    pub context_string: String,
    /// Spatial calibration; reset for the image size when the context changes.
    pub calibration: CalibrationHelper,
    /// Sorted by timestamp.
    pub keyframes: Vec<Keyframe>,
    pub chronos: Vec<Drawing>,
    pub tracks: Vec<Drawing>,
    pub spotlights: Spotlights,
    pub number_sequence: NumberSequence,
    pub coordinate_system: CoordinateSystem,
    pub test_grid: TestGrid,
    pub trackability: TrackabilityManager,
    /// `VideoFilters` element (attributes and children) kept verbatim.
    pub video_filters: Element,
    /// `CameraMotion` element kept verbatim when present.
    pub camera_motion: Option<Element>,
    /// Formats a timestamp for display (`TimeCodeBuilder`); unnamed keyframes use it.
    pub timecode: fn(i64) -> String,
}

/// Default timecode: the raw timestamp, as the oracle's builder.
pub fn timestamp_text(timestamp: i64) -> String {
    timestamp.to_string()
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata { video: VideoContext::default(), global_title: String::new(), image_aspect: ImageAspectRatio::Auto, image_rotation: ImageRotation::Rotate0, mirrored: false, demosaicing: Demosaicing::None, deinterlacing: false, stabilization_track: Uuid::nil(), background_color: Color::argb(0, 255, 255, 255), context_string: String::new(), calibration: CalibrationHelper::without_tracking(Size::default(), Units::default(), 25.0), keyframes: Vec::new(), chronos: Vec::new(), tracks: Vec::new(), spotlights: Spotlights::default(), number_sequence: NumberSequence::default(), coordinate_system: CoordinateSystem::default(), test_grid: TestGrid::default(), trackability: TrackabilityManager::default(), video_filters: Element::new("VideoFilters"), camera_motion: None, timecode: timestamp_text }
    }
}

impl Metadata {
    /// Metadata for a video of `image_size`: the calibration is reset for that size.
    pub fn new(image_size: Size) -> Self {
        let mut m = Metadata::default();
        m.set_image_size(image_size);
        m
    }

    /// `Metadata.HighSpeedFactor` setter: 0 is read as 1.
    pub fn set_high_speed_factor(&mut self, factor: f64) { self.video.high_speed_factor = if factor == 0.0 { 1.0 } else { factor }; }

    /// `Metadata.ImageSize` setter: re-initializes the calibration for the new size.
    pub fn set_image_size(&mut self, image_size: Size) {
        self.video.image_size = image_size;
        let fps = self.calibration.capture_fps();
        self.calibration = CalibrationHelper::without_tracking(image_size, Units::default(), fps);
    }

    /// Index of the keyframe at `timestamp`.
    pub fn keyframe_index(&self, timestamp: i64) -> Option<usize> {
        self.keyframes.iter().position(|k| k.timestamp == timestamp)
    }

    pub fn keyframe(&self, id: Uuid) -> Option<&Keyframe> {
        self.keyframes.iter().find(|k| k.id == id)
    }

    /// `MergeInsertKeyframe`: same id → update; same time → merge drawings; else insert sorted.
    pub fn merge_insert_keyframe(&mut self, keyframe: Keyframe) {
        if let Some(known) = self.keyframes.iter_mut().find(|k| k.id == keyframe.id) {
            known.timestamp = keyframe.timestamp;
            known.color = keyframe.color;
            known.name = keyframe.name;
            known.comments = keyframe.comments;
            for d in keyframe.drawings {
                match known.drawings.iter().position(|x| x.id() == d.id()) {
                    Some(i) => known.drawings[i] = d,
                    None => known.drawings.push(d),
                }
            }
            return;
        }
        for i in 0..self.keyframes.len() {
            if keyframe.timestamp < self.keyframes[i].timestamp {
                self.keyframes.insert(i, keyframe);
                return;
            }
            if keyframe.timestamp == self.keyframes[i].timestamp {
                for d in keyframe.drawings {
                    if !self.keyframes[i].drawings.iter().any(|x| x.id() == d.id()) {
                        self.keyframes[i].drawings.push(d);
                    }
                }
                return;
            }
        }
        self.keyframes.push(keyframe);
    }

    /// Registers every trackable drawing with its current points (the player does this when a
    /// drawing is added); the trackability manager then answers the writers' reference values.
    pub fn register_trackables(&mut self) {
        for d in self.keyframes.iter().flat_map(|k| k.drawings.iter()) {
            register_drawing(&mut self.trackability, d);
        }
        self.register_spotlights();
    }

    /// Spotlight items are trackable through their centre; their reference timestamp stays 0.
    pub fn register_spotlights(&mut self) {
        for s in &self.spotlights.items {
            self.trackability.add(s.id, crate::multi::Spotlight::KVA_NAME, 0, &[("o".to_owned(), s.center)]);
        }
    }

    pub fn attached_drawings(&self) -> impl Iterator<Item = &Drawing> {
        self.keyframes.iter().flat_map(|k| k.drawings.iter())
    }
}

/// Registers one drawing when its kind is trackable in the original (`ITrackable`).
pub fn register_drawing(trackability: &mut TrackabilityManager, d: &Drawing) {
    let points = d.trackable_points();
    if d.is_trackable() && !points.is_empty() {
        trackability.add(d.id(), &d.header().name, d.header().reference_timestamp, &points);
    }
}
