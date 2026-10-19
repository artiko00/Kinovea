//! Drawing styles: an ordered map of typed elements (`StyleElements`) persisted as the KVA
//! `DrawingStyle` block, and the tool descriptions that provide defaults.

mod element;
#[cfg(test)]
mod element_test;
mod import;
#[cfg(test)]
mod import_test;
mod tools;
#[cfg(test)]
mod tools_test;

pub use element::{DashStyle, LineEnding, LineShape, PenShape, StyleValue, ToggleVariant, TrackShape};
pub use tools::{default_style, standard_tools, Tool};

use kinovea_core::preferences::dom::Element;

/// One `Key` → value entry of a `DrawingStyle` block.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleElement {
    pub key: String,
    pub value: StyleValue,
}

/// Ordered style entries; order is the KVA element order.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyleElements {
    pub elements: Vec<StyleElement>,
    memo: Vec<StyleElement>,
}

impl StyleElements {
    pub fn new(elements: Vec<(&str, StyleValue)>) -> Self {
        StyleElements {
            elements: elements.into_iter().map(|(k, v)| StyleElement { key: k.to_owned(), value: v }).collect(),
            memo: Vec::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&StyleValue> {
        self.elements.iter().find(|e| e.key == key).map(|e| &e.value)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut StyleValue> {
        self.elements.iter_mut().find(|e| e.key == key).map(|e| &mut e.value)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn insert(&mut self, key: &str, value: StyleValue) {
        match self.get_mut(key) {
            Some(v) => *v = value,
            None => self.elements.push(StyleElement { key: key.to_owned(), value }),
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<StyleValue> {
        let i = self.elements.iter().position(|e| e.key == key)?;
        Some(self.elements.remove(i).value)
    }

    /// `ImportValues`: copies the values of matching keys, ignoring keys not present here.
    /// Unknown kinds are not imported, as the original `ReadXml` drops them.
    pub fn import_values(&mut self, other: &StyleElements) {
        for e in other.elements.iter().filter(|e| !matches!(e.value, StyleValue::Unknown(_))) {
            if let Some(v) = self.get_mut(&e.key) {
                import::import_value(v, &e.value);
            }
        }
    }

    /// `ImportXML`: parses a block and imports its values into the existing keys.
    pub fn import_kva(&mut self, block: &Element) {
        let parsed = StyleElements::read_kva(block);
        self.import_values(&parsed);
    }

    /// Parses a `DrawingStyle` or `DefaultStyle` block; unknown element kinds are kept verbatim.
    pub fn read_kva(block: &Element) -> StyleElements {
        let elements = block
            .children
            .iter()
            .map(|c| StyleElement {
                key: c.attribute("Key").unwrap_or("").to_owned(),
                value: StyleValue::read_kva(c),
            })
            .collect();
        StyleElements { elements, memo: Vec::new() }
    }

    /// Children of the `DrawingStyle` block.
    pub fn write_kva(&self) -> Vec<Element> {
        self.elements.iter().map(|e| e.value.write_kva(&e.key)).collect()
    }

    /// `SanityCheck`: adds the preset entries that are missing.
    pub fn complete_from(&mut self, preset: &StyleElements) {
        for e in &preset.elements {
            if !self.contains(&e.key) {
                self.elements.push(e.clone());
            }
        }
    }

    pub fn memorize(&mut self) {
        self.memo = self.elements.clone();
    }

    pub fn restore(&mut self) {
        self.elements = self.memo.clone();
    }

    /// Stable hash of the persisted values.
    pub fn content_hash(&self) -> i32 {
        self.elements.iter().fold(0, |h, e| h ^ e.value.content_hash())
    }
}
