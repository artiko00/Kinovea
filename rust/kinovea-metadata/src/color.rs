//! ARGB colour with the KVA text forms `A;R;G;B` and `R;G;B` (`XmlHelper.ParseColor/WriteColor`).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color::rgb(0, 0, 0);

    pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Color { a, r, g, b }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { a: 255, r, g, b }
    }

    /// `R;G;B` or `A;R;G;B`; `fallback` when unreadable (named colours are not supported).
    pub fn parse(text: &str, fallback: Color) -> Color {
        if let Some(hex) = text.trim().strip_prefix('#') {
            return Color::parse_hex(hex).unwrap_or(fallback);
        }
        let parts: Vec<Option<u8>> = text.split(';').map(|p| p.trim().parse().ok()).collect();
        match parts.as_slice() {
            [Some(r), Some(g), Some(b)] => Color::rgb(*r, *g, *b),
            [Some(a), Some(r), Some(g), Some(b)] => Color::argb(*a, *r, *g, *b),
            _ => fallback,
        }
    }

    /// `#RRGGBB` or `#AARRGGBB`, as `ColorConverter` accepts.
    fn parse_hex(hex: &str) -> Option<Color> {
        let v = u32::from_str_radix(hex, 16).ok()?;
        match hex.len() {
            6 => Some(Color::rgb((v >> 16) as u8, (v >> 8) as u8, v as u8)),
            8 => Some(Color::argb((v >> 24) as u8, (v >> 16) as u8, (v >> 8) as u8, v as u8)),
            _ => None,
        }
    }

    /// `A;R;G;B` when `alpha`, `R;G;B` otherwise.
    pub fn write(&self, alpha: bool) -> String {
        if alpha {
            format!("{};{};{};{}", self.a, self.r, self.g, self.b)
        } else {
            format!("{};{};{}", self.r, self.g, self.b)
        }
    }

    /// `System.Drawing.Color.GetHashCode()` of a colour built from ARGB: the ARGB value hash.
    pub fn hash_code(&self) -> i32 {
        let argb = (u32::from(self.a) << 24) | (u32::from(self.r) << 16) | (u32::from(self.g) << 8) | u32::from(self.b);
        argb as i32
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}
