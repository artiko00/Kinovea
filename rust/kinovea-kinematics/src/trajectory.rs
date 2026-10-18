//! A tracked trajectory in world coordinates with optional Butterworth filtering per axis and
//! its least-squares circle (`FilteredTrajectory`).

use crate::butterworth::filter_samples;
use crate::context::MeasurementContext;
use crate::filtering_result::FilteringResult;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::shapes::Circle;
use kinovea_geometry::{fit_circle, PointF};

/// Number of cutoff frequencies tried per axis.
pub const CUTOFF_TESTS: usize = 100;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FilteredTrajectory {
    pub times: Vec<i64>,
    pub raw_xs: Vec<f64>,
    pub raw_ys: Vec<f64>,
    /// `EnableFiltering && samples > 10`.
    pub can_filter: bool,
    pub filter_result_xs: Vec<FilteringResult>,
    pub filter_result_ys: Vec<FilteringResult>,
    /// Index into `filter_result_xs`; `None` when not filtered (the original's −1).
    pub x_cutoff_index: Option<usize>,
    pub y_cutoff_index: Option<usize>,
    pub best_fit_circle: Circle,
}

impl FilteredTrajectory {
    /// Maps the samples to world space through `ctx`, filters when allowed and fits the circle.
    pub fn new(samples: &[TimedPoint], ctx: &dyn MeasurementContext) -> Self {
        let mut t = FilteredTrajectory {
            times: samples.iter().map(|s| s.t).collect(),
            ..Default::default()
        };
        for s in samples {
            let p = ctx.point_at_time(PointF::new(s.x, s.y), s.t);
            t.raw_xs.push(f64::from(p.x));
            t.raw_ys.push(f64::from(p.y));
        }
        t.can_filter = ctx.filtering_enabled() && samples.len() > 10;
        if t.can_filter {
            let fps = ctx.capture_fps();
            if let Ok(f) = filter_samples(&t.raw_xs, fps, CUTOFF_TESTS) {
                t.filter_result_xs = f.results;
                t.x_cutoff_index = f.best_cutoff_index;
            }
            if let Ok(f) = filter_samples(&t.raw_ys, fps, CUTOFF_TESTS) {
                t.filter_result_ys = f.results;
                t.y_cutoff_index = f.best_cutoff_index;
            }
        }
        let points: Vec<PointF> = (0..t.length()).map(|i| t.coordinates(i)).collect();
        t.best_fit_circle = fit_circle(&points);
        t
    }

    pub fn length(&self) -> usize {
        self.times.len()
    }

    /// Filtered X series when a cutoff was selected, raw otherwise.
    pub fn xs(&self) -> &[f64] {
        self.x_cutoff_index.map_or(&self.raw_xs, |i| &self.filter_result_xs[i].data)
    }

    /// Filtered Y series when a cutoff was selected, raw otherwise.
    pub fn ys(&self) -> &[f64] {
        self.y_cutoff_index.map_or(&self.raw_ys, |i| &self.filter_result_ys[i].data)
    }

    /// Raw world coordinates; indices past the end return the last sample.
    pub fn raw_coordinates(&self, index: usize) -> PointF {
        let i = index.min(self.length() - 1);
        PointF::new(self.raw_xs[i] as f32, self.raw_ys[i] as f32)
    }

    /// Filtered coordinates per axis when available, raw otherwise; past the end → last raw.
    pub fn coordinates(&self, index: usize) -> PointF {
        if index >= self.length() {
            return self.raw_coordinates(index);
        }
        let raw = self.raw_coordinates(index);
        let pick = |cutoff: Option<usize>, results: &[FilteringResult], fallback: f32| match cutoff {
            None => fallback,
            Some(c) if results[c].data.len() > index => results[c].data[index] as f32,
            Some(_) => 0.0,
        };
        PointF::new(
            pick(self.x_cutoff_index, &self.filter_result_xs, raw.x),
            pick(self.y_cutoff_index, &self.filter_result_ys, raw.y),
        )
    }
}
