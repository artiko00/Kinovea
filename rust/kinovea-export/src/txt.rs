//! The TXT trajectory writer (`ExporterTXTTrajectory`): per series a `# name` header, one line per
//! sample with the time and the X, Y of each point, and a blank line.

#[cfg(test)]
#[path = "txt_test.rs"]
mod txt_test;

use crate::csv::{lines_text, CsvFormat};
use crate::measured_data::MeasuredData;
use crate::number_text::to_text;

pub fn write_txt_trajectory(md: &MeasuredData, format: &CsvFormat) -> String {
    let number = |value: f32| to_text(value, format.decimal);
    let mut lines = vec![
        "#Kinovea Trajectory data export".to_owned(),
        "#T X Y".to_owned(),
        String::new(),
    ];
    for series in &md.timeseries {
        lines.push(format!("# {}", series.name));
        for (i, time) in series.times.iter().enumerate() {
            let mut cells = vec![number(*time)];
            for (_, points) in &series.data {
                cells.push(number(points[i].x));
                cells.push(number(points[i].y));
            }
            lines.push(cells.join(" "));
        }
        lines.push(String::new());
    }
    lines_text(&lines)
}
