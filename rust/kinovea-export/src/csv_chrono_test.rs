use crate::case::{check_optional_file, measured};
use crate::csv::CsvFormat;
use crate::csv_chrono::write_csv_chrono;
use crate::measured_data::{TimeEntry, TimeSection};
use kinovea_metadata::drawing::ChronoColumn;

#[test]
fn matches_the_oracle_byte_for_byte_or_writes_no_file() {
    check_optional_file("chrono.csv", |md| {
        write_csv_chrono(md, &CsvFormat::new('.'))
    });
}

#[test]
fn cumulative_and_tag_rows_follow_the_visible_columns() {
    let mut md = measured("minimal");
    md.times = vec![
        TimeEntry {
            name: "c".to_owned(),
            sections: vec![section("a", "t\"x", 1.5, 1.5), section("b", "", 2.0, 3.5)],
            has_tags: true,
            visible_columns: vec![
                ChronoColumn::Duration,
                ChronoColumn::Cumul,
                ChronoColumn::Tag,
            ],
        },
        TimeEntry {
            name: "d".to_owned(),
            sections: vec![section("b", "", 2.0, 2.0)],
            has_tags: false,
            visible_columns: vec![ChronoColumn::Duration],
        },
    ];
    let text = write_csv_chrono(&md, &CsvFormat::new('.'));
    assert_eq!(
        text.as_deref(),
        Some(concat!(
            "\"\",\"a\",\"b\"\r\n",
            "\"c (Cumulative)\",\"1.5\",\"3.5\"\r\n",
            "\"c (Duration)\",\"1.5\",\"2\"\r\n",
            "\"c (Tag)\",\"t\"\"x\",\"\"\r\n",
            "\"d (Duration)\",\"\",\"2\"\r\n",
        ))
    );
}

fn section(name: &str, tag: &str, duration: f32, cumul: f32) -> TimeSection {
    TimeSection {
        name: name.to_owned(),
        tag: tag.to_owned(),
        duration,
        cumul,
        ..TimeSection::default()
    }
}
