//! Colour helpers of `StyleData`: alpha from opacity and the black or white foreground.

use kinovea_metadata::Color;

/// `(int)(opacity * 255)`, the truncating cast the original applies to fading opacity.
pub fn alpha_of(opacity: f32) -> i32 {
    (opacity * 255.0) as i32
}

/// `Color.FromArgb(alpha, color)` when `alpha` is 0..=255; otherwise the colour keeps its own alpha.
pub fn with_alpha(color: Color, alpha: i32) -> Color {
    if (0..=255).contains(&alpha) {
        Color {
            a: alpha as u8,
            ..color
        }
    } else {
        color
    }
}

/// `Color.GetBrightness`: `(max + min) / 2` over the channels, in 0..=1.
fn brightness(color: Color) -> f32 {
    let max = color.r.max(color.g).max(color.b) as f32 / 255.0;
    let min = color.r.min(color.g).min(color.b) as f32 / 255.0;
    (max + min) / 2.0
}

/// `StyleData.GetForegroundColor(background)`: black on bright backgrounds, white otherwise.
pub fn foreground_for(background: Color) -> Color {
    if brightness(background) >= 0.5 {
        Color::BLACK
    } else {
        Color::rgb(255, 255, 255)
    }
}
