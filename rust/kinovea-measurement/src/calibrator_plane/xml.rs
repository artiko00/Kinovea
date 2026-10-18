//! KVA fragments of a plane or line calibration (`WritePlaneXml`, `WriteLineXml` and readers).

use super::{make_quad, CalibratorPlane};
use crate::calibration_axis::CalibrationAxis;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_float, parse_point_f, parse_size_f, write_float, write_point_f, write_size_f};
use kinovea_geometry::{PointF, Quadrilateral, SizeF};

fn point(name: &str, p: PointF) -> Element {
    Element::leaf(name, &write_point_f(p.x, p.y))
}

fn read_point(e: &Element) -> PointF {
    let (x, y) = parse_point_f(&e.text);
    PointF::new(x, y)
}

impl CalibratorPlane {
    /// `Size`, `Quadrilateral{A,B,C,D}`, `Origin`, `Offset`.
    pub fn write_plane_xml(&self) -> Vec<Element> {
        let q = self.quad_image;
        let mut quad = Element::new("Quadrilateral");
        for (name, v) in ["A", "B", "C", "D"].iter().zip(q.vertices) {
            quad.push(point(name, v));
        }
        vec![
            Element::leaf("Size", &write_size_f(self.size.width, self.size.height)),
            quad,
            point("Origin", self.origin),
            point("Offset", self.offset),
        ]
    }

    /// `Length`, `Segment{A,B}` (D and C of the quad), `Origin`, `Offset`, `Axis`.
    pub fn write_line_xml(&self) -> Vec<Element> {
        let mut segment = Element::new("Segment");
        segment.push(point("A", self.quad_image.d())).push(point("B", self.quad_image.c()));
        vec![
            Element::leaf("Length", &write_float(self.size.width)),
            segment,
            point("Origin", self.origin),
            point("Offset", self.offset),
            Element::leaf("Axis", self.axis.name()),
        ]
    }

    /// Reads a `CalibrationPlane` element; the quad is scaled, origin and offset are not.
    pub fn read_plane_xml(&mut self, e: &Element, scale: PointF) {
        for c in &e.children {
            match c.name.as_str() {
                "Size" => {
                    let (w, h) = parse_size_f(&c.text);
                    self.size = SizeF::new(w, h);
                }
                "Quadrilateral" => self.quad_image = read_quad(c, scale),
                "Origin" => self.origin = read_point(c),
                "Offset" => self.offset = read_point(c),
                _ => {}
            }
        }
        self.finish_read();
    }

    /// Reads a `CalibrationLine` element, including the legacy `Scale` form.
    pub fn read_line_xml(&mut self, e: &Element, scale: PointF) {
        let mut length = 0.0;
        let (mut start, mut end) = (PointF::EMPTY, PointF::EMPTY);
        self.axis = CalibrationAxis::LineHorizontal;
        for c in &e.children {
            match c.name.as_str() {
                "Length" => length = parse_float(&c.text),
                "Segment" => {
                    start = c.child("A").map_or(PointF::EMPTY, read_point);
                    end = c.child("B").map_or(PointF::EMPTY, read_point);
                }
                "Origin" => {
                    let o = read_point(c);
                    let o = if o.x.is_nan() || o.y.is_nan() { PointF::EMPTY } else { o };
                    self.origin = o.scale(scale.x, scale.y);
                }
                "Offset" => self.offset = read_point(c),
                "Axis" => self.axis = c.text.parse().unwrap_or(CalibrationAxis::LineHorizontal),
                "Scale" => {
                    let baked = parse_float(&c.text);
                    let pixels = 100.0;
                    start = self.origin;
                    end = self.origin.translate(pixels, 0.0);
                    length = pixels * baked;
                    self.origin = PointF::new(0.0, length);
                }
                _ => {}
            }
        }
        self.size = SizeF::new(length, length);
        self.quad_image = make_quad(start, end, self.axis);
        self.finish_read();
    }

    fn finish_read(&mut self) {
        self.quad_world = Quadrilateral::from_size(self.size.width, self.size.height);
        let _ = self.mapping.update(&self.quad_world, &self.quad_image);
        self.valid = self.quad_image.is_convex();
        self.initialized = true;
    }
}

fn read_quad(e: &Element, scale: PointF) -> Quadrilateral {
    let mut q = Quadrilateral::empty();
    for (i, name) in ["A", "B", "C", "D"].iter().enumerate() {
        if let Some(c) = e.child(name) {
            q[i] = read_point(c);
        }
    }
    q.scale(scale.x, scale.y);
    q
}
