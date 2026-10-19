//! The CSV chronometer writer (`ExporterCSVChrono`): one column per section name, and per
//! chronometer a cumulative row (when visible), a duration row and a tag row (when visible).

#[cfg(test)]
#[path = "csv_chrono_test.rs"]
mod csv_chrono_test;

use crate::csv::{lines_text, number_cell, row, text_cell, CsvFormat};
use crate::measured_data::{MeasuredData, TimeEntry, TimeSection};
use kinovea_metadata::drawing::ChronoColumn;

/// Header and the rows of every chronometer, or `None` when there is no chronometer (no file).
pub fn write_csv_chrono(md: &MeasuredData, format: &CsvFormat) -> Option<String> {
    if md.times.is_empty() {
        return None;
    }
    let names = section_names(md);
    let mut headers = vec![text_cell("")];
    headers.extend(names.iter().map(|name| text_cell(name)));
    let mut lines = vec![row(&headers, format)];
    for entry in &md.times {
        if entry.visible_columns.contains(&ChronoColumn::Cumul) {
            lines.push(section_row(entry, &names, "Cumulative", format, |s| {
                number_cell(s.cumul, format)
            }));
        }
        lines.push(section_row(entry, &names, "Duration", format, |s| {
            number_cell(s.duration, format)
        }));
        if entry.has_tags && entry.visible_columns.contains(&ChronoColumn::Tag) {
            lines.push(section_row(entry, &names, "Tag", format, |s| {
                text_cell(&s.tag)
            }));
        }
    }
    Some(lines_text(&lines))
}

/// Distinct section names in order of first appearance.
fn section_names(md: &MeasuredData) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for section in md.times.iter().flat_map(|entry| &entry.sections) {
        if !names.contains(&section.name) {
            names.push(section.name.clone());
        }
    }
    names
}

/// `<name> (<label>)` followed by one cell per section name: the value, or `""` when the
/// chronometer has no section of that name.
fn section_row(
    entry: &TimeEntry,
    names: &[String],
    label: &str,
    format: &CsvFormat,
    value: impl Fn(&TimeSection) -> String,
) -> String {
    let mut cells = vec![text_cell(&format!("{} ({label})", entry.name))];
    for name in names {
        cells.push(match entry.sections.iter().find(|s| &s.name == name) {
            Some(section) => value(section),
            None => text_cell(""),
        });
    }
    row(&cells, format)
}
