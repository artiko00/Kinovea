//! Drawing and keyframe identities: `Guid` text forms and the fixture normalization used by
//! the oracle (`Oracle.NormalizeIds`).

use uuid::Uuid;

/// A fresh random id, as `Guid.NewGuid()`.
pub fn new_id() -> Uuid {
    Uuid::new_v4()
}

/// `Guid.ToString()`: lowercase hyphenated.
pub fn id_text(id: Uuid) -> String {
    id.hyphenated().to_string()
}

/// Parses a `Guid` text; `None` when malformed.
pub fn parse_id(text: &str) -> Option<Uuid> {
    Uuid::parse_str(text.trim()).ok()
}

/// Replaces every non-empty GUID by `00000000-0000-4000-8000-<n>` in order of first appearance,
/// exactly as the oracle does before committing a fixture.
pub fn normalize_ids(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut seen: Vec<String> = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if i + 36 <= bytes.len() && is_guid(&text[i..i + 36]) {
            let guid = &text[i..i + 36];
            if guid == "00000000-0000-0000-0000-000000000000" {
                out.push_str(guid);
            } else {
                let n = match seen.iter().position(|g| g.eq_ignore_ascii_case(guid)) {
                    Some(p) => p + 1,
                    None => {
                        seen.push(guid.to_owned());
                        seen.len()
                    }
                };
                out.push_str(&format!("00000000-0000-4000-8000-{n:012}"));
            }
            i += 36;
        } else {
            let c = text[i..].chars().next().unwrap();
            out.push(c);
            i += c.len_utf8();
        }
    }
    out
}

fn is_guid(s: &str) -> bool {
    if s.len() != 36 {
        return false;
    }
    s.bytes().enumerate().all(|(i, b)| match i {
        8 | 13 | 18 | 23 => b == b'-',
        _ => b.is_ascii_hexdigit(),
    })
}
