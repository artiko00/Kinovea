//! KVA 2.x reader (`MetadataSerializer.Load`).

use super::ImportContext;
use crate::color::Color;
use crate::drawing::Drawing;
use crate::keyframe::Keyframe;
use crate::kva_io::{identity, parse_enum, Scale};
use crate::metadata::{register_drawing, Metadata};
use crate::migration::{descendants, migrate_legacy, needs_migration};
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, parse_size};
use kinovea_geometry::{PointF, Size};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KvaError {
    Xml(String),
    NotKva(String),
}

impl fmt::Display for KvaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KvaError::Xml(e) => write!(f, "invalid KVA xml: {e}"),
            KvaError::NotKva(root) => write!(f, "not a KVA document: root is {root}"),
        }
    }
}

impl std::error::Error for KvaError {}

/// Reads a 2.x document into `m`, which must already describe the current video.
pub fn read_kva(m: &mut Metadata, xml: &str) -> Result<ImportContext, KvaError> {
    let mut root = Element::parse(xml).map_err(|e| KvaError::Xml(e.to_string()))?;
    if root.name != "KinoveaVideoAnalysis" {
        return Err(KvaError::NotKva(root.name));
    }
    let mut relative_trajectories = false;
    if format_version(&root).is_some_and(needs_migration) {
        root = migrate_legacy(&root);
        relative_trajectories = true;
    }
    let mut ctx = ImportContext::default();
    let mut scale = Scale::IDENTITY;
    let mut same: Option<bool> = None;
    let mut same_context = |m: &Metadata, ctx: &ImportContext| *same.get_or_insert_with(|| ctx.is_same_context(&m.video));
    for c in &root.children {
        let t = c.text.trim();
        match c.name.as_str() {
            "OriginalFilename" => ctx.file_name = c.text.clone(),
            "FullPath" => {
                ctx.path = c.text.clone();
                if m.video.video_path.is_empty() && !ctx.path.is_empty() {
                    m.video.video_path = ctx.path.clone();
                }
            }
            "ByteLength" => ctx.byte_length = t.parse().unwrap_or(0),
            "CaptureRecording" => ctx.is_capture_recording = parse_boolean(t),
            "GlobalTitle" => m.global_title = c.text.clone(),
            "ImageSize" => {
                let (w, h) = parse_size(t);
                ctx.image_size = Size::new(w, h);
                scale = ctx.scaling(&m.video);
            }
            "Aspect" => m.image_aspect = parse_enum(t, m.image_aspect),
            "Rotation" => m.image_rotation = parse_enum(t, m.image_rotation),
            "Mirror" => m.mirrored = parse_boolean(t),
            "Demosaicing" => m.demosaicing = parse_enum(t, m.demosaicing),
            "Deinterlacing" => m.deinterlacing = parse_boolean(t),
            "StabilizationTrack" => m.stabilization_track = crate::ids::parse_id(t).unwrap_or_default(),
            "BackgroundColor" => m.background_color = Color::parse(t, Color::argb(0, 0, 0, 0)),
            "AverageTimeStampsPerFrame" => ctx.average_timestamps_per_frame = t.parse().unwrap_or(0.0),
            "FirstTimeStamp" => ctx.first_timestamp = t.parse().unwrap_or(0),
            "CaptureFramerate" => m.calibration.set_capture_fps(t.parse().unwrap_or(25.0)),
            "SelectionStart" if same_context(m, &ctx) => m.video.selection_start = t.parse().unwrap_or(0),
            "SelectionEnd" if same_context(m, &ctx) => m.video.selection_end = t.parse().unwrap_or(0),
            "TimeOrigin" => ctx.time_origin = t.parse().unwrap_or(0),
            "Context" => m.context_string = c.text.clone(),
            "Calibration" => m.calibration.read_xml(c, PointF::new(scale.x, scale.y), ctx.image_size),
            "Keyframes" => {
                let is_same = same_context(m, &ctx);
                let video = m.video.clone();
                let remap = |ts: i64| ctx.remap(ts, &video, is_same);
                for k in c.children.iter().filter(|k| k.name == "Keyframe") {
                    let mut keyframe = Keyframe::read_kva(k, scale, &remap);
                    m.name_untitled(&mut keyframe.drawings);
                    register_keyframe(m, &keyframe);
                    m.merge_insert_keyframe(keyframe);
                }
            }
            "Tracks" => {
                let is_same = same_context(m, &ctx);
                let video = m.video.clone();
                let remap = |ts: i64| ctx.remap(ts, &video, is_same);
                let tracks: Vec<Drawing> = c.children.iter().filter_map(|d| Drawing::read_kva(d, scale, &remap)).filter(|d| matches!(d, Drawing::Track(_))).collect();
                for mut d in tracks {
                    if !m.tracks.iter().any(|x| x.id() == d.id()) {
                        m.name_untitled(std::slice::from_mut(&mut d));
                        m.tracks.push(d);
                    }
                }
            }
            "Chronos" => {
                let chronos: Vec<Drawing> = c.children.iter().filter_map(|d| Drawing::read_kva(d, scale, &identity)).filter(Drawing::is_chrono_like).collect();
                for mut d in chronos {
                    if !m.chronos.iter().any(|x| x.id() == d.id()) {
                        m.name_untitled(std::slice::from_mut(&mut d));
                        m.chronos.push(d);
                    }
                }
            }
            "Spotlights" => {
                let is_same = same_context(m, &ctx);
                let video = m.video.clone();
                m.spotlights.read_kva(c, scale, &|ts| ctx.remap(ts, &video, is_same));
                m.register_spotlights();
            }
            "AutoNumbers" | "NumberSequence" => {
                let is_same = same_context(m, &ctx);
                let video = m.video.clone();
                m.number_sequence.read_kva(c, scale, &|ts| ctx.remap(ts, &video, is_same));
            }
            "CoordinateSystem" => m.coordinate_system.read_kva(c),
            "TestGrid" => m.test_grid.read_kva(c),
            "Trackability" => m.trackability.read_kva(c),
            "VideoFilters" => m.video_filters = c.clone(),
            "CameraMotion" => m.camera_motion = Some(c.clone()),
            _ => {}
        }
    }
    if same_context(m, &ctx) {
        m.video.time_origin = ctx.time_origin;
    } else if ctx.is_capture_recording {
        m.video.time_origin = ctx.remap_from(ctx.time_origin, ctx.first_timestamp, m.video.first_timestamp, &m.video);
    }
    // `FixRelativeTrajectories` runs before `AfterKVAImport`, so the keyframe labels see absolute times.
    if relative_trajectories {
        for t in m.tracks.iter_mut() {
            if let Drawing::Track(track) = t {
                track.fix_relative_trajectories();
            }
        }
    }
    after_import(m);
    Ok(ctx)
}

/// `descendant::FormatVersion` of the original: the first one in document order, as a number.
fn format_version(root: &Element) -> Option<f64> {
    descendants(root, "FormatVersion").first().and_then(|v| v.text.trim().parse().ok())
}

/// What the player does when a keyframe's drawings are added: register the trackable ones.
fn register_keyframe(m: &mut Metadata, keyframe: &Keyframe) {
    for d in &keyframe.drawings {
        register_drawing(&mut m.trackability, d);
    }
}

/// `AfterKVAImport`. `Assign`: every trackable drawing takes its current name and points as
/// reference values. `UpdateOrigin`: the coordinate system origin comes from the calibration, and
/// its tracker follows. `CleanUnassigned`: trackers without a trackable drawing are dropped.
fn after_import(m: &mut Metadata) {
    m.update_trajectories_for_keyframes();
    for d in m.keyframes.iter().flat_map(|k| k.drawings.iter()) {
        register_drawing(&mut m.trackability, d);
    }
    m.register_spotlights();
    m.coordinate_system.origin = m.calibration.origin();
    let cs = &m.coordinate_system;
    if m.trackability.tracker(cs.header.id).is_some() {
        m.trackability.add(cs.header.id, &cs.header.name, 0, &cs.trackable_points());
    }
    let mut live: Vec<uuid::Uuid> = m
        .attached_drawings()
        .filter(|d| d.is_trackable() && !d.trackable_points().is_empty())
        .map(|d| d.id())
        .chain(m.spotlights.items.iter().map(|s| s.id))
        .collect();
    live.push(m.coordinate_system.header.id);
    m.trackability.trackers.retain(|t| live.contains(&t.drawing_id));
}
