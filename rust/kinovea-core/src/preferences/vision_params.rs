//! Lens calibration, camera motion and keyframe preset defaults of the `Player` group.

use super::dom::Element;
use super::enums::CameraMotionFeatureType;
use super::fields::xml_struct;
use super::helpers::{Color, Size, XmlField};

#[derive(Debug, Clone, PartialEq)]
pub struct LensCalibrationParameters {
    pub max_images: i32,
    pub pattern_size: Size,
    pub max_iterations: i32,
}

impl Default for LensCalibrationParameters {
    fn default() -> Self {
        Self {
            max_images: 12,
            pattern_size: Size {
                width: 10,
                height: 7,
            },
            max_iterations: 30,
        }
    }
}

xml_struct!(LensCalibrationParameters {
    max_images => "MaxImages", pattern_size => "PatternSize", max_iterations => "MaxIterations",
});

#[derive(Debug, Clone, PartialEq)]
pub struct CameraMotionParameters {
    pub step_by_step: bool,
    pub feature_type: CameraMotionFeatureType,
    pub features_per_frame: i32,
    pub ransac_reproj_threshold: f32,
    pub use_distance_threshold: bool,
    pub distance_threshold_normalized: f32,
    pub use_distance_ratio_test: bool,
    pub min_track_length: i32,
}

impl Default for CameraMotionParameters {
    fn default() -> Self {
        Self {
            step_by_step: true,
            feature_type: CameraMotionFeatureType::SIFT,
            features_per_frame: 2048,
            ransac_reproj_threshold: 1.25,
            use_distance_threshold: true,
            distance_threshold_normalized: 0.1,
            use_distance_ratio_test: true,
            min_track_length: 6,
        }
    }
}

xml_struct!(CameraMotionParameters {
    step_by_step => "StepByStep", feature_type => "FeatureType", features_per_frame => "FeaturesPerFrame",
    ransac_reproj_threshold => "RansacReprojThreshold", use_distance_threshold => "UseDistanceThreshold",
    distance_threshold_normalized => "DistanceThresholdNormalized",
    use_distance_ratio_test => "UseDistanceRatioTest", min_track_length => "MinTrackLength",
});

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyframePreset {
    pub name: String,
    pub color: Color,
}

xml_struct!(KeyframePreset { name => "Name", color => "Color" });

/// The ten keyframe colour presets.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyframePresets(pub Vec<KeyframePreset>);

impl Default for KeyframePresets {
    fn default() -> Self {
        let colors = [
            (51, 152, 255),
            (51, 255, 255),
            (152, 255, 51),
            (255, 255, 51),
            (255, 152, 51),
            (255, 51, 51),
            (255, 51, 255),
            (152, 51, 255),
            (51, 51, 255),
            (152, 152, 152),
        ];
        Self(
            colors
                .iter()
                .map(|&(r, g, b)| KeyframePreset {
                    name: String::new(),
                    color: Color::rgb(r, g, b),
                })
                .collect(),
        )
    }
}

impl XmlField for KeyframePresets {
    fn write(&self, name: &str) -> Option<Element> {
        let mut e = Element::new(name);
        e.children
            .extend(self.0.iter().filter_map(|p| p.write("KeyframePreset")));
        Some(e)
    }
    /// Presets replace the defaults one by one, so a shorter list keeps the remaining defaults.
    fn read(&mut self, element: &Element) {
        for (i, c) in element
            .children
            .iter()
            .filter(|c| c.name == "KeyframePreset")
            .enumerate()
        {
            let mut preset = KeyframePreset {
                name: String::new(),
                color: Color::rgb(70, 130, 180),
            };
            preset.read(c);
            match self.0.get_mut(i) {
                Some(slot) => *slot = preset,
                None => self.0.push(preset),
            }
        }
    }
}
