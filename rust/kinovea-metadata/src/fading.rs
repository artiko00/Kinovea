//! Opacity over time of a drawing (`InfosFading`), with its KVA block.

use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::{parse_boolean, parse_float, write_boolean, write_float};

#[derive(Debug, Clone, PartialEq)]
pub struct Fading {
    pub enabled: bool,
    pub use_default: bool,
    pub always_visible: bool,
    pub fading_frames: i32,
    pub opaque_frames: i32,
    pub reference_timestamp: i64,
    pub average_timestamps_per_frame: f64,
    pub master_factor: f32,
}

impl Default for Fading {
    fn default() -> Self {
        Fading {
            enabled: true,
            use_default: true,
            always_visible: false,
            fading_frames: 20,
            opaque_frames: 1,
            reference_timestamp: 0,
            average_timestamps_per_frame: 0.0,
            master_factor: 1.0,
        }
    }
}

impl Fading {
    /// `Enabled`, `UseDefault`, `AlwaysVisible`, `MasterFactor`, `OpaqueFrames`, `Frames`.
    pub fn write_kva(&self) -> Vec<Element> {
        vec![
            Element::leaf("Enabled", &write_boolean(self.enabled)),
            Element::leaf("UseDefault", &write_boolean(self.use_default)),
            Element::leaf("AlwaysVisible", &write_boolean(self.always_visible)),
            Element::leaf("MasterFactor", &write_float(self.master_factor)),
            Element::leaf("OpaqueFrames", &self.opaque_frames.to_string()),
            Element::leaf("Frames", &self.fading_frames.to_string()),
        ]
    }

    /// Reads an `InfosFading` element; unknown children are ignored and `Frames` is at least 1.
    pub fn read_kva(&mut self, e: &Element) {
        for c in &e.children {
            let t = c.text.trim();
            match c.name.as_str() {
                "Enabled" => self.enabled = parse_boolean(t),
                "UseDefault" => self.use_default = parse_boolean(t),
                "AlwaysVisible" => self.always_visible = parse_boolean(t),
                "MasterFactor" => self.master_factor = parse_float(t),
                "OpaqueFrames" => self.opaque_frames = t.parse().unwrap_or(self.opaque_frames),
                "Frames" => self.fading_frames = t.parse().unwrap_or(self.fading_frames),
                _ => {}
            }
        }
        self.fading_frames = self.fading_frames.max(1);
    }

    /// Opacity in `[0, master_factor]` at `timestamp`; `default` supplies the values when
    /// `use_default` is set (the player preference in the original).
    pub fn opacity_factor(&self, timestamp: i64, default: &Fading) -> f64 {
        let src = if self.use_default { default } else { self };
        self.compute(timestamp, src.always_visible, src.opaque_frames, src.fading_frames, src.master_factor)
    }

    /// Opacity for a tracked drawing: the larger of the baseline and the relative fade.
    pub fn opacity_trackable(&self, relative_timestamps: i64, current: i64, default: &Fading) -> f64 {
        let baseline = self.opacity_factor(current, default);
        if relative_timestamps < 0 {
            return baseline;
        }
        let fading = f64::from(self.fading_frames) * self.average_timestamps_per_frame;
        let relative = (1.0 - relative_timestamps as f64 / fading).max(0.0) * f64::from(self.master_factor);
        baseline.max(relative)
    }

    fn compute(&self, query: i64, always_visible: bool, opaque_frames: i32, fading_frames: i32, master: f32) -> f64 {
        let master = f64::from(master);
        if always_visible {
            return master;
        }
        let opaque_timestamps = f64::from(opaque_frames - 1) * self.average_timestamps_per_frame;
        let start = self.reference_timestamp;
        let end = (start as f64 + opaque_timestamps).round() as i64;
        if query >= start && query <= end {
            return master;
        }
        let distance = if query < start { start - query } else { query - end };
        let fading = f64::from(fading_frames) * self.average_timestamps_per_frame;
        (1.0 - distance as f64 / fading).max(0.0) * master
    }

    /// XOR of the `.NET` hashes of the persisted fields.
    pub fn content_hash(&self) -> i32 {
        let b = |v: bool| if v { 1 } else { 0 };
        let long = |v: i64| (v as i32) ^ ((v >> 32) as i32);
        b(self.enabled) ^ b(self.use_default) ^ b(self.always_visible) ^ self.fading_frames ^ self.opaque_frames ^ long(self.reference_timestamp) ^ (self.master_factor.to_bits() as i32)
    }
}
