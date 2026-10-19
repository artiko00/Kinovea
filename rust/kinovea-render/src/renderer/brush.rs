//! `SolidBrush`: the only brush the original drawings use (`HatchBrush` and gradients are not used).

use kinovea_metadata::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brush {
    pub color: Color,
}

impl Brush {
    pub const fn solid(color: Color) -> Self {
        Brush { color }
    }
}
