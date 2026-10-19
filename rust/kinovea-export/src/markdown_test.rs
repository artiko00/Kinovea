use crate::case::{cases_with_fixture, fixture_text, load_case};
use crate::markdown::write_markdown;
use crate::rtf::rtf_to_text;

/// The raw timestamp, as the oracle's time code builder.
fn timestamp(ts: i64) -> String {
    ts.to_string()
}

#[test]
fn matches_the_oracle_without_images() {
    let cases = cases_with_fixture("report.md");
    assert!(!cases.is_empty());
    for name in cases {
        let loaded = load_case(&name);
        let got = write_markdown(&loaded.metadata, &[], timestamp, rtf_to_text);
        assert_eq!(got, fixture_text(&name, "report.md"), "{name}");
    }
}

#[test]
fn image_paths_follow_each_written_keyframe() {
    let loaded = load_case("complete");
    let paths: Vec<String> = (0..9).map(|i| format!("images/{i}.png")).collect();
    let text = write_markdown(&loaded.metadata, &paths, timestamp, rtf_to_text);
    assert!(text.starts_with(concat!(
        "# complete  \r\n\r\n\r\n",
        "## comments  \r\n*48048*\r\n\r\n",
        "![](images/0.png){width=100%}\r\n\r\n",
        "Comments  \r\nComments with formatting\r\n\r\n",
    )));
    assert!(text.contains("## labels  \r\n*106606*\r\n\r\n![](images/1.png){width=100%}\r\n\r\n"));
}

#[test]
fn keyframes_before_the_selection_are_skipped_and_images_stay_consecutive() {
    let mut loaded = load_case("complete");
    loaded.metadata.video.selection_start = 200_000;
    let paths: Vec<String> = (0..6).map(|i| format!("images/{i}.png")).collect();
    let text = write_markdown(&loaded.metadata, &paths, timestamp, rtf_to_text);
    assert!(text.starts_with(concat!(
        "# complete  \r\n\r\n\r\n",
        "## lines  \r\n*205705*\r\n\r\n",
        "![](images/0.png){width=100%}\r\n\r\n",
    )));
    assert!(!text.contains("## comments"));
}

#[test]
fn a_keyframe_without_title_takes_its_time_code() {
    let mut loaded = load_case("complete");
    loaded.metadata.keyframes[0].name.clear();
    let text = write_markdown(&loaded.metadata, &[], timestamp, rtf_to_text);
    assert!(text.starts_with("# complete  \r\n\r\n\r\n## 48048  \r\n*48048*\r\n\r\n"));
}
