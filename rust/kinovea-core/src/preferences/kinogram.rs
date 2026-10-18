//! Export and kinogram defaults of the `Player` group (ports of `ExportProfile`,
//! `KinogramParameters`).

use super::dom::Element;
use super::enums::*;
use super::fields::xml_struct;
use super::helpers::{parse_bool, Color, PointF, Size, XmlField};

#[derive(Debug, Clone, PartialEq)]
pub struct ExportProfile {
    pub name: String,
    pub container: VideoContainer,
    pub codec: VideoCodec,
    pub encoding_quality: EncodingQuality,
    pub encoding_speed: EncodingSpeed,
    pub gop_size: i32,
}

impl Default for ExportProfile {
    fn default() -> Self {
        Self {
            name: "Default".to_owned(),
            container: VideoContainer::MP4,
            codec: VideoCodec::H264,
            encoding_quality: EncodingQuality::High,
            encoding_speed: EncodingSpeed::Medium,
            gop_size: 12,
        }
    }
}

xml_struct!(ExportProfile {
    name => "Name", container => "Container", codec => "Codec",
    encoding_quality => "EncodingQuality", encoding_speed => "EncodingSpeed", gop_size => "GOPSize",
});

/// One tile crop position of a kinogram: `<CropPosition anchor="true" offset="n">x;y</CropPosition>`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CropPosition {
    pub anchor: bool,
    pub offset: i32,
    pub position: PointF,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CropPositions(pub Vec<CropPosition>);

impl XmlField for CropPositions {
    fn write(&self, name: &str) -> Option<Element> {
        let mut e = Element::new(name);
        for cp in &self.0 {
            let mut c = cp.position.write("CropPosition")?;
            c.attributes.push((
                "anchor".into(),
                if cp.anchor {
                    "true".into()
                } else {
                    "false".into()
                },
            ));
            c.attributes.push(("offset".into(), cp.offset.to_string()));
            e.push(c);
        }
        Some(e)
    }
    fn read(&mut self, element: &Element) {
        self.0 = element
            .children
            .iter()
            .filter(|c| c.name == "CropPosition")
            .map(|c| {
                let mut position = PointF::default();
                position.read(c);
                CropPosition {
                    anchor: c.attribute("anchor").is_some_and(parse_bool),
                    offset: c
                        .attribute("offset")
                        .and_then(|o| o.parse().ok())
                        .unwrap_or(0),
                    position,
                }
            })
            .collect();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KinogramParameters {
    pub tile_count: i32,
    pub rows: i32,
    pub crop_size: Size,
    pub crop_positions: CropPositions,
    pub auto_interpolate: bool,
    pub left_to_right: bool,
    pub border_color: Color,
    pub border_visible: bool,
    pub label_color: Color,
    pub label_size: i32,
    pub measure_label_type: MeasureLabelType,
}

impl Default for KinogramParameters {
    fn default() -> Self {
        Self {
            tile_count: 18,
            rows: 3,
            crop_size: Size {
                width: 400,
                height: 600,
            },
            crop_positions: CropPositions::default(),
            auto_interpolate: true,
            left_to_right: true,
            border_color: Color::rgb(44, 44, 44),
            border_visible: true,
            label_color: Color::rgb(0, 0, 0),
            label_size: 8,
            measure_label_type: MeasureLabelType::None,
        }
    }
}

xml_struct!(KinogramParameters {
    tile_count => "TileCount", rows => "Rows", crop_size => "CropSize", crop_positions => "CropPositions",
    auto_interpolate => "AutoInterpolate", left_to_right => "LeftToRight", border_color => "BorderColor",
    border_visible => "BorderVisible", label_color => "LabelColor", label_size => "LabelSize",
    measure_label_type => "MeasureLabelType",
});
