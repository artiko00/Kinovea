use crate::paint::helpers::rounded_rect::draw_text_box;
use crate::paint::PaintContext;
use crate::renderer::{Font, FontFamily, Renderer};
use crate::style::StyleData;
use crate::viewport::ImageToViewport;
use kinovea_geometry::RectangleF;
use kinovea_metadata::measure_label::MeasureLabel;
use kinovea_metadata::Color;

/// The font of a mini label: bold Arial at `font_size`, stretched to the viewport scale.
pub(crate) fn mini_font(font_size: i32, scale: f32) -> Font {
    let style = StyleData {
        font_size,
        font_family: FontFamily::Arial,
        font_bold: true,
        ..StyleData::default()
    };
    style.font(scale)
}

/// Box of a mini label in image coordinates: the text measured at the viewport scale, untransformed,
/// placed at the label position (`MiniLabel.Draw` leaves this size in `background.Rectangle`).
pub(crate) fn label_box(
    r: &dyn Renderer,
    label: &MeasureLabel,
    text: &str,
    font_size: i32,
    viewport: &ImageToViewport,
) -> RectangleF {
    let font = mini_font(font_size, viewport.scale as f32);
    let size = viewport.untransform_size_f(r.measure_text(text, &font));
    RectangleF::from_location_size(label.position, size)
}

/// Port of `MiniLabel.Draw(canvas, transformer, opacity)` for a label whose owner already set its
/// text, back colour and font size (`MiniLabel.SetText`, `BackColor`, `FontSize`). The owner
/// passes the label anchored to the current point (`frame::attach_label`), since the original moves
/// the label with `SetAttach(p, true)` before drawing. Nothing is painted at zero opacity.
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
        ..StyleData::default()
    };
    let font = mini_font(font_size, ctx.viewport.scale as f32);
    let fill = style.background_brush((opacity * 255.0) as i32);
    let fore = style.foreground_brush((opacity * 255.0) as i32);
    let connector = style.background_pen((opacity * 64.0) as i32);

    let boxed = label_box(r, label, text, font_size, ctx.viewport);
    let attach = ctx.viewport.transform_point_f(label.attach);
    let centre = ctx.viewport.transform_point_f(boxed.center());

    let dot = RectangleF::new(attach.x as f32 - 2.0, attach.y as f32 - 2.0, 4.0, 4.0);
    r.fill_ellipse(dot, &fill);
    r.stroke_line(attach.to_point_f(), centre.to_point_f(), &connector);

    let origin = ctx.viewport.transform_point_f(label.position);
    draw_text_box(r, text, &font, &fill, &fore, origin, None);
}
