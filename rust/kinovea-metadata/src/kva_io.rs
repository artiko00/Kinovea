//! Shared KVA value helpers: points with import scaling, timestamps, enums and the common
//! `DrawingStyle` / `InfosFading` children.

use crate::fading::Fading;
use crate::style::StyleElements;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_point_f, write_point_f};
use kinovea_geometry::PointF;

/// Which parts of a drawing to serialize (`SerializationFilter`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Filter {
    pub core: bool,
    pub style: bool,
    pub fading: bool,
}

impl Filter {
    pub const KVA: Filter = Filter { core: true, style: true, fading: true };
    pub const CORE: Filter = Filter { core: true, style: false, fading: false };
}

/// Import scaling applied to every coordinate (`PointF scale` of the original readers).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

impl Scale {
    pub const IDENTITY: Scale = Scale { x: 1.0, y: 1.0 };

    pub fn apply(&self, p: PointF) -> PointF {
        p.scale(self.x, self.y)
    }

    pub fn min(&self) -> f32 {
        self.x.min(self.y)
    }
}

/// Maps file timestamps to the current video (`TimestampMapper`).
pub type TimestampMapper<'a> = &'a dyn Fn(i64) -> i64;

pub fn identity(t: i64) -> i64 {
    t
}

pub fn read_point(e: &Element, scale: Scale) -> PointF {
    let (x, y) = parse_point_f(&e.text);
    scale.apply(PointF::new(x, y))
}

pub fn point_leaf(name: &str, p: PointF) -> Element {
    Element::leaf(name, &write_point_f(p.x, p.y))
}

/// `XmlHelper.ParseTimestamp`: a long, 0 when unreadable.
pub fn parse_timestamp(text: &str) -> i64 {
    text.trim().parse().unwrap_or(0)
}

/// `-1` stands for `long.MaxValue` (an unset end time).
pub fn read_open_timestamp(text: &str, map: TimestampMapper) -> i64 {
    let v = parse_timestamp(text);
    if v == -1 { i64::MAX } else { map(v) }
}

pub fn write_open_timestamp(v: i64) -> String {
    if v == i64::MAX { "-1".to_owned() } else { v.to_string() }
}

pub fn style_block(style: &StyleElements) -> Element {
    let mut e = Element::new("DrawingStyle");
    e.children = style.write_kva();
    e
}

pub fn fading_block(fading: &Fading) -> Element {
    let mut e = Element::new("InfosFading");
    e.children = fading.write_kva();
    e
}

/// Reads the two children every attached drawing shares; returns `false` for other names.
pub fn read_style_or_fading(c: &Element, style: &mut StyleElements, fading: &mut Fading) -> bool {
    match c.name.as_str() {
        "DrawingStyle" => style.import_kva(c),
        "InfosFading" => fading.read_kva(c),
        _ => return false,
    }
    true
}

/// Appends `DrawingStyle` and `InfosFading` per the filter.
pub fn push_style_and_fading(out: &mut Vec<Element>, filter: Filter, style: &StyleElements, fading: &Fading) {
    if filter.style {
        out.push(style_block(style));
    }
    if filter.fading {
        out.push(fading_block(fading));
    }
}

/// Parses an enum name with a fallback (`XmlHelper.ParseEnum`).
pub fn parse_enum<T: std::str::FromStr>(text: &str, fallback: T) -> T {
    text.trim().parse().unwrap_or(fallback)
}
