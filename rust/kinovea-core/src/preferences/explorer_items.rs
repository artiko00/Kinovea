//! Building blocks of the `FileExplorer` group: recent lists, favourites, property visibility.

use super::dom::Element;
use super::enums::{FileProperty, NamedEnum};
use super::fields::xml_fields;
use super::helpers::{parse_bool, XmlField};

/// A recent-paths list capped at write time, omitted when empty.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Recents {
    pub items: Vec<String>,
}

impl Recents {
    pub(crate) fn write(&self, name: &str, item: &str, max: i32) -> Option<Element> {
        if self.items.is_empty() {
            return None;
        }
        let mut e = Element::new(name);
        for path in self.items.iter().take(max.max(0) as usize) {
            if !path.is_empty() {
                e.push_leaf(item, path);
            }
        }
        Some(e)
    }

    pub(crate) fn read(&mut self, element: &Element, item: &str) {
        self.items = element
            .children
            .iter()
            .filter(|c| c.name == item)
            .map(|c| c.text.clone())
            .collect();
    }
}

/// A favourite folder shown in the explorer shortcuts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FavoriteFolder {
    pub friendly_name: String,
    pub location: String,
}

xml_fields!(FavoriteFolder { friendly_name => "FriendlyName", location => "Location" });

/// Which file properties the explorer shows, in the original dictionary order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePropertyVisibility {
    pub visible: Vec<(FileProperty, bool)>,
}

impl Default for FilePropertyVisibility {
    fn default() -> Self {
        use FileProperty::*;
        Self {
            visible: vec![
                (Size, true),
                (Framerate, true),
                (Duration, true),
                (CreationTime, false),
                (HasKva, true),
            ],
        }
    }
}

impl XmlField for FilePropertyVisibility {
    fn write(&self, name: &str) -> Option<Element> {
        let mut e = Element::new(name);
        for (prop, on) in &self.visible {
            e.push_leaf(prop.name(), if *on { "true" } else { "false" });
        }
        Some(e)
    }
    /// Only properties already known are updated, like `FilePropertyVisibility.FromXML`.
    fn read(&mut self, element: &Element) {
        for c in &element.children {
            if let Ok(prop) = c
                .text
                .parse::<FileProperty>()
                .or_else(|_| c.name.parse::<FileProperty>())
            {
                if let Some(entry) = self.visible.iter_mut().find(|(p, _)| *p == prop) {
                    entry.1 = parse_bool(&c.text);
                }
            }
        }
    }
}
