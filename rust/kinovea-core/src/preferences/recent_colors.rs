//! Recently used drawing colours of the `Player` group.

use super::dom::Element;
use super::helpers::{Color, XmlField};

/// Recently used drawing colours, capped by `max` at write time and omitted when empty.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RecentColors(pub Vec<Color>);

impl RecentColors {
    pub(crate) fn write(&self, max: i32) -> Option<Element> {
        if self.0.is_empty() {
            return None;
        }
        let mut e = Element::new("RecentColors");
        e.children.extend(
            self.0
                .iter()
                .take(max.max(0) as usize)
                .filter_map(|c| c.write("RecentColor")),
        );
        Some(e)
    }
    pub(crate) fn read(&mut self, element: &Element) {
        self.0 = element
            .children
            .iter()
            .filter(|c| c.name == "RecentColor")
            .map(|c| {
                let mut color = Color::default();
                color.read(c);
                color
            })
            .collect();
    }
}
