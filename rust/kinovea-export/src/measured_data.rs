//! The measured-data model shared by every writer (`MeasuredData` and its entries).

use kinovea_core::preferences::enums::ExportSpace;
use kinovea_core::timecode::TimecodeFormat;
use kinovea_core::units::{time_abbreviation, LengthUnit};
use kinovea_geometry::{PointF, Size};
use kinovea_metadata::drawing::ChronoColumn;
use kinovea_metadata::Metadata;

/// Export options: the preferences the original reads, plus the producer written in the header.
#[derive(Debug, Clone, PartialEq)]
pub struct ExportPrefs {
    pub export_space: ExportSpace,
    pub timecode_format: TimecodeFormat,
    /// `Software.ApplicationName + "." + Software.Version`; the caller decides the version.
    pub producer: String,
}

/// Unit symbols written next to the values (`MeasuredDataUnits`, symbols only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitSymbols {
    pub time_symbol: &'static str,
    pub length_symbol: &'static str,
    pub angle_symbol: &'static str,
}

/// Symbols for the document: lengths are in world units only in world space.
pub fn unit_symbols(metadata: &Metadata, prefs: &ExportPrefs) -> UnitSymbols {
    let units = metadata.calibration.units();
    let length = match prefs.export_space {
        ExportSpace::WorldSpace => units.length,
        ExportSpace::ImageSpace => LengthUnit::Pixels,
    };
    UnitSymbols {
        time_symbol: time_abbreviation(prefs.timecode_format),
        length_symbol: length.abbreviation(),
        angle_symbol: units.angle.abbreviation(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasuredData {
    pub producer: String,
    pub full_path: String,
    pub original_filename: String,
    pub image_size: Size,
    pub capture_framerate: f32,
    pub video_framerate: f32,
    pub units: UnitSymbols,
    pub keyframes: Vec<KeyframeData>,
    pub positions: Vec<Position>,
    pub distances: Vec<Scalar>,
    pub angles: Vec<Scalar>,
    pub times: Vec<TimeEntry>,
    pub timeseries: Vec<Timeseries>,
}

/// A keyframe; `comment` is the stored RTF, converted to text by the writer that needs it.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyframeData {
    pub name: String,
    pub time: f32,
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub name: String,
    pub time: f32,
    pub point: PointF,
}

/// A distance or an angle.
#[derive(Debug, Clone, PartialEq)]
pub struct Scalar {
    pub name: String,
    pub time: f32,
    pub value: f32,
}

/// A stopwatch: its sections, whether a section has a tag, and the columns its table shows.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeEntry {
    pub name: String,
    pub sections: Vec<TimeSection>,
    pub has_tags: bool,
    pub visible_columns: Vec<ChronoColumn>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeSection {
    pub name: String,
    pub tag: String,
    pub start: f32,
    pub stop: f32,
    pub duration: f32,
    pub cumul: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Timeseries {
    pub name: String,
    pub times: Vec<f32>,
    /// One entry per tracked point, keyed as the original (`"0"` for a track).
    pub data: Vec<(String, Vec<PointF>)>,
}
