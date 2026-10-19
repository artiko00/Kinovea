//! Typed style values with their KVA text forms (`StyleElement*` classes).

use crate::color::Color;
use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, write_boolean};

crate::text_enum!(LineShape [Solid, Dash, Squiggle]);
crate::text_enum!(LineEnding [None, StartArrow, EndArrow, DoubleArrow]);
crate::text_enum!(PenShape [Solid, Dash]);
crate::text_enum!(DashStyle [Solid, Dash, Dot, DashDot, DashDotDot, Custom]);
crate::text_enum!(ToggleVariant [Unknown, Curved, Perspective, Clock]);

/// Dash style plus time ticks (`TrackShape`), written as `Dash;true`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrackShape {
    pub dash: DashStyle,
    pub steps: bool,
}

impl TrackShape {
    pub const SOLID: TrackShape = TrackShape { dash: DashStyle::Solid, steps: false };

    pub fn parse(text: &str) -> TrackShape {
        let mut parts = text.split(';');
        match (parts.next().and_then(|d| d.trim().parse().ok()), parts.next(), parts.next()) {
            (Some(dash), Some(steps), None) => TrackShape { dash, steps: steps.trim() == "true" },
            _ => TrackShape::SOLID,
        }
    }

    pub fn write(&self) -> String {
        format!("{};{}", self.dash.name(), if self.steps { "true" } else { "false" })
    }
}

pub const LINE_SIZES: [i32; 13] = [1, 2, 3, 4, 6, 8, 10, 12, 14, 18, 24, 30, 36];
pub const FONT_SIZES: [i32; 16] = [6, 7, 8, 9, 10, 11, 12, 14, 18, 24, 30, 36, 48, 60, 72, 96];

/// `List<int>.PickAmong`: the value itself when listed, else the first option above it, else
/// the largest option.
pub fn pick_among(options: &[i32], candidate: i32) -> i32 {
    if options.contains(&candidate) {
        return candidate;
    }
    options.iter().copied().find(|v| candidate <= *v).unwrap_or(options[options.len() - 1])
}

#[derive(Debug, Clone, PartialEq)]
pub enum StyleValue {
    Color(Color),
    FontSize(i32),
    PenSize(i32),
    LineSize(i32),
    LineShape(LineShape),
    Arrows(LineEnding),
    TrackShape(TrackShape),
    PenShape(PenShape),
    Int { min: i32, max: i32, value: i32, display_name: Option<String> },
    Toggle { value: bool, variant: ToggleVariant },
    /// An element kind this model does not know, kept verbatim.
    Unknown(Element),
}

impl StyleValue {
    pub fn int(value: i32) -> Self {
        StyleValue::Int { min: 0, max: 100, value, display_name: None }
    }

    /// KVA element name of the kind.
    pub fn kva_name(&self) -> &str {
        match self {
            StyleValue::Color(_) => "Color",
            StyleValue::FontSize(_) => "FontSize",
            StyleValue::PenSize(_) => "PenSize",
            StyleValue::LineSize(_) => "LineSize",
            StyleValue::LineShape(_) => "LineShape",
            StyleValue::Arrows(_) => "Arrows",
            StyleValue::TrackShape(_) => "TrackShape",
            StyleValue::PenShape(_) => "PenShape",
            StyleValue::Int { .. } => "Int",
            StyleValue::Toggle { .. } => "Toggle",
            StyleValue::Unknown(e) => &e.name,
        }
    }

    /// Reads one element of a style block from its kind, `Value` and extra children.
    pub fn read_kva(e: &Element) -> StyleValue {
        let value = e.child_text("Value").unwrap_or("").trim();
        let int = |default: i32| value.parse::<i32>().unwrap_or(default);
        match e.name.as_str() {
            "Color" => StyleValue::Color(Color::parse(value, Color::BLACK)),
            "FontSize" => StyleValue::FontSize(pick_among(&FONT_SIZES, int(14))),
            "PenSize" => StyleValue::PenSize(pick_among(&LINE_SIZES, int(3))),
            "LineSize" => StyleValue::LineSize(pick_among(&LINE_SIZES, int(3))),
            "LineShape" => StyleValue::LineShape(value.parse().unwrap_or(LineShape::Solid)),
            "Arrows" => StyleValue::Arrows(value.parse().unwrap_or(LineEnding::None)),
            "TrackShape" => StyleValue::TrackShape(TrackShape::parse(value)),
            "PenShape" => StyleValue::PenShape(value.parse().unwrap_or(PenShape::Solid)),
            "Int" => StyleValue::Int {
                min: e.child_text("Min").and_then(|t| t.trim().parse().ok()).unwrap_or(0),
                max: e.child_text("Max").and_then(|t| t.trim().parse().ok()).unwrap_or(100),
                value: int(10),
                display_name: e.attribute("DisplayName").map(str::to_owned),
            },
            "Toggle" => {
                let variant = e.child_text("Variant").and_then(|t| t.trim().parse().ok()).unwrap_or(ToggleVariant::Unknown);
                let variant = if variant == ToggleVariant::Unknown { ToggleVariant::Curved } else { variant };
                StyleValue::Toggle { value: parse_boolean(value), variant }
            }
            _ => StyleValue::Unknown(e.clone()),
        }
    }

    /// `<Kind Key="key"><Value>…</Value></Kind>` (plus `Variant` for toggles).
    pub fn write_kva(&self, key: &str) -> Element {
        if let StyleValue::Unknown(e) = self {
            return e.clone();
        }
        let mut out = Element::new(self.kva_name());
        out.attributes.push(("Key".into(), key.to_owned()));
        let text = match self {
            StyleValue::Color(c) => c.write(true),
            StyleValue::FontSize(v) | StyleValue::PenSize(v) | StyleValue::LineSize(v) => v.to_string(),
            StyleValue::LineShape(v) => v.name().to_owned(),
            StyleValue::Arrows(v) => v.name().to_owned(),
            StyleValue::TrackShape(v) => v.write(),
            StyleValue::PenShape(v) => v.name().to_owned(),
            StyleValue::Int { value, .. } => value.to_string(),
            StyleValue::Toggle { value, .. } => write_boolean(*value),
            StyleValue::Unknown(_) => unreachable!(),
        };
        out.push_leaf("Value", &text);
        if let StyleValue::Toggle { variant, .. } = self {
            out.push_leaf("Variant", variant.name());
        }
        out
    }

    pub fn content_hash(&self) -> i32 {
        match self {
            StyleValue::Color(c) => c.hash_code(),
            StyleValue::FontSize(v) | StyleValue::PenSize(v) | StyleValue::LineSize(v) => *v,
            StyleValue::LineShape(v) => *v as i32,
            StyleValue::Arrows(v) => *v as i32 + 7,
            StyleValue::TrackShape(v) => (v.dash as i32) * 2 + i32::from(v.steps),
            StyleValue::PenShape(v) => *v as i32 + 3,
            StyleValue::Int { value, .. } => *value,
            StyleValue::Toggle { value, variant } => i32::from(*value) ^ (*variant as i32) << 1,
            StyleValue::Unknown(e) => e.text.len() as i32,
        }
    }
}
