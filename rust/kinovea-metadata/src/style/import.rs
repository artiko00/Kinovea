//! `ImportValues` of the original style elements: the `Value` setter of each class accepts
//! its own kind only and falls back to the class default otherwise.

use crate::color::Color;
use crate::style::{LineEnding, LineShape, PenShape, StyleValue, TrackShape};

/// Copies `src` into `dst`. The element keeps its own properties (`Min`, `Max`,
/// `DisplayName`, `Variant`); a value of another kind resets it to the class default.
pub fn import_value(dst: &mut StyleValue, src: &StyleValue) {
    if let Some((value, default)) = int_slot(dst) {
        *value = int_of(src).unwrap_or(default);
        return;
    }
    match (dst, src) {
        (StyleValue::Toggle { value, .. }, StyleValue::Toggle { value: v, .. }) => *value = *v,
        (StyleValue::Toggle { value, .. }, _) => *value = false,
        (dst, src) if std::mem::discriminant(dst) == std::mem::discriminant(src) => *dst = src.clone(),
        (dst, _) => {
            let default = class_default(dst);
            *dst = default;
        }
    }
}

/// Value slot and class default of the integer classes, which take any integer.
fn int_slot(v: &mut StyleValue) -> Option<(&mut i32, i32)> {
    match v {
        StyleValue::Int { value, .. } => Some((value, 10)),
        StyleValue::FontSize(value) => Some((value, 14)),
        StyleValue::PenSize(value) | StyleValue::LineSize(value) => Some((value, 3)),
        _ => None,
    }
}

fn int_of(v: &StyleValue) -> Option<i32> {
    match v {
        StyleValue::Int { value, .. } => Some(*value),
        StyleValue::FontSize(value) | StyleValue::PenSize(value) | StyleValue::LineSize(value) => Some(*value),
        _ => None,
    }
}

/// Default of the element class of `v`. `Toggle` and the integer classes are handled by the caller.
fn class_default(v: &StyleValue) -> StyleValue {
    match v {
        StyleValue::Color(_) => StyleValue::Color(Color::BLACK),
        StyleValue::LineShape(_) => StyleValue::LineShape(LineShape::Solid),
        StyleValue::Arrows(_) => StyleValue::Arrows(LineEnding::None),
        StyleValue::TrackShape(_) => StyleValue::TrackShape(TrackShape::SOLID),
        StyleValue::PenShape(_) => StyleValue::PenShape(PenShape::Solid),
        _ => v.clone(),
    }
}
