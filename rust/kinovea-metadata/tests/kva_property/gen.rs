//! Seeded generator and XML builders shared by the KVA idempotence properties.

use kinovea_core::preferences::dom::Element;
use kinovea_metadata::ids::id_text;
use uuid::Uuid;

pub const SIZES: [u32; 13] = [1, 2, 3, 4, 6, 8, 10, 12, 14, 18, 24, 30, 36];
pub const FONT_SIZES: [u32; 13] = [8, 9, 10, 11, 12, 14, 16, 18, 20, 24, 28, 32, 36];
pub const MEASURES: [&str; 6] = [
    "None",
    "Distance",
    "Speed",
    "Position",
    "Frame",
    "AngleToHorizontal",
];
pub const TEXTS: [&str; 4] = ["Hello", "Sprint", "Línea & <b>", "a;b"];

/// Linear congruential generator with the Numerical Recipes constants and a fixed seed.
pub struct Rng(u32);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng((seed as u32).wrapping_mul(2_654_435_761).wrapping_add(1))
    }

    pub fn bits(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.0 >> 8
    }

    /// Uniform integer in `lo..=hi`.
    pub fn int(&mut self, lo: i64, hi: i64) -> i64 {
        lo + i64::from(self.bits()) % (hi - lo + 1)
    }

    pub fn chance(&mut self) -> bool {
        self.bits().is_multiple_of(2)
    }

    pub fn flag(&mut self) -> &'static str {
        if self.chance() {
            "true"
        } else {
            "false"
        }
    }

    pub fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        items[self.bits() as usize % items.len()]
    }

    /// Coordinate in half-pixel steps, so the text stays short and exact.
    pub fn coord(&mut self) -> String {
        (self.int(0, 800) as f32 / 2.0).to_string()
    }

    pub fn point(&mut self) -> String {
        format!("{};{}", self.coord(), self.coord())
    }

    pub fn color(&mut self) -> String {
        format!(
            "255;{};{};{}",
            self.int(0, 255),
            self.int(0, 255),
            self.int(0, 255)
        )
    }

    pub fn uuid(&mut self) -> String {
        id_text(Uuid::from_u128(u128::from(self.bits()) + 1))
    }
}

pub fn leaf(name: &str, text: impl ToString) -> Element {
    Element::leaf(name, &text.to_string())
}

pub fn node(name: &str, children: Vec<Element>) -> Element {
    Element {
        children,
        ..Element::new(name)
    }
}

/// `<Name id="…" name="…">children</Name>`, the shape of every attached drawing.
pub fn drawing(name: &str, r: &mut Rng, children: Vec<Element>) -> Element {
    let mut e = node(name, children);
    e.attributes = vec![
        ("id".into(), r.uuid()),
        ("name".into(), "Drawing <1> & co".into()),
    ];
    e
}

/// `<Kind Key="key"><Value>value</Value></Kind>`, a `DrawingStyle` entry.
pub fn setting(kind: &str, key: &str, value: impl ToString) -> Element {
    let mut e = node(kind, vec![leaf("Value", value)]);
    e.attributes.push(("Key".into(), key.into()));
    e
}
