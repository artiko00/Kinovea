//! Point-list drawings: free-hand `Pencil`, `Polyline` and `TimePolyline`.

use super::{Header, KvaDrawing, RefValue};
use crate::kva_io::*;
use crate::style::default_style;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;

/// Free-hand stroke; points are anonymous.
#[derive(Debug, Clone, PartialEq)]
pub struct Pencil {
    pub header: Header,
    pub points: Vec<PointF>,
}

impl Pencil {
    pub fn new(points: Vec<PointF>) -> Self {
        Pencil {
            header: Header::new("", default_style("Pencil")),
            points,
        }
    }
}

impl KvaDrawing for Pencil {
    const KVA_NAME: &'static str = "Pencil";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = Pencil::new(Vec::new());
        d.header.read_attributes(e);
        for c in &e.children {
            if c.name == "PointList" {
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
            let mut list = Element::new("PointList");
            list.attributes
                .push(("Count".into(), self.points.len().to_string()));
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

    fn content_hash(&self) -> i32 {
        self.points.iter().fold(0, |h, p| h ^ p.hash_code())
            ^ self.header.style.content_hash()
            ^ self.header.fading.content_hash()
    }
}

/// Keyed points (`key` attribute), trackable one by one; shared by `Polyline` and `TimePolyline`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyedPoints(pub Vec<(String, PointF)>);

impl KeyedPoints {
    fn read(list: &Element, scale: Scale) -> KeyedPoints {
        let mut points: Vec<(String, PointF)> = Vec::new();
        for p in list.children.iter().filter(|p| p.name == "Point") {
            let key = p.attribute("key").unwrap_or("0").to_owned();
            if points.iter().any(|(k, _)| *k == key) {
                continue;
            }
            points.push((key, read_point(p, scale)));
        }
        KeyedPoints(points)
    }

    fn write(&self) -> Element {
        let mut list = Element::new("PointList");
        list.attributes
            .push(("Count".into(), self.0.len().to_string()));
        for (key, p) in &self.0 {
            let mut e = point_leaf("Point", *p);
            e.attributes.push(("key".into(), key.clone()));
            list.children.push(e);
        }
        list
    }

    fn set(&mut self, key: &str, value: PointF) {
        if let Some(entry) = self.0.iter_mut().find(|(k, _)| k == key) {
            entry.1 = value;
        }
    }

    fn hash(&self) -> i32 {
        self.0.iter().fold(0, |h, (_, p)| h ^ p.hash_code())
    }
}

macro_rules! keyed_drawing {
    ($(#[$doc:meta])* $name:ident, $kva:literal, $tool:literal) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            pub header: Header,
            pub points: KeyedPoints,
        }

        impl $name {
            pub fn new(points: Vec<(String, PointF)>) -> Self {
                $name { header: Header::new("", default_style($tool)), points: KeyedPoints(points) }
            }
        }

        impl KvaDrawing for $name {
            const KVA_NAME: &'static str = $kva;

            fn header(&self) -> &Header {
                &self.header
            }

            fn header_mut(&mut self) -> &mut Header {
                &mut self.header
            }

            fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
                let mut d = $name::new(Vec::new());
                d.header.read_attributes(e);
                for c in &e.children {
                    if c.name == "PointList" {
                        d.points = KeyedPoints::read(c, scale);
                    } else {
                        read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
                    }
                }
                d
            }

            fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
                let mut out = Vec::new();
                if filter.core {
                    out.push(self.points.write());
                }
                push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
                out
            }

            fn trackable_points(&self) -> Vec<(String, PointF)> {
                self.points.0.clone()
            }

            fn set_trackable_point(&mut self, key: &str, value: PointF) {
                self.points.set(key, value);
            }

            fn content_hash(&self) -> i32 {
                self.points.hash() ^ self.header.style.content_hash() ^ self.header.fading.content_hash()
            }
        }
    };
}

keyed_drawing!(
    /// Segments between keyed points (`DrawingPolyline`).
    Polyline, "Polyline", "Polyline"
);
keyed_drawing!(
    /// Polyline whose vertices carry time positions (`DrawingTimePolyline`).
    TimePolyline, "TimePolyline", "TimePolyline"
);
