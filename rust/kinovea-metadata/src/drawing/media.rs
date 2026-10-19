//! Image overlays: embedded or sticker bitmap (`DrawingBitmap`) and SVG file (`DrawingSVG`).

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::style::StyleElements;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean, write_rectangle_f};
use kinovea_geometry::RectangleF;

#[derive(Debug, Clone, PartialEq)]
pub struct Bitmap {
    pub header: Header,
    pub file: String,
    /// Integer rectangle, as the original truncates the parsed `RectangleF`.
    pub bounding_box: RectangleF,
    pub is_sticker: bool,
    pub sticker_reference: String,
    /// PNG payload as base64, kept verbatim.
    pub bitmap_base64: String,
}

impl Bitmap {
    pub fn new(bounding_box: RectangleF) -> Self {
        Bitmap {
            header: Header::new("", StyleElements::default()),
            file: String::new(),
            bounding_box,
            is_sticker: false,
            sticker_reference: String::new(),
            bitmap_base64: String::new(),
        }
    }
}

impl KvaDrawing for Bitmap {
    const KVA_NAME: &'static str = "Bitmap";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, _scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = Bitmap::new(RectangleF::EMPTY);
        d.header.read_attributes(e);
        for c in &e.children {
            match c.name.as_str() {
                "File" => d.file = c.text.clone(),
                "BoundingBox" => {
                    let v: Vec<f32> = c
                        .text
                        .split(';')
                        .filter_map(|p| p.trim().parse().ok())
                        .collect();
                    if v.len() == 4 {
                        d.bounding_box =
                            RectangleF::new(v[0].trunc(), v[1].trunc(), v[2].trunc(), v[3].trunc());
                    }
                }
                "Sticker" => d.is_sticker = parse_boolean(c.text.trim()),
                "StickerReference" => d.sticker_reference = c.text.clone(),
                "Bitmap" => d.bitmap_base64 = c.text.clone(),
                "InfosFading" => d.header.fading.read_kva(c),
                _ => {}
            }
        }
        if !d.bitmap_base64.is_empty() {
            d.file.clear();
        }
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            let b = self.bounding_box;
            out.push(Element::leaf("File", &self.file));
            out.push(Element::leaf(
                "BoundingBox",
                &write_rectangle_f(b.x, b.y, b.width, b.height),
            ));
            out.push(Element::leaf("Sticker", &write_boolean(self.is_sticker)));
            if self.is_sticker {
                out.push(Element::leaf("StickerReference", &self.sticker_reference));
            } else {
                out.push(Element::leaf("Bitmap", &self.bitmap_base64));
            }
        }
        if filter.fading {
            out.push(fading_block(&self.header.fading));
        }
        out
    }

    /// A sticker reference or an embedded payload; a file only if it exists.
    fn is_valid(&self) -> bool {
        (self.is_sticker && !self.sticker_reference.is_empty())
            || !self.bitmap_base64.is_empty()
            || (!self.file.is_empty() && std::path::Path::new(&self.file).exists())
    }

    fn content_hash(&self) -> i32 {
        let b = self.bounding_box;
        (b.x.to_bits() ^ b.y.to_bits() ^ b.width.to_bits() ^ b.height.to_bits()) as i32
            ^ self.header.fading.content_hash()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Svg {
    pub header: Header,
    pub file: String,
}

impl Svg {
    pub fn new(file: &str) -> Self {
        Svg {
            header: Header::new("", StyleElements::default()),
            file: file.to_owned(),
        }
    }
}

impl KvaDrawing for Svg {
    const KVA_NAME: &'static str = "SVG";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, _scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = Svg::new("");
        d.header.read_attributes(e);
        for c in &e.children {
            match c.name.as_str() {
                "File" => d.file = c.text.clone(),
                "InfosFading" => d.header.fading.read_kva(c),
                _ => {}
            }
        }
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            out.push(Element::leaf("File", &self.file));
        }
        if filter.fading {
            out.push(fading_block(&self.header.fading));
        }
        out
    }

    /// The original returns 0 for SVG drawings.
    fn content_hash(&self) -> i32 {
        0
    }

    /// The original only keeps SVG drawings whose file could be loaded.
    fn is_valid(&self) -> bool {
        !self.file.is_empty() && std::path::Path::new(&self.file).exists()
    }
}
