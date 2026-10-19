//! Port of `DrawingTestGrid.Draw`. The lines are defined in normalized space [-1, +1] (+X right, +Y
//! down) and mapped to the image with the video size, which the original receives through `Scale`.

use crate::paint::helpers::frame::to_viewport;
use crate::paint::PaintContext;
use crate::renderer::Renderer;
use crate::style::StyleData;
use kinovea_geometry::{PointF, Size, SizeF};
use kinovea_metadata::singletons::TestGrid;

/// Half-width of the safe framing in normalized space (`margin = 0.8f`).
const FRAMING_MARGIN: f32 = 0.8;

/// Port of `DrawingTestGrid.Draw`: nothing is drawn while hidden or when the video has no size.
pub fn paint_test_grid(d: &TestGrid, ctx: &PaintContext, r: &mut dyn Renderer) {
    let image = ctx.metadata.video.image_size;
    if !d.visible || image == Size::default() {
        return;
    }
    let size = SizeF::new(image.width as f32, image.height as f32);
    // `styleData.GetPen(255)`: width 1, not stretched by the viewport.
    let pen = StyleData::from_elements(&d.header.style).marker_pen(255);
    for (a, b) in normalized_lines(d) {
        r.stroke_line(
            to_viewport(ctx, map(a, size)),
            to_viewport(ctx, map(b, size)),
            &pen,
        );
    }
}

/// `Map`: normalized coordinates to image coordinates.
fn map(p: PointF, size: SizeF) -> PointF {
    PointF::new(
        (p.x * 0.5 + 0.5) * size.width,
        (p.y * 0.5 + 0.5) * size.height,
    )
}

/// The enabled lines, in the drawing order of `DrawGrid`.
fn normalized_lines(d: &TestGrid) -> Vec<(PointF, PointF)> {
    let mut lines = Vec::new();
    if d.show_horizontal_axis {
        lines.push((PointF::new(-1.0, 0.0), PointF::new(1.0, 0.0)));
    }
    if d.show_vertical_axis {
        lines.push((PointF::new(0.0, -1.0), PointF::new(0.0, 1.0)));
    }
    if d.show_framing {
        lines.extend(framing());
    }
    if d.show_thirds {
        lines.extend(thirds());
    }
    lines
}

/// `frameLeft`, `frameTop`, `frameRight`, `frameBottom`.
fn framing() -> [(PointF, PointF); 4] {
    let m = FRAMING_MARGIN;
    let a = PointF::new(-m, -m);
    let b = PointF::new(m, -m);
    let c = PointF::new(m, m);
    let d = PointF::new(-m, m);
    [(a, d), (a, b), (b, c), (d, c)]
}

/// `thirdsLeft`, `thirdsTop`, `thirdsRight`, `thirdsBottom`.
fn thirds() -> [(PointF, PointF); 4] {
    let t: f32 = 1.0 / 3.0;
    [
        (PointF::new(-t, -1.0), PointF::new(-t, 1.0)),
        (PointF::new(-1.0, -t), PointF::new(1.0, -t)),
        (PointF::new(t, -1.0), PointF::new(t, 1.0)),
        (PointF::new(-1.0, t), PointF::new(1.0, t)),
    ]
}
