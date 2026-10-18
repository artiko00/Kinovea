//! Perspective grid (`DrawingPlane`) and lens distortion grid (`DrawingDistortionGrid`).

use super::rectangle::CORNER_NAMES;
use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::style::{default_style, StyleElements, StyleValue};
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, parse_float, write_boolean, write_float};
use kinovea_geometry::{PointF, Quadrilateral};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    pub header: Header,
    pub quad: Quadrilateral,
    pub show_grid: bool,
    pub show_x_line: bool,
    pub show_y_line: bool,
    pub x_line_coord: f32,
    pub y_line_coord: f32,
}

impl Plane {
    pub fn new(quad: Quadrilateral) -> Self {
        Plane {
            header: Header::new("", default_style("Plane")),
            quad,
            show_grid: true,
            show_x_line: false,
            show_y_line: false,
            x_line_coord: 0.5,
            y_line_coord: 0.5,
        }
    }

    /// Legacy files carry a single `divisions` element: it becomes `cols` and `rows`.
    fn import_style(style: &mut StyleElements, block: &Element) {
        let mut parsed = StyleElements::read_kva(block);
        if let Some(StyleValue::Int { value, .. }) = parsed.remove("divisions") {
            parsed.insert("cols", StyleValue::int(value));
            parsed.insert("rows", StyleValue::int(value));
        }
        style.import_values(&parsed);
    }
}

impl KvaDrawing for Plane {
    const KVA_NAME: &'static str = "Plane";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = Plane::new(Quadrilateral::empty());
        d.header.read_attributes(e);
        for c in &e.children {
            let t = c.text.trim();
            if let Some(i) = CORNER_NAMES.iter().position(|n| *n == c.name) {
                d.quad[i] = read_point(c, scale);
                continue;
            }
            match c.name.as_str() {
                "ReferenceTimestamp" => d.header.reference_timestamp = parse_timestamp(t),
                "ShowGrid" => d.show_grid = parse_boolean(t),
                "ShowXLine" => d.show_x_line = parse_boolean(t),
                "ShowYLine" => d.show_y_line = parse_boolean(t),
                "XLineCoord" => d.x_line_coord = t.parse().unwrap_or(0.5),
                "YLineCoord" => d.y_line_coord = t.parse().unwrap_or(0.5),
                "DrawingStyle" => Plane::import_style(&mut d.header.style, c),
                "InfosFading" => d.header.fading.read_kva(c),
                _ => {}
            }
        }
        let _ = parse_float;
        d
    }

    fn write_kva(&self, filter: Filter, refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            for (i, name) in CORNER_NAMES.iter().enumerate() {
                out.push(point_leaf(name, refs(self.header.id, &i.to_string())));
            }
            out.push(Element::leaf(
                "ReferenceTimestamp",
                &self.header.reference_timestamp.to_string(),
            ));
            out.push(Element::leaf("ShowGrid", &write_boolean(self.show_grid)));
            out.push(Element::leaf("ShowXLine", &write_boolean(self.show_x_line)));
            out.push(Element::leaf("ShowYLine", &write_boolean(self.show_y_line)));
            out.push(Element::leaf("XLineCoord", &write_float(self.x_line_coord)));
            out.push(Element::leaf("YLineCoord", &write_float(self.y_line_coord)));
        }
        push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
        out
    }

    fn trackable_points(&self) -> Vec<(String, PointF)> {
        self.quad
            .vertices
            .iter()
            .enumerate()
            .map(|(i, p)| (i.to_string(), *p))
            .collect()
    }

    fn set_trackable_point(&mut self, key: &str, value: PointF) {
        if let Some(i) = key.parse::<usize>().ok().filter(|i| *i < 4) {
            self.quad[i] = value;
        }
    }

    fn content_hash(&self) -> i32 {
        let flags = i32::from(self.show_grid)
            | i32::from(self.show_x_line) << 1
            | i32::from(self.show_y_line) << 2;
        self.quad.hash_code()
            ^ flags
            ^ (self.x_line_coord.to_bits() as i32)
            ^ (self.y_line_coord.to_bits() as i32)
            ^ self.header.style.content_hash()
            ^ self.header.fading.content_hash()
    }
}

/// Grid of points drawn over a distorted image (`DistortionGrid`).
#[derive(Debug, Clone, PartialEq)]
pub struct DistortionGrid {
    pub header: Header,
    pub points: Vec<PointF>,
}

impl DistortionGrid {
    pub fn new(points: Vec<PointF>) -> Self {
        DistortionGrid {
            header: Header::new("", default_style("DistortionGrid")),
            points,
        }
    }
}

impl KvaDrawing for DistortionGrid {
    const KVA_NAME: &'static str = "DistortionGrid";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = DistortionGrid::new(Vec::new());
        d.header.read_attributes(e);
        for c in &e.children {
            if c.name == "Points" {
                d.points = c
                    .children
                    .iter()
                    .filter(|p| p.name == "Point")
                    .map(|p| read_point(p, scale))
                    .collect();
            } else {
                read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
            }
        }
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        if filter.core {
            let mut list = Element::new("Points");
            list.children = self
                .points
                .iter()
                .map(|p| point_leaf("Point", *p))
                .collect();
            out.push(list);
        }
        push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
        out
    }

    fn trackable_points(&self) -> Vec<(String, PointF)> {
        self.points
            .iter()
            .enumerate()
            .map(|(i, p)| (i.to_string(), *p))
            .collect()
    }

    fn set_trackable_point(&mut self, key: &str, value: PointF) {
        if let Some(p) = key
            .parse::<usize>()
            .ok()
            .and_then(|i| self.points.get_mut(i))
        {
            *p = value;
        }
    }

    fn content_hash(&self) -> i32 {
        self.points.iter().fold(0, |h, p| h ^ p.hash_code())
            ^ self.header.style.content_hash()
            ^ self.header.fading.content_hash()
    }
}
