//! The CSV trajectory writer (`ExporterCSVTrajectory`): one table indexed by the sample times of
//! every tracked point, with two columns (X, Y) per point.

#[cfg(test)]
#[path = "csv_trajectory_test.rs"]
mod csv_trajectory_test;

use crate::csv::{lines_text, number_cell, row, text_cell, CsvFormat};
use crate::measured_data::MeasuredData;
use kinovea_geometry::PointF;

/// Header and one row per distinct time, or `None` when no series has a sample (no file).
pub fn write_csv_trajectory(md: &MeasuredData, format: &CsvFormat) -> Option<String> {
    let headers = headers(md);
    let samples = samples(md);
    if samples.is_empty() {
        return None;
    }
    let mut lines = vec![row(&headers, format)];
    for table_row in table(&samples, headers.len()) {
        let cells: Vec<String> = table_row
            .iter()
            .map(|value| number_cell(*value, format))
            .collect();
        lines.push(row(&cells, format));
    }
    Some(lines_text(&lines))
}

fn headers(md: &MeasuredData) -> Vec<String> {
    let mut headers = vec![text_cell("Time")];
    for series in &md.timeseries {
        for (key, _) in &series.data {
            headers.push(text_cell(&format!("{}/{key}/X", series.name)));
            headers.push(text_cell(&format!("{}/{key}/Y", series.name)));
        }
    }
    headers
}

/// `(time, column, point)` of every sample, sorted by time; the column of a point is 1 + 2 × its index.
fn samples(md: &MeasuredData) -> Vec<(f32, usize, PointF)> {
    let mut samples = Vec::new();
    let mut column = 1;
    for series in &md.timeseries {
        for (_, points) in &series.data {
            for (time, point) in series.times.iter().zip(points) {
                samples.push((*time, column, *point));
            }
            column += 2;
        }
    }
    samples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    samples
}

/// Consecutive samples with the same time share a row; a row starts as NaN everywhere but time.
fn table(samples: &[(f32, usize, PointF)], width: usize) -> Vec<Vec<f32>> {
    let mut rows: Vec<Vec<f32>> = Vec::new();
    for (time, column, point) in samples {
        if rows.last().map(|r| r[0]) != Some(*time) {
            let mut fresh = vec![f32::NAN; width];
            fresh[0] = *time;
            rows.push(fresh);
        }
        let current = rows.last_mut().expect("row");
        current[*column] = point.x;
        current[*column + 1] = point.y;
    }
    rows
}
