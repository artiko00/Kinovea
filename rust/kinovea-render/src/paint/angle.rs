//! Owner: P3. Painters ported from `DrawingAngle.Draw`.

pub mod value;
#[cfg(test)]
mod value_test;

use super::helpers::arrow::{arrow_offset, draw_arrow};
use super::helpers::frame::{opacity, point_box, rect_f, to_viewport};
use super::PaintContext;
use crate::paint::helpers::angle_helper::{AngleFlags, AngleHelper};
use crate::renderer::{Pen, Renderer};
use crate::style::StyleData;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Angle;
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::StyleElements;
use value::{draw_value, value_label, Value};

/// `defaultBackgroundAlpha`: opacity of the disk and of the text box.
const DISK_ALPHA: f64 = 92.0;
/// The arrow is only drawn on circles wider than this, in viewport pixels.
const ARROW_MIN_WIDTH: i32 = 100;
const HANDLE_RADIUS: f32 = 3.0;
/// Length of the default legs, in untransformed pixels (`FixIfNull`).
const DEFAULT_LEG: i32 = 50;

/// Port of `DrawingAngle.Draw`: disk, arc, the two legs with their handles, the arrow and the value.
pub fn paint_angle(d: &Angle, ctx: &PaintContext, r: &mut dyn Renderer) {
    let factor = opacity(&d.header.fading, ctx);
    if factor <= 0.0 {
        return;
    }
    let style = angle_style(&d.header.style);
    let (o, a, b) = fix_legs(d.o, d.a, d.b, ctx);
    // The angle painter always passes a zero radius: the arrow carries the sweep's extent.
    let flags = AngleFlags {
        radius: 0,
        signed: d.signed,
        ccw: d.counter_clockwise,
        supplementary: d.supplementary,
    };
    let helper = AngleHelper::new(o, a, b, flags, &ctx.metadata.calibration);
    let sweep = helper.sweep;
    let bounds = ctx.viewport.transform_rect_f(sweep.bounding_box);
    if bounds.width == 0 && bounds.height == 0 {
        return;
    }

    let alpha = (factor * 255.0) as i32;
    let mut edges = style.background_pen(alpha);
    edges.width = 2.0;
    let edges_brush = style.background_brush(alpha);
    let disk = style.background_brush((factor * DISK_ALPHA) as i32);
    let circle = rect_f(bounds);
    let origin = ctx.viewport.transform_point_f(o);
    let reference = ctx.viewport.transform_point_f(a);
    let measure = ctx.viewport.transform_point_f(b);

    r.fill_pie(circle, sweep.start, sweep.sweep, &disk);
    if d.show_circle {
        r.stroke_ellipse(circle, &edges);
    } else {
        r.stroke_arc(circle, sweep.start, sweep.sweep, &edges);
    }
    let dashed = Pen {
        dash: DashStyle::Dash,
        ..edges.clone()
    };
    r.stroke_line(origin.to_point_f(), reference.to_point_f(), &dashed);
    r.stroke_line(origin.to_point_f(), measure.to_point_f(), &edges);
    r.stroke_ellipse(rect_f(point_box(origin, HANDLE_RADIUS)), &edges);
    r.fill_ellipse(rect_f(point_box(reference, HANDLE_RADIUS)), &edges_brush);
    r.fill_ellipse(rect_f(point_box(measure, HANDLE_RADIUS)), &edges_brush);

    if bounds.width > ARROW_MIN_WIDTH {
        let arrow_start = to_viewport(ctx, sweep.arrow_start);
        let arrow_end = to_viewport(ctx, sweep.arrow_end);
        let offset = arrow_offset(edges.width, arrow_end, arrow_start);
        let tip = PointF::new(arrow_end.x + offset.x, arrow_end.y + offset.y);
        draw_arrow(r, &edges, tip, arrow_start);
    }

    let value = Value {
        text: value_label(d, &helper, &ctx.metadata.calibration),
        helper,
        style: &style,
        disk: &disk,
        factor,
    };
    draw_value(r, &value, origin.to_point_f(), ctx);
}

/// `SetupStyle` and `BindStyle`: the background follows the line colour ("Bicolor") and the font
/// is Arial bold.
fn angle_style(elements: &StyleElements) -> StyleData {
    let style = StyleData::from_elements(elements);
    StyleData {
        background_color: style.color,
        font_bold: true,
        ..style
    }
}

/// `FixIfNull`: a leg that nearly coincides with the origin goes back to its default place, the
/// reference leg upwards and the measure leg to the right. Only the painted copy changes.
fn fix_legs(o: PointF, a: PointF, b: PointF, ctx: &PaintContext) -> (PointF, PointF, PointF) {
    let length = ctx.viewport.untransform_distance(DEFAULT_LEG) as f32;
    let a = if nearly_coincide(a, o) {
        o.translate(0.0, -length)
    } else {
        a
    };
    let b = if nearly_coincide(b, o) {
        o.translate(length, 0.0)
    } else {
        b
    };
    (o, a, b)
}

/// `PointF.NearlyCoincideWith`: both coordinates differ by less than half a pixel.
fn nearly_coincide(p: PointF, q: PointF) -> bool {
    (p.x - q.x).abs() < 0.5 && (p.y - q.y).abs() < 0.5
}
