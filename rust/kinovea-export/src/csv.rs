//! The CSV dialect of `CSVHelper`: quoted text cells, the list separator derived from the decimal
//! separator, numbers written with `float.ToString(nfi)`, and NaN as an empty unquoted cell.

#[cfg(test)]
#[path = "csv_test.rs"]
mod csv_test;

pub use crate::csv_chrono::write_csv_chrono;
pub use crate::csv_trajectory::write_csv_trajectory;

use crate::number_text::to_text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CsvFormat {
    pub decimal: char,
    pub list_separator: char,
}

impl CsvFormat {
    /// The list separator is `;` only when the decimal separator is `,` (`GetListSeparator`).
    pub fn new(decimal: char) -> Self {
        let list_separator = if decimal == ',' { ';' } else { ',' };
        CsvFormat {
            decimal,
            list_separator,
        }
    }
}

/// A text cell: the text between quotes, with quotes doubled (`WriteCell(string)`).
pub fn text_cell(text: &str) -> String {
    format!("\"{}\"", text.replace('"', "\"\""))
}

/// A number cell (`WriteCell(float, nfi)`): NaN is written as nothing, without quotes.
pub fn number_cell(value: f32, format: &CsvFormat) -> String {
    if value.is_nan() {
        String::new()
    } else {
        text_cell(&to_text(value, format.decimal))
    }
}

pub fn row(cells: &[String], format: &CsvFormat) -> String {
    cells.join(&format.list_separator.to_string())
}

/// Lines as `File.WriteAllLines` writes them: each one followed by CRLF.
pub fn lines_text(lines: &[String]) -> String {
    lines.iter().map(|line| format!("{line}\r\n")).collect()
}
