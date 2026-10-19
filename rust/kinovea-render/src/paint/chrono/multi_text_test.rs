use crate::paint::chrono::multi_text::build_text;
use kinovea_core::video_section::VideoSection;
use kinovea_metadata::drawing::{ChronoColumn, ChronoSection};

fn raw(t: i64) -> String {
    t.to_string()
}

fn section(start: i64, end: i64, name: &str, tag: &str) -> ChronoSection {
    ChronoSection {
        section: VideoSection { start, end },
        name: name.to_owned(),
        tag: tag.to_owned(),
    }
}

/// Section 1 is [0, 100] unnamed (shown as "1") with tag "A"; section 2 is [100, 300] named "Jump".
fn two_sections() -> Vec<ChronoSection> {
    vec![section(0, 100, "", "A"), section(100, 300, "Jump", "")]
}

const ALL: [ChronoColumn; 4] = [
    ChronoColumn::Name,
    ChronoColumn::Duration,
    ChronoColumn::Cumul,
    ChronoColumn::Tag,
];

#[test]
fn table_aligns_columns_and_marks_the_current_section() {
    // At 150: section 1 is over (duration 100, cumul 100); section 2 is current (duration 50,
    // cumul 150). Name width 4, duration and cumul width 3, tag width 1 (from section 1).
    // Each row starts with the marker, "▶ " for the current section and two spaces otherwise.
    let text = build_text(&two_sections(), 150, &ALL, raw);
    assert_eq!(text, "     1: 100 | 100 | A\n▶ Jump:  50 | 150 |  \n");
}

#[test]
fn only_the_visible_columns_are_written() {
    let text = build_text(
        &two_sections(),
        150,
        &[ChronoColumn::Name, ChronoColumn::Cumul],
        raw,
    );
    assert_eq!(text, "     1: 100\n▶ Jump: 150\n");
}

#[test]
fn before_the_second_section_shows_only_the_first_duration() {
    // Before the start of section 2 the text is the duration of section 1 alone: 50.
    let text = build_text(&two_sections(), 50, &ALL, raw);
    assert_eq!(text, "50");
}

#[test]
fn before_the_first_section_shows_zero() {
    let sections = vec![section(100, 200, "", "")];
    assert_eq!(build_text(&sections, 50, &ALL, raw), "0");
}

#[test]
fn a_single_section_past_its_end_shows_its_full_duration() {
    let sections = vec![section(0, 100, "", "")];
    assert_eq!(build_text(&sections, 500, &ALL, raw), "100");
}
