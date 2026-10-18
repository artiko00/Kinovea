//! Trajectory of a tracked point (`DrawingTrack`): timed positions, labels and options.

use super::tracking_parameters::TrackingParameters;
use super::{Header, KvaDrawing, RefValue};
use crate::color::Color;
use crate::kva_io::*;
use crate::measure_label::{MeasureLabel, MeasureLabelType};
use crate::style::{StyleElements, StyleValue, TrackShape};
use kinovea_core::preferences::dom::Element;
use kinovea_core::timed_point::TimedPoint;
use kinovea_core::xml::{parse_boolean, write_boolean};
use kinovea_geometry::PointF;

/// Label attached to one of the positions.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyframeLabel {
    pub label: MeasureLabel,
    pub attach_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub header: Header,
    pub begin_timestamp: i64,
    pub is_visible: bool,
    pub visible: i64,
    pub invisible: i64,
    pub measure_label_type: MeasureLabelType,
    pub parameters: TrackingParameters,
    pub positions: Vec<TimedPoint>,
    pub main_label: MeasureLabel,
    pub keyframe_labels: Vec<KeyframeLabel>,
    pub see_future: bool,
    pub show_track_label: bool,
    pub show_keyframe_labels: bool,
    pub use_keyframe_colors: bool,
    pub is_interactive_track: bool,
    pub show_rotation_circle: bool,
}

impl Track {
    /// Style keys of the original constructor: colour, track shape, line size, point size, label size.
    pub fn default_style(color: Color) -> StyleElements {
        StyleElements::new(vec![
            ("color", StyleValue::Color(color)),
            ("track shape", StyleValue::TrackShape(TrackShape::SOLID)),
            ("line size", StyleValue::LineSize(3)),
            ("TrackPointSize", StyleValue::PenSize(3)),
            ("label size", StyleValue::FontSize(8)),
        ])
    }

    pub fn new(positions: Vec<TimedPoint>) -> Self {
        let begin = positions.first().map_or(0, |p| p.t);
        Track {
            header: Header::new("", Track::default_style(Color::rgb(255, 0, 0))),
            begin_timestamp: begin,
            is_visible: true,
            visible: begin,
            invisible: i64::MAX,
            measure_label_type: MeasureLabelType::None,
            parameters: TrackingParameters::default(),
            positions,
            main_label: MeasureLabel::new(PointF::EMPTY),
            keyframe_labels: Vec::new(),
            see_future: true,
            show_track_label: false,
            show_keyframe_labels: true,
            use_keyframe_colors: true,
            is_interactive_track: false,
            show_rotation_circle: false,
        }
    }

    /// Index of the position whose time is closest to `timestamp` (first on ties).
    pub fn closest_point(&self, timestamp: i64) -> usize {
        let mut best = (i64::MAX, 0);
        for (i, p) in self.positions.iter().enumerate() {
            let err = (p.t - timestamp).abs();
            if err < best.0 {
                best = (err, i);
            }
        }
        best.1
    }

    /// `UpdateKeyframeLabels`: one label per keyframe strictly after the start and up to the
    /// last position; existing labels at those times are kept, the others are dropped.
    pub fn update_keyframe_labels(&mut self, keyframes: &[i64]) {
        if !self.is_visible {
            return;
        }
        let Some(last) = self.positions.last().map(|p| p.t) else {
            return;
        };
        let mut matched = vec![false; self.keyframe_labels.len()];
        for &t in keyframes
            .iter()
            .filter(|t| **t > self.begin_timestamp && **t <= last)
        {
            match self
                .keyframe_labels
                .iter()
                .position(|k| k.label.timestamp == t)
            {
                Some(i) => matched[i] = true,
                None => {
                    let index = self.closest_point(t);
                    let point = self.point_at(index);
                    let mut label = MeasureLabel::new(point);
                    label.timestamp = self.positions[index].t;
                    self.keyframe_labels.push(KeyframeLabel {
                        label,
                        attach_index: index,
                    });
                    matched.push(true);
                }
            }
        }
        let mut i = 0;
        self.keyframe_labels.retain(|_| {
            i += 1;
            matched[i - 1]
        });
    }

    fn point_at(&self, index: usize) -> PointF {
        self.positions
            .get(index)
            .map_or(PointF::EMPTY, |p| PointF::new(p.x, p.y))
    }
}

impl KvaDrawing for Track {
    const KVA_NAME: &'static str = "Track";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> Self {
        let mut d = Track::new(Vec::new());
        d.header.read_attributes(e);
        let mut labels: Vec<MeasureLabel> = Vec::new();
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "TimePosition" => d.begin_timestamp = map(parse_timestamp(t)),
                "IsVisible" => d.is_visible = parse_boolean(t),
                "Visible" => d.visible = map(parse_timestamp(t)),
                "Invisible" => d.invisible = read_open_timestamp(t, map),
                "ExtraData" => d.measure_label_type = parse_enum(t, MeasureLabelType::None),
                "TrackerParameters" => d.parameters = TrackingParameters::read_kva(c),
                "TrackPointList" => {
                    d.positions = c
                        .children
                        .iter()
                        .filter(|p| p.name == "TrackPoint")
                        .filter_map(|p| p.text.trim().parse::<TimedPoint>().ok())
                        .map(|p| TimedPoint {
                            x: p.x * scale.x,
                            y: p.y * scale.y,
                            t: map(p.t),
                            r: p.r * scale.x,
                        })
                        .collect();
                }
                "DrawingStyle" => d.header.style.import_kva(c),
                "MainLabel" => d.main_label = MeasureLabel::read_kva(c, scale),
                "KeyframeLabelList" => {
                    labels = c
                        .children
                        .iter()
                        .filter(|l| l.name == "KeyframeLabel")
                        .map(|l| MeasureLabel::read_kva(l, scale))
                        .collect()
                }
                "SeeFuture" => d.see_future = parse_boolean(t),
                "ShowTrackLabel" => d.show_track_label = parse_boolean(t),
                "ShowKeyframeLabels" => d.show_keyframe_labels = parse_boolean(t),
                "UseKeyframeColors" => d.use_keyframe_colors = parse_boolean(t),
                "IsInteractiveTrack" => d.is_interactive_track = parse_boolean(t),
                "DisplayBestFitCircle" | "ShowRotationCircle" => {
                    d.show_rotation_circle = parse_boolean(t)
                }
                _ => {}
            }
        }
        if !d.positions.is_empty() {
            for mut label in labels {
                let index = d.closest_point(label.timestamp);
                label.set_attach(d.point_at(index));
                d.keyframe_labels.push(KeyframeLabel {
                    label,
                    attach_index: index,
                });
            }
            d.main_label.set_attach(d.point_at(0));
        }
        d.visible = d.visible.max(0);
        d.invisible = d.invisible.max(0);
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            out.push(Element::leaf(
                "TimePosition",
                &self.begin_timestamp.to_string(),
            ));
            out.push(Element::leaf("IsVisible", &write_boolean(self.is_visible)));
            out.push(Element::leaf(
                "Visible",
                &write_open_timestamp(self.visible),
            ));
            out.push(Element::leaf(
                "Invisible",
                &write_open_timestamp(self.invisible),
            ));
            out.push(Element::leaf("ExtraData", self.measure_label_type.name()));
            let mut params = Element::new("TrackerParameters");
            params.children = self.parameters.write_kva();
            out.push(params);
            let mut list = Element::new("TrackPointList");
            list.attributes
                .push(("Count".into(), self.positions.len().to_string()));
            list.children = self
                .positions
                .iter()
                .map(|p| Element::leaf("TrackPoint", &p.to_string()))
                .collect();
            out.push(list);
            let mut main = Element::new("MainLabel");
            main.attributes
                .push(("Text".into(), self.header.name.clone()));
            main.children = self.main_label.write_kva_attached(self.point_at(0));
            out.push(main);
            if !self.keyframe_labels.is_empty() {
                let mut list = Element::new("KeyframeLabelList");
                list.attributes
                    .push(("Count".into(), self.keyframe_labels.len().to_string()));
                for kfl in &self.keyframe_labels {
                    let mut e = Element::new("KeyframeLabel");
                    e.children = kfl
                        .label
                        .write_kva_attached(self.point_at(kfl.attach_index));
                    list.push(e);
                }
                out.push(list);
            }
            for (name, v) in [
                ("SeeFuture", self.see_future),
                ("ShowTrackLabel", self.show_track_label),
                ("ShowKeyframeLabels", self.show_keyframe_labels),
                ("UseKeyframeColors", self.use_keyframe_colors),
                ("IsInteractiveTrack", self.is_interactive_track),
                ("ShowRotationCircle", self.show_rotation_circle),
            ] {
                out.push(Element::leaf(name, &write_boolean(v)));
            }
        }
        if filter.style {
            out.push(style_block(&self.header.style));
        }
        out
    }

    fn content_hash(&self) -> i32 {
        let long = |v: i64| (v as i32) ^ ((v >> 32) as i32);
        let flags = i32::from(self.see_future)
            | i32::from(self.show_track_label) << 1
            | i32::from(self.show_keyframe_labels) << 2
            | i32::from(self.use_keyframe_colors) << 3
            | i32::from(self.is_interactive_track) << 4
            | i32::from(self.show_rotation_circle) << 5;
        long(self.visible)
            ^ long(self.invisible)
            ^ self.positions.iter().fold(0, |h, p| h ^ p.content_hash())
            ^ self.header.style.content_hash()
            ^ self.main_label.content_hash()
            ^ self.parameters.content_hash()
            ^ flags
    }
}
