use crate::import_test::{assert_matches_oracle, document, kva_text, repo};
use crate::srt::{import_srt, strip_tags};
use kinovea_metadata::{Drawing, Keyframe, Metadata};

fn clip_text() -> String {
    std::fs::read_to_string(repo().join("tests/data/import/clip.srt")).unwrap()
}

fn label_text(k: &Keyframe) -> String {
    match &k.drawings[0] {
        Drawing::Label(l) => l.text.clone(),
        other => panic!("expected a label, found {}", other.kva_name()),
    }
}

#[test]
fn clip_matches_the_oracle_with_one_label_per_cue() {
    let mut m = document("srt");
    let report = import_srt(&mut m, &clip_text()).unwrap();
    assert_matches_oracle(&m, "srt");
    assert_eq!(report.keyframes, 3);
    assert_eq!(report.drawings, 3);
    let starts: Vec<i64> = m.keyframes.iter().map(|k| k.timestamp).collect();
    assert_eq!(starts, vec![4800, 20000, 40000]);
}

#[test]
fn cue_text_keeps_one_line_per_text_line_and_drops_tags() {
    let mut m = document("srt");
    import_srt(&mut m, &clip_text()).unwrap();
    let texts: Vec<String> = m.keyframes.iter().map(label_text).collect();
    assert_eq!(
        texts,
        vec!["Hello world\n", "Second\nline two\n", "Third\n"]
    );
}

#[test]
fn formatting_tags_are_removed_and_lone_angle_brackets_kept() {
    assert_eq!(
        strip_tags("<i>Hello</i> <font color=\"#ff0000\">there</font>"),
        "Hello there"
    );
    assert_eq!(strip_tags("a < b"), "a < b");
}

#[test]
fn block_without_text_is_ignored_and_the_next_cue_is_kept() {
    let text = "1\n00:00:01,000 --> 00:00:02,000\n\n2\n00:00:03,000 --> 00:00:04,000\nHi\n";
    let mut m = document("srt");
    let report = import_srt(&mut m, text).unwrap();
    assert_eq!(report.keyframes, 1);
    assert_eq!(label_text(&m.keyframes[0]), "Hi\n");
}

#[test]
fn windows_line_endings_import_like_unix_ones() {
    let lf = clip_text();
    let crlf = lf.replace('\n', "\r\n");
    let mut a: Metadata = document("srt");
    let mut b: Metadata = document("srt");
    import_srt(&mut a, &lf).unwrap();
    import_srt(&mut b, &crlf).unwrap();
    assert_eq!(kva_text(&a), kva_text(&b));
}
