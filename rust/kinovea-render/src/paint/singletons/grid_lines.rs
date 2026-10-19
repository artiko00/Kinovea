//! Port of `CreateVerticalGridLines` and `CreateHorizontalGridLines`. Both walk the plane from the origin
//! in steps, project each line, clip it to the image and record a graduation for the visible ones. They
//! differ only in the axis the lines run along, which `Walk` describes.

use super::grid::{CoordinateSystemGrid, GridLine};
use super::tick::{TextAlignment, TickMark};
use kinovea_geometry::{clip_line, clip_segment, PointF, Quadrilateral, RectangleF, SizeF};
use kinovea_measurement::CalibratorPlane;

/// What the walk needs from the calibration: the plane, the image clip, the extended plane in
/// plane space and whether the image has no vanishing point inside (`orthogonal`).
pub struct PlaneFrame<'a> {
    pub calibrator: &'a CalibratorPlane,
    pub clip: RectangleF,
    pub plane: SizeF,
    pub extended: Quadrilateral,
    pub orthogonal: bool,
}

/// Axis of a walk: vertical lines sit at `x = t`, horizontal ones at `y = t`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Walk {
    Vertical,
    Horizontal,
}

impl Walk {
    /// Plane point of the line at `t` (across it) and `s` (along it).
    fn point(self, t: f32, s: f32) -> PointF {
        match self {
            Walk::Vertical => PointF::new(t, s),
            Walk::Horizontal => PointF::new(s, t),
        }
    }

    /// Range of `t` over the extended plane.
    fn range(self, ext: &Quadrilateral) -> (f32, f32) {
        let [a, b, _, d] = ext.vertices;
        match self {
            Walk::Vertical => (a.x, b.x),
            Walk::Horizontal => (d.y, a.y),
        }
    }

    /// Along-line positions of the extended plane for the forward and backward ends of a line, used
    /// when the vanishing point is visible.
    fn ends(self, ext: &Quadrilateral) -> (f32, f32) {
        let [a, b, _, d] = ext.vertices;
        match self {
            Walk::Vertical => (a.y, d.y),
            Walk::Horizontal => (b.x, a.x),
        }
    }

    /// Length of the plane along the line.
    fn length(self, plane: SizeF) -> f32 {
        match self {
            Walk::Vertical => plane.height,
            Walk::Horizontal => plane.width,
        }
    }

    /// Offset of the calibration along the walk's `t`.
    fn offset(self, offset: PointF) -> f32 {
        match self {
            Walk::Vertical => offset.x,
            Walk::Horizontal => offset.y,
        }
    }

    /// Graduation alignment: the origin of the vertical axis is labelled too, the horizontal one is not.
    fn tick_alignment(self, is_axis: bool) -> Option<TextAlignment> {
        match self {
            Walk::Vertical if is_axis => Some(TextAlignment::BottomRight),
            Walk::Vertical => Some(TextAlignment::Bottom),
            Walk::Horizontal => (!is_axis).then_some(TextAlignment::Left),
        }
    }

    fn axis(self, grid: &mut CoordinateSystemGrid) -> &mut Option<GridLine> {
        match self {
            Walk::Vertical => &mut grid.vertical_axis,
            Walk::Horizontal => &mut grid.horizontal_axis,
        }
    }
}

/// `CreateVerticalGridLines` / `CreateHorizontalGridLines`: `vanish` is the image point of the walk's
/// direction at infinity, when visible.
pub fn create_lines(
    grid: &mut CoordinateSystemGrid,
    frame: &PlaneFrame,
    walk: Walk,
    start: f32,
    step: f32,
    vanish: Option<PointF>,
) {
    let (lo, hi) = walk.range(&frame.extended);
    let (forward, backward) = walk.ends(&frame.extended);
    let length = walk.length(frame.plane);
    let mut t = start;
    let mut partly_visible = true;
    while partly_visible && t >= lo && t <= hi {
        let pa = frame.calibrator.project(walk.point(t, 0.0));
        let pb = frame.calibrator.project(walk.point(t, length));
        if pa.z < 0.0 || pb.z < 0.0 {
            t += step;
            continue;
        }
        let a = PointF::new(pa.x / pa.z, pa.y / pa.z);
        let b = PointF::new(pb.x / pb.z, pb.y / pb.z);
        let result = match vanish {
            // The vanishing point is past b: clip from a towards the forward end.
            Some(v) if (b.x - a.x) / (v.x - a.x) > 0.0 => {
                let vb = frame.calibrator.untransform(walk.point(t, forward));
                clip_line(frame.clip, a, vb, f64::MIN, 1.0)
            }
            // The vanishing point is before a: clip from the backward end towards b.
            Some(_) => {
                let va = frame.calibrator.untransform(walk.point(t, backward));
                clip_line(frame.clip, va, b, 0.0, f64::MAX)
            }
            None => clip_segment(frame.clip, a, b),
        };
        partly_visible = result.visible;
        if partly_visible {
            let is_axis = t == 0.0;
            let line = GridLine {
                start: result.a,
                end: result.b,
            };
            if is_axis {
                *walk.axis(grid) = Some(line);
            } else {
                grid.grid_lines.push(line);
            }
            if let Some(alignment) = walk.tick_alignment(is_axis) {
                if frame.clip.contains(a)
                    && (frame.orthogonal || tick_mark_visible((t / step) as i32))
                {
                    grid.tick_marks.push(TickMark {
                        value: t + walk.offset(frame.calibrator.offset),
                        location: a,
                        alignment,
                    });
                }
            }
        }
        t += step;
    }
}

/// `TickMarkVisible`: decimates the graduations far from the origin to avoid crowding.
pub fn tick_mark_visible(step: i32) -> bool {
    step <= 5 || (step as f64).sqrt() % 1.0 == 0.0
}
