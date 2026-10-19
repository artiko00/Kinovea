//! `HSVRange`: min-max bounds of the HSV filter, stored as `min;max` pairs in KVA.

use kinovea_core::units_precision::format_f32_default;

#[derive(Debug, Clone, PartialEq)]
pub struct HsvRange {
    pub hue: (f32, f32),
    pub saturation: (f32, f32),
    pub value: (f32, f32),
}

impl Default for HsvRange {
    fn default() -> Self {
        HsvRange {
            hue: (0.0, 360.0),
            saturation: (0.0, 255.0),
            value: (0.0, 255.0),
        }
    }
}

impl HsvRange {
    /// `HSVRange.ReadXml` (`HSVRange.cs:77-95`): the six parts are parsed in order over the
    /// defaults. The first part missing or unreadable ends the read, and the fields already set
    /// are kept (the original's `catch` block is empty).
    pub fn parse(text: &str) -> HsvRange {
        let mut range = HsvRange::default();
        let mut bounds = [
            &mut range.hue.0,
            &mut range.hue.1,
            &mut range.saturation.0,
            &mut range.saturation.1,
            &mut range.value.0,
            &mut range.value.1,
        ];
        for (bound, part) in bounds.iter_mut().zip(text.split(';')) {
            match part.trim().parse::<f32>() {
                Ok(v) => **bound = v,
                Err(_) => break,
            }
        }
        range
    }

    pub fn write(&self) -> String {
        self.bounds()
            .iter()
            .map(|v| format_f32_default(*v))
            .collect::<Vec<_>>()
            .join(";")
    }

    /// XOR of the bit patterns of the six bounds, as `HSVRange.ContentHash`.
    pub fn content_hash(&self) -> i32 {
        self.bounds()
            .iter()
            .fold(0, |hash, v| hash ^ v.to_bits() as i32)
    }

    fn bounds(&self) -> [f32; 6] {
        [
            self.hue.0,
            self.hue.1,
            self.saturation.0,
            self.saturation.1,
            self.value.0,
            self.value.1,
        ]
    }
}
