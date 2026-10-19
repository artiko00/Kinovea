//! Template-matching tracker settings persisted with tracks and trackable points
//! (`TrackingParameters`, `HSVRange`).

use kinovea_core::preferences::dom::Element;
use kinovea_core::units_precision::format_f32_default;
use kinovea_core::xml::{parse_boolean, parse_size, write_boolean, write_size};
use kinovea_geometry::Size;

crate::text_enum!(TrackingAlgorithm [Correlation, Circle]);

pub mod hsv_range;
#[cfg(test)]
mod hsv_range_test;
pub use hsv_range::HsvRange;

/// `TrackingParameters.maxWindowSize`: both window sides are clamped to this.
const MAX_WINDOW_SIZE: i32 = 400;

#[derive(Debug, Clone, PartialEq)]
pub struct TrackingParameters {
    pub algorithm: TrackingAlgorithm,
    search_window: Size,
    block_window: Size,
    pub similarity_threshold: f64,
    pub template_update_threshold: f64,
    pub use_mask: bool,
    pub hsv_range: HsvRange,
    pub dilate: i32,
    pub erode: i32,
    /// Not stored in KVA, as in the original.
    pub reset_on_move: bool,
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
            reset_on_move: true,
        }
    }
}

impl TrackingParameters {
    pub fn search_window(&self) -> Size {
        self.search_window
    }

    pub fn block_window(&self) -> Size {
        self.block_window
    }

    /// `SearchWindow` setter: clamped, then grown to the block window.
    pub fn set_search_window(&mut self, size: Size) {
        self.search_window = clamp_window(size);
        self.fit_block_window();
    }

    /// `BlockWindow` setter: clamped, then the search window is grown to fit.
    pub fn set_block_window(&mut self, size: Size) {
        self.block_window = clamp_window(size);
        self.fit_block_window();
    }

    /// `FitBlockWindow`: correlation needs the search window to hold the whole block.
    fn fit_block_window(&mut self) {
        if self.algorithm != TrackingAlgorithm::Correlation {
            return;
        }
        self.search_window.width = self.search_window.width.max(self.block_window.width);
        self.search_window.height = self.search_window.height.max(self.block_window.height);
    }

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
            ^ self.hsv_range.content_hash()
    }
}

fn clamp_window(size: Size) -> Size {
    Size::new(
        size.width.min(MAX_WINDOW_SIZE),
        size.height.min(MAX_WINDOW_SIZE),
    )
}
