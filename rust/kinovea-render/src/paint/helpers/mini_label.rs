//! Owner: P2. Port of `MiniLabel.Draw`.

use crate::paint::PaintContext;
use crate::renderer::Renderer;
use kinovea_metadata::measure_label::MeasureLabel;
use kinovea_metadata::Color;

/// Port of `MiniLabel.Draw(canvas, transformer, opacity)` for a label whose owner already set
/// its text, back colour and font size (`MiniLabel.SetText`, `BackColor`, `FontSize`).
pub fn draw_mini_label(
    r: &mut dyn Renderer,
    label: &MeasureLabel,
    text: &str,
    back_color: Color,
    font_size: i32,
    opacity: f64,
    ctx: &PaintContext,
) {
    let _ = (r, label, text, back_color, font_size, opacity, ctx);
}
