//! `ImportValues` of the original style elements: the `Value` setter of each class accepts
//! its own kind only and falls back to the class default otherwise.

use crate::color::Color;
use crate::style::{LineEnding, LineShape, PenShape, StyleValue, TrackShape};

/// Copies `src` into `dst`. The element keeps its own properties (`Min`, `Max`,
/// `DisplayName`, `Variant`); a value of another kind resets it to the class default.
pub fn import_value(dst: &mut StyleValue, src: &StyleValue) {
    match (dst, src) {
        (StyleValue::Int { value, .. }, StyleValue::Int { value: v, .. }) => *value = *v,
        (StyleValue::Int { value, .. }, _) => *value = 10,
        (StyleValue::Toggle { value, .. }, StyleValue::Toggle { value: v, .. }) => *value = *v,
        (StyleValue::Toggle { value, .. }, _) => *value = false,
        (dst, src) if std::mem::discriminant(dst) == std::mem::discriminant(src) => *dst = src.clone(),
        (dst, _) => {
            let default = class_default(dst);
            *dst = default;
        }
    }
}

/// Default of the element class of `v`. `Int`, `Toggle` and unknown kinds are handled by the caller.
fn class_default(v: &StyleValue) -> StyleValue {
    match v {
        StyleValue::Color(_) => StyleValue::Color(Color::BLACK),
        StyleValue::FontSize(_) => StyleValue::FontSize(14),
        StyleValue::PenSize(_) => StyleValue::PenSize(3),
        StyleValue::LineSize(_) => StyleValue::LineSize(3),
        StyleValue::LineShape(_) => StyleValue::LineShape(LineShape::Solid),
        StyleValue::Arrows(_) => StyleValue::Arrows(LineEnding::None),
        StyleValue::TrackShape(_) => StyleValue::TrackShape(TrackShape::SOLID),
        StyleValue::PenShape(_) => StyleValue::PenShape(PenShape::Solid),
        _ => v.clone(),
    }
}
