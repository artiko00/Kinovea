//! Value encodings shared by the preference groups (`XmlHelper.cs` equivalents) and the
//! `XmlField` trait that maps a field to one XML element.

use super::dom::Element;
use super::enums::NamedEnum;
use crate::units_precision::format_f32_default;

/// `XmlHelper.ParseBoolean`: anything but `false`, `False` and `0` is true.
pub fn parse_bool(s: &str) -> bool {
    s != "false" && s != "False" && s != "0"
}

/// An RGB colour written as `R;G;B`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// An integer size written as `Width;Height`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

/// A float point written as `X;Y`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

/// A float written with three decimals (`ToString("0.000")`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Fixed3(pub f64);

fn split_numbers<T: std::str::FromStr>(s: &str, n: usize) -> Option<Vec<T>> {
    let parts: Vec<T> = s.split(';').filter_map(|p| p.trim().parse().ok()).collect();
    (parts.len() == n).then_some(parts)
}

/// A value that reads from and writes to one XML element. `write` returns `None` when the
/// original writer omits the element (empty lists).
pub trait XmlField {
    fn write(&self, name: &str) -> Option<Element>;
    fn read(&mut self, element: &Element);
}

macro_rules! leaf_field {
    ($t:ty, |$v:ident| $to:expr, |$s:ident| $from:expr) => {
        impl XmlField for $t {
            fn write(&self, name: &str) -> Option<Element> {
                let $v = self;
                Some(Element::leaf(name, &$to))
            }
            fn read(&mut self, element: &Element) {
                let $s = element.text.as_str();
                if let Some(v) = $from {
                    *self = v;
                }
            }
        }
    };
}

leaf_field!(bool, |v| if *v { "true" } else { "false" }, |s| Some(
    parse_bool(s)
));
leaf_field!(i32, |v| v.to_string(), |s| s.trim().parse().ok());
leaf_field!(f32, |v| format_f32_default(*v), |s| s.trim().parse().ok());
leaf_field!(Fixed3, |v| format!("{:.3}", v.0), |s| s
    .trim()
    .parse()
    .ok()
    .map(Fixed3));
leaf_field!(String, |v| v.clone(), |s| Some(s.to_owned()));
leaf_field!(Color, |v| format!("{};{};{}", v.r, v.g, v.b), |s| {
    split_numbers::<u8>(s, 3)
        .map(|p| Color::rgb(p[0], p[1], p[2]))
        .or_else(|| split_numbers::<u8>(s, 4).map(|p| Color::rgb(p[1], p[2], p[3])))
});
leaf_field!(Size, |v| format!("{};{}", v.width, v.height), |s| {
    split_numbers::<i32>(s, 2).map(|p| Size {
        width: p[0],
        height: p[1],
    })
});
leaf_field!(
    PointF,
    |v| format!("{};{}", format_f32_default(v.x), format_f32_default(v.y)),
    |s| { split_numbers::<f32>(s, 2).map(|p| PointF { x: p[0], y: p[1] }) }
);
leaf_field!(uuid::Uuid, |v| v.to_string(), |s| s.trim().parse().ok());

impl<T: NamedEnum> XmlField for T {
    fn write(&self, name: &str) -> Option<Element> {
        Some(Element::leaf(name, self.name()))
    }
    /// Unknown names keep the current value, as the original falls back to its default.
    fn read(&mut self, element: &Element) {
        if let Ok(v) = element.text.parse() {
            *self = v;
        }
    }
}
