//! Owner: P2. Port of `MiniLabel.Draw`: the label box with its connector to the attach point.

use crate::paint::helpers::rounded_rect::draw_text_box;
use crate::paint::PaintContext;
use crate::renderer::{Font, FontFamily, Renderer};
use crate::style::StyleData;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::measure_label::MeasureLabel;
use kinovea_metadata::Color;

/// Port of `MiniLabel.Draw(canvas, transformer, opacity)` for a label whose owner already set its
/// text, back colour and font size (`MiniLabel.SetText`, `BackColor`, `FontSize`). The owner
/// passes the label anchored to the current point (see [`anchor_to`]), since the original moves
/// the label with `SetAttach(p, true)` before drawing. Nothing is painted at zero opacity.
///
/// The original also stores the untransformed text size as the label size, for hit testing. That
/// write is not done here because the label is borrowed immutably.
pub fn draw_mini_label(
    r: &mut dyn Renderer,
    label: &MeasureLabel,
    text: &str,
    back_color: Color,
    font_size: i32,
    opacity: f64,
    ctx: &PaintContext,
) {
    if opacity <= 0.0 {
        return;
    }
    let style = StyleData {
        background_color: back_color,
        font_size,
        font_family: FontFamily::Arial,
        font_bold: true,
        ..StyleData::default()
    };
    let font: Font = style.font(ctx.viewport.scale as f32);
    let fill = style.background_brush((opacity * 255.0) as i32);
    let fore = style.foreground_brush((opacity * 255.0) as i32);
    let connector = style.background_pen((opacity * 64.0) as i32);

    let size = r.measure_text(text, &font);
    let untransformed = ctx.viewport.untransform_size_f(size);
    let centre = PointF::new(
        label.position.x + untransformed.width / 2.0,
        label.position.y + untransformed.height / 2.0,
    );
    let attach = ctx.viewport.transform_point_f(label.attach);
    let centre = ctx.viewport.transform_point_f(centre);

    let dot = RectangleF::new(attach.x as f32 - 2.0, attach.y as f32 - 2.0, 4.0, 4.0);
    r.fill_ellipse(dot, &fill);
    r.stroke_line(attach.to_point_f(), centre.to_point_f(), &connector);

    let origin = ctx.viewport.transform_point_f(label.position);
    draw_text_box(r, text, &font, &fill, &fore, origin, None);
}

/// `SetAttach(p, true)`: the attach point moves to `attach` and the label moves by the same offset,
/// so its position relative to the point is kept.
pub fn anchor_to(label: &MeasureLabel, attach: PointF) -> MeasureLabel {
    let dx = attach.x - label.attach.x;
    let dy = attach.y - label.attach.y;
    MeasureLabel {
        position: PointF::new(label.position.x + dx, label.position.y + dy),
        attach,
        ..*label
    }
}
