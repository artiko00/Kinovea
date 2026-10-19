use super::{lines_text, number_cell, row, text_cell, CsvFormat};

#[test]
fn list_separator_is_semicolon_only_with_comma_decimal() {
    assert_eq!(CsvFormat::new('.').list_separator, ',');
    assert_eq!(CsvFormat::new(',').list_separator, ';');
}

#[test]
fn text_cells_are_quoted_with_doubled_quotes() {
    assert_eq!(text_cell("t\"x"), "\"t\"\"x\"");
    assert_eq!(text_cell(""), "\"\"");
}

#[test]
fn nan_is_an_empty_unquoted_cell() {
    assert_eq!(number_cell(f32::NAN, &CsvFormat::new('.')), "");
    assert_eq!(number_cell(0.5, &CsvFormat::new(',')), "\"0,5\"");
}

#[test]
fn rows_join_with_the_list_separator_and_lines_end_with_crlf() {
    let format = CsvFormat::new('.');
    let cells = ["\"a\"".to_owned(), "\"b\"".to_owned()];
    assert_eq!(row(&cells, &format), "\"a\",\"b\"");
    assert_eq!(lines_text(&["x".to_owned(), "y".to_owned()]), "x\r\ny\r\n");
}
