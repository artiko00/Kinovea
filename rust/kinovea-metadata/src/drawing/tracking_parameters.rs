//! Template-matching tracker settings persisted with tracks and trackable points
//! (`TrackingParameters`, `HSVRange`).

use kinovea_core::preferences::dom::Element;
use kinovea_core::units_precision::format_f32_default;
use kinovea_core::xml::{parse_boolean, parse_size, write_boolean, write_size};
use kinovea_geometry::Size;

crate::text_enum!(TrackingAlgorithm [Correlation, Circle]);

#[derive(Debug, Clone, PartialEq)]
pub struct HsvRange {
    pub hue: (f32, f32),
    pub saturation: (f32, f32),
    pub value: (f32, f32),
}

impl Default for HsvRange {
    fn default() -> Self {
        HsvRange {
            hue: (0.0, 360.0),
            saturation: (0.0, 255.0),
            value: (0.0, 255.0),
        }
    }
}

impl HsvRange {
    pub fn parse(text: &str) -> HsvRange {
        let v: Vec<f32> = text
            .split(';')
            .filter_map(|p| p.trim().parse().ok())
            .collect();
        if v.len() != 6 {
            return HsvRange {
                hue: (0.0, 0.0),
                saturation: (0.0, 0.0),
                value: (0.0, 0.0),
            };
        }
        HsvRange {
            hue: (v[0], v[1]),
            saturation: (v[2], v[3]),
            value: (v[4], v[5]),
        }
    }

    pub fn write(&self) -> String {
        [
            self.hue.0,
            self.hue.1,
            self.saturation.0,
            self.saturation.1,
            self.value.0,
            self.value.1,
        ]
        .iter()
        .map(|v| format_f32_default(*v))
        .collect::<Vec<_>>()
        .join(";")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackingParameters {
    pub algorithm: TrackingAlgorithm,
    pub search_window: Size,
    pub block_window: Size,
    pub similarity_threshold: f64,
    pub template_update_threshold: f64,
    pub use_mask: bool,
    pub hsv_range: HsvRange,
    pub dilate: i32,
    pub erode: i32,
}

impl Default for TrackingParameters {
    fn default() -> Self {
        TrackingParameters {
            algorithm: TrackingAlgorithm::Correlation,
            search_window: Size::new(100, 100),
            block_window: Size::new(20, 20),
            similarity_threshold: 0.5,
            template_update_threshold: 0.8,
            use_mask: false,
            hsv_range: HsvRange::default(),
            dilate: 0,
            erode: 0,
        }
    }
}

impl TrackingParameters {
    pub fn read_kva(e: &Element) -> TrackingParameters {
        let mut p = TrackingParameters::default();
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "Algorithm" => p.algorithm = t.parse().unwrap_or(TrackingAlgorithm::Correlation),
                "SearchWindow" => {
                    let (w, h) = parse_size(t);
                    p.search_window = Size::new(w, h);
                }
                "BlockWindow" => {
                    let (w, h) = parse_size(t);
                    p.block_window = Size::new(w, h);
                }
                "SimilarityThreshold" => {
                    p.similarity_threshold = t.parse().unwrap_or(p.similarity_threshold)
                }
                "TemplateUpdateThreshold" => {
                    p.template_update_threshold = t.parse().unwrap_or(p.template_update_threshold)
                }
                "UseMask" => p.use_mask = parse_boolean(t),
                "HSVRange" => p.hsv_range = HsvRange::parse(t),
                "Dilate" => p.dilate = t.parse().unwrap_or(0),
                "Erode" => p.erode = t.parse().unwrap_or(0),
                _ => {}
            }
        }
        p
    }

    /// Children of the `TrackerParameters` element; thresholds are written as `float`.
    pub fn write_kva(&self) -> Vec<Element> {
        vec![
            Element::leaf("Algorithm", self.algorithm.name()),
            Element::leaf(
                "SearchWindow",
                &write_size(self.search_window.width, self.search_window.height),
            ),
            Element::leaf(
                "BlockWindow",
                &write_size(self.block_window.width, self.block_window.height),
            ),
            Element::leaf(
                "SimilarityThreshold",
                &format_f32_default(self.similarity_threshold as f32),
            ),
            Element::leaf(
                "TemplateUpdateThreshold",
                &format_f32_default(self.template_update_threshold as f32),
            ),
            Element::leaf("UseMask", &write_boolean(self.use_mask)),
            Element::leaf("HSVRange", &self.hsv_range.write()),
            Element::leaf("Dilate", &self.dilate.to_string()),
            Element::leaf("Erode", &self.erode.to_string()),
        ]
    }

    pub fn content_hash(&self) -> i32 {
        (self.algorithm as i32)
            ^ (self.search_window.width ^ self.search_window.height << 16)
            ^ (self.block_window.width ^ self.block_window.height << 16)
            ^ (self.similarity_threshold.to_bits() as i32)
            ^ (self.template_update_threshold.to_bits() as i32)
            ^ i32::from(self.use_mask)
            ^ self.dilate
            ^ self.erode << 8
    }
}
