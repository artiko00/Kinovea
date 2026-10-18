//! Fading and tracking defaults of the `Player` group (ports of `InfosFading`, `HSVRange`,
//! `TrackingParameters`).

use super::dom::Element;
use super::enums::TrackingAlgorithm;
use super::fields::xml_struct;
use super::helpers::{Size, XmlField};
use crate::units_precision::format_f32_default;

#[derive(Debug, Clone, PartialEq)]
pub struct InfosFading {
    pub enabled: bool,
    pub use_default: bool,
    pub always_visible: bool,
    pub master_factor: f32,
    pub opaque_frames: i32,
    pub fading_frames: i32,
}

impl Default for InfosFading {
    fn default() -> Self {
        Self {
            enabled: true,
            use_default: true,
            always_visible: false,
            master_factor: 1.0,
            opaque_frames: 1,
            fading_frames: 20,
        }
    }
}

xml_struct!(InfosFading {
    enabled => "Enabled", use_default => "UseDefault", always_visible => "AlwaysVisible",
    master_factor => "MasterFactor", opaque_frames => "OpaqueFrames", fading_frames => "Frames",
});

/// Hue, saturation and value bounds written as six `;`-separated floats.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsvRange {
    pub hue: (f32, f32),
    pub saturation: (f32, f32),
    pub value: (f32, f32),
}

impl Default for HsvRange {
    fn default() -> Self {
        Self {
            hue: (0.0, 360.0),
            saturation: (0.0, 255.0),
            value: (0.0, 255.0),
        }
    }
}

impl XmlField for HsvRange {
    fn write(&self, name: &str) -> Option<Element> {
        let v = [
            self.hue.0,
            self.hue.1,
            self.saturation.0,
            self.saturation.1,
            self.value.0,
            self.value.1,
        ];
        let text: Vec<String> = v.iter().map(|f| format_f32_default(*f)).collect();
        Some(Element::leaf(name, &text.join(";")))
    }
    fn read(&mut self, element: &Element) {
        let p: Vec<f32> = element
            .text
            .split(';')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if p.len() == 6 {
            *self = Self {
                hue: (p[0], p[1]),
                saturation: (p[2], p[3]),
                value: (p[4], p[5]),
            };
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackingParameters {
    pub algorithm: TrackingAlgorithm,
    pub search_window: Size,
    pub block_window: Size,
    pub similarity_threshold: f32,
    pub template_update_threshold: f32,
    pub use_mask: bool,
    pub hsv_range: HsvRange,
    pub dilate: i32,
    pub erode: i32,
}

impl Default for TrackingParameters {
    fn default() -> Self {
        Self {
            algorithm: TrackingAlgorithm::Correlation,
            search_window: Size {
                width: 100,
                height: 100,
            },
            block_window: Size {
                width: 20,
                height: 20,
            },
            similarity_threshold: 0.5,
            template_update_threshold: 0.8,
            use_mask: false,
            hsv_range: HsvRange::default(),
            dilate: 0,
            erode: 0,
        }
    }
}

xml_struct!(TrackingParameters {
    algorithm => "Algorithm", search_window => "SearchWindow", block_window => "BlockWindow",
    similarity_threshold => "SimilarityThreshold", template_update_threshold => "TemplateUpdateThreshold",
    use_mask => "UseMask", hsv_range => "HSVRange", dilate => "Dilate", erode => "Erode",
});
