//! `System.Drawing.Font` as the drawings use it: family, em size in pixels and weight.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    Arial,
    Consolas,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Font {
    pub family: FontFamily,
    /// Em size in viewport pixels (points converted at 96 DPI by the style layer).
    pub em_px: f32,
    pub bold: bool,
}
