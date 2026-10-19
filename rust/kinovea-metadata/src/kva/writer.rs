//! KVA 2.1 writer (`MetadataSerializer.WriteXml`).

use super::PRODUCER;
use crate::drawing::RefValue;
use crate::kva_io::Filter;
use crate::metadata::Metadata;
use crate::singletons::{singleton_element, CoordinateSystem, TestGrid};
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{format_f64_invariant, write_boolean};
use kinovea_geometry::PointF;
use uuid::Uuid;

/// Which sections to write (`KVAExportFlags`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportFlags {
    pub drawings: bool,
    pub calibration: bool,
    pub video_specific: bool,
}

impl ExportFlags {
    pub const FULL: ExportFlags = ExportFlags { drawings: true, calibration: true, video_specific: true };
    pub const CAPTURE_RECORDING: ExportFlags = ExportFlags { drawings: true, calibration: true, video_specific: false };
    pub const DRAWINGS: ExportFlags = ExportFlags { drawings: true, calibration: false, video_specific: false };
    pub const CALIBRATION: ExportFlags = ExportFlags { drawings: false, calibration: true, video_specific: false };
    pub const VIDEO_SPECIFIC: ExportFlags = ExportFlags { drawings: false, calibration: false, video_specific: true };
}

/// Builds the document; `version` is the software version written in `Producer`.
pub fn write_kva(m: &Metadata, version: &str, is_capture_recording: bool, flags: ExportFlags) -> Element {
    let mut root = Element::new("KinoveaVideoAnalysis");
    general_information(&mut root, m, version, is_capture_recording);
    let refs = |id: Uuid, key: &str| -> PointF { m.trackability.reference_value(id, key) };
    if flags.calibration {
        let mut cal = Element::new("Calibration");
        cal.children = m.calibration.write_xml();
        root.push(cal);
    }
    if flags.drawings && !m.keyframes.is_empty() {
        let mut kfs = Element::new("Keyframes");
        kfs.children = m.keyframes.iter().map(|k| k.to_element(Filter::KVA, &refs, m.timecode)).collect();
        root.push(kfs);
    }
    if flags.video_specific {
        push_list(&mut root, "Chronos", &m.chronos, &refs);
        push_list(&mut root, "Tracks", &m.tracks, &refs);
        if !m.spotlights.items.is_empty() {
            let mut e = Element::new("Spotlights");
            e.children = m.spotlights.write_kva(Filter::KVA);
            root.push(e);
        }
        if !m.number_sequence.items.is_empty() {
            let mut e = Element::new("NumberSequence");
            e.children = m.number_sequence.write_kva(Filter::KVA);
            root.push(e);
        }
    }
    if flags.calibration {
        root.push(singleton_element(CoordinateSystem::KVA_NAME, &m.coordinate_system.header, m.coordinate_system.write_kva(Filter::KVA, &refs)));
    }
    if flags.drawings {
        root.push(singleton_element(TestGrid::KVA_NAME, &m.test_grid.header, m.test_grid.write_kva(Filter::KVA)));
    }
    if flags.video_specific {
        let mut trackability = Element::new("Trackability");
        trackability.children = m.trackability.write_kva();
        root.push(trackability);
        root.push(m.video_filters.clone());
        if let Some(motion) = &m.camera_motion {
            root.push(motion.clone());
        }
    }
    root
}

fn push_list(root: &mut Element, name: &str, drawings: &[crate::drawing::Drawing], refs: RefValue) {
    if drawings.is_empty() {
        return;
    }
    let mut e = Element::new(name);
    e.children = drawings.iter().map(|d| d.to_element(Filter::KVA, refs)).collect();
    root.push(e);
}

fn general_information(root: &mut Element, m: &Metadata, version: &str, is_capture_recording: bool) {
    let v = &m.video;
    root.push_leaf("FormatVersion", "2.1");
    root.push_leaf("Producer", &format!("{PRODUCER}.{version}"));
    if !is_capture_recording {
        root.push_leaf("OriginalFilename", &crate::kva::context::file_stem(&v.video_path));
        root.push_leaf("FullPath", &v.video_path);
        root.push_leaf("ByteLength", &v.video_byte_length.to_string());
    } else {
        root.push_leaf("CaptureRecording", &write_boolean(true));
    }
    if !m.global_title.is_empty() {
        root.push_leaf("GlobalTitle", &m.global_title);
    }
    root.push_leaf("ImageSize", &format!("{};{}", v.image_size.width, v.image_size.height));
    root.push_leaf("Aspect", m.image_aspect.name());
    root.push_leaf("Rotation", m.image_rotation.name());
    root.push_leaf("Mirror", &write_boolean(m.mirrored));
    root.push_leaf("Demosaicing", m.demosaicing.name());
    root.push_leaf("Deinterlacing", &write_boolean(m.deinterlacing));
    root.push_leaf("StabilizationTrack", &crate::ids::id_text(m.stabilization_track));
    root.push_leaf("BackgroundColor", &m.background_color.write(true));
    root.push_leaf("AverageTimeStampsPerFrame", &format_f64_invariant(v.average_timestamps_per_frame));
    root.push_leaf("CaptureFramerate", &format_f64_invariant(m.calibration.capture_fps()));
    root.push_leaf("VideoFramerate", &format_f64_invariant(1000.0 / v.baseline_frame_interval_ms));
    if !is_capture_recording {
        root.push_leaf("FirstTimeStamp", &v.first_timestamp.to_string());
        root.push_leaf("SelectionStart", &v.selection_start.to_string());
        root.push_leaf("SelectionEnd", &v.selection_end.to_string());
    }
    root.push_leaf("TimeOrigin", &v.time_origin.to_string());
    if !m.context_string.is_empty() {
        root.push_leaf("Context", &m.context_string);
    }
}
