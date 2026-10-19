use super::rtf_to_text;

const SAMPLES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../Tools/XML/Schema/samples"
);
const EXPORTS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/export");

/// Text between the first `open` and the following `close`, or empty when either is missing.
fn between(text: &str, open: &str, close: &str) -> String {
    let rest = text.split_once(open).map_or("", |(_, rest)| rest);
    rest.split(close).next().unwrap_or_default().to_string()
}

/// `(title, raw comment)` of every keyframe in a KVA file. A keyframe without `<Comment>` has "".
fn kva_keyframes(xml: &str) -> Vec<(String, String)> {
    xml.split("<Keyframe>")
        .skip(1)
        .map(|chunk| {
            let title = between(chunk, "<Title>", "</Title>");
            let comment = between(chunk, "<Comment>", "</Comment>");
            let comment = comment
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&apos;", "'")
                .replace("&amp;", "&");
            (title, comment)
        })
        .collect()
}

/// Values of every `"key": "..."` string field in document order, with JSON escapes decoded.
fn json_strings(json: &str, key: &str) -> Vec<String> {
    let marker = format!("\"{key}\": \"");
    json.split(marker.as_str())
        .skip(1)
        .map(|rest| {
            let mut out = String::new();
            let mut chars = rest.chars();
            while let Some(c) = chars.next() {
                match c {
                    '"' => break,
                    '\\' => match chars.next() {
                        Some('n') => out.push('\n'),
                        Some('t') => out.push('\t'),
                        Some('r') => out.push('\r'),
                        Some('u') => {
                            let hex: String = chars.by_ref().take(4).collect();
                            let code = u32::from_str_radix(&hex, 16).unwrap();
                            out.push(char::from_u32(code).unwrap());
                        }
                        Some(other) => out.push(other),
                        None => break,
                    },
                    _ => out.push(c),
                }
            }
            out
        })
        .collect()
}

#[test]
fn plain_text_inside_group() {
    assert_eq!(rtf_to_text(r"{\rtf1\ansi Hola mundo}"), "Hola mundo");
}

#[test]
fn par_is_a_line_break() {
    assert_eq!(rtf_to_text(r"{\rtf1 uno\par dos}"), "uno\ndos");
}

#[test]
fn line_is_a_line_break() {
    assert_eq!(rtf_to_text(r"{\rtf1 uno\line dos}"), "uno\ndos");
}

#[test]
fn tab_is_a_tab() {
    assert_eq!(rtf_to_text(r"{\rtf1 a\tab b}"), "a\tb");
}

#[test]
fn hex_escape_uses_cp1252() {
    assert_eq!(rtf_to_text(r"{\rtf1 caf\'e9}"), "café");
    assert_eq!(rtf_to_text(r"{\rtf1 \'80\'93x\'94}"), "€“x”");
}

#[test]
fn unicode_escape_skips_fallback() {
    assert_eq!(rtf_to_text(r"{\rtf1 \u233?}"), "é");
    assert_eq!(rtf_to_text(r"{\rtf1 \u233 ?x}"), "éx");
}

#[test]
fn unicode_escape_skips_uc_characters() {
    assert_eq!(rtf_to_text(r"{\rtf1\uc2 a\u233\'e9b}"), "aé");
    assert_eq!(rtf_to_text(r"{\rtf1\uc0 a\u233?b}"), "aé?b");
}

#[test]
fn unicode_escape_negative_value() {
    assert_eq!(rtf_to_text(r"{\rtf1 \u-4\'3f}"), "\u{fffc}");
}

#[test]
fn nested_groups_keep_text_in_order() {
    assert_eq!(rtf_to_text(r"{\rtf1 a{\b b{\i c}d}e}"), "abcde");
}

#[test]
fn ignored_destinations_are_skipped() {
    let rtf = r"{\rtf1{\fonttbl{\f0 Arial;}}{\colortbl ;\red0\green0\blue0;}{\stylesheet{\s0 Normal;}}{\info{\title x}}{\*\generator Kinovea;}texto}";
    assert_eq!(rtf_to_text(rtf), "texto");
}

#[test]
fn escaped_symbols_are_literal() {
    assert_eq!(rtf_to_text(r"{\rtf1 \{a\}\\}"), "{a}\\");
}

#[test]
fn literal_newlines_in_source_are_ignored() {
    assert_eq!(rtf_to_text("{\\rtf1\r\nuno\\par\r\ndos}"), "uno\ndos");
}

#[test]
fn final_paragraph_mark_is_not_text() {
    assert_eq!(rtf_to_text(r"{\rtf1 uno\par}"), "uno");
    assert_eq!(rtf_to_text(r"{\rtf1\par}"), "");
    assert_eq!(rtf_to_text(r"{\rtf1 uno\par dos\par}"), "uno\ndos");
}

#[test]
fn inner_empty_paragraphs_are_kept() {
    assert_eq!(rtf_to_text(r"{\rtf1 uno\par\par dos}"), "uno\n\ndos");
}

#[test]
fn export_fixtures_comments_match_kva() {
    for case in ["complete", "simple", "minimal"] {
        let xml = std::fs::read_to_string(format!("{SAMPLES}/{case}.kva")).unwrap();
        let json = std::fs::read_to_string(format!("{EXPORTS}/{case}/measured.json")).unwrap();
        let kva = kva_keyframes(&xml);
        let names: Vec<String> = kva.iter().map(|(name, _)| name.clone()).collect();
        let comments: Vec<String> = kva.iter().map(|(_, rtf)| rtf_to_text(rtf)).collect();
        // Only the keyframes section: drawings also carry `name` fields.
        let keyframes = between(&json, "\"keyframes\"", "\"positions\"");
        assert_eq!(names, json_strings(&keyframes, "name"), "{case} names");
        assert_eq!(
            comments,
            json_strings(&keyframes, "comment"),
            "{case} comments"
        );
    }
}
