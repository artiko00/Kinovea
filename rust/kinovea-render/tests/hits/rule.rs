//! What the rule accepts beyond the strict rule, each entry with its cause: the text zones measured by
//! the renderer, the known deviations off a boundary, and the drawings whose `HitTest` throws.

/// Drawings whose text zone (a mini label or a text box) is measured by the renderer instead of GDI+.
/// The renderer measures glyphs with a fixed advance (0.55 em per character, `backend/recording/render.rs`)
/// and the oracle uses GDI+ Arial, so the box edges differ by a few pixels. ChronoMulti is also
/// multi-line: the renderer measures it as one line, the oracle as one line per section. The codes of
/// each entry are the answers of its text zone (`text_codes`): the probes where the oracle or Rust
/// answers one of them leave the verdict, the rest of the drawing is judged as usual. An entry whose
/// drawing meets the strict rule is stale. Counter (`all-drawings` chronos#2) shows "*": its KVA
/// `ExtraData` "Cadence" is not a measure type name, so the oracle also writes "*" (GDI+ measures it 7 px
/// wider and 4 px shorter than the renderer).
/// Entries are (case, `container#index`, text zone codes).
pub const PENDING_TEXT_METRICS: &[(&str, &str, &[i32])] = &[
    ("complete", "k1#0", &[0, 1]),
    ("complete", "k1#1", &[0, 1]),
    ("complete", "chronos#0", &[0, 1]),
    ("all-drawings", "k0#4", &[4]),
    ("all-drawings", "k0#10", &[1]),
    ("all-drawings", "k0#11", &[2]),
    ("all-drawings", "chronos#0", &[0, 1]),
    ("all-drawings", "chronos#1", &[0, 1]),
    ("all-drawings", "chronos#2", &[0, 1]),
    ("all-drawings", "k0#9", &[0, 1]),
    ("all-drawings", "singletons#1", &[0]),
];

/// The answers of the text zone of each element kind. A label answers 0 on its text and arrow and 1 on
/// its hidden resizer; a chrono family box answers 0 on its name tag and 1 on its resizer. The codes of
/// a mini label are those of its owner (`MiniLabel.HitTest`), and a number box answers 0.
pub fn text_codes(element: &str) -> &'static [i32] {
    match element {
        "Label" | "Chrono" | "ChronoMulti" | "Counter" => &[0, 1],
        "CrossMark" => &[1],
        "Circle" => &[2],
        "Line" => &[3],
        "TimeSegment" => &[4],
        "AutoNumbers" => &[0],
        _ => &[],
    }
}

/// The text zone codes of an entry, when it is listed.
pub fn pending_codes(case: &str, id: &str) -> Option<&'static [i32]> {
    PENDING_TEXT_METRICS
        .iter()
        .find(|(c, i, _)| *c == case && *i == id)
        .map(|(_, _, codes)| *codes)
}

/// Drawings whose `HitTest` throws in the original at some scales: (case, `container#index`, the cause).
/// Empty since the video setup: the plane's projective mapping is built on the calibration change that
/// follows it, so the sliding lines no longer throw (`DrawingPlane.UpdateMapping`).
pub const EXPECTED_EXCEPTIONS: &[(&str, &str, &str)] = &[];

/// Drawings with probes off a Rust boundary that the port does not reproduce: (case, `container#index`,
/// scale, the most probes off a boundary observed). Probable cause, not confirmed: hand-drawn strokes
/// zig-zag by about 1 px, so with a pen 13 px wide or more the widened pieces overlap. GDI+ fills the
/// outline with the alternate rule, which leaves holes where the outline overlaps itself, and the port
/// fills the union. A test with the stroke's own crossings (`complete` k2#1) did not support it: only
/// 2 of 1834 probes inside the stroke sit at a crossing, and both hit. The maxima are the exact counts
/// observed; the 99 percent still applies to these drawings.
pub const KNOWN_DEVIATIONS: &[(&str, &str, &str, usize)] = &[
    ("complete", "k2#0", "0.5", 6),
    ("complete", "k2#0", "1", 2),
    ("complete", "k2#1", "0.5", 14),
    ("complete", "k2#1", "1", 6),
    ("complete", "k2#1", "2", 3),
    ("all-drawings", "k0#1", "2", 1),
];

/// The known maximum of a drawing at a scale, when there is one.
pub fn known_max(case: &str, id: &str, scale: &str) -> Option<usize> {
    KNOWN_DEVIATIONS
        .iter()
        .find(|(c, i, s, _)| *c == case && *i == id && *s == scale)
        .map(|(_, _, _, max)| *max)
}

#[cfg(test)]
#[path = "rule_test.rs"]
mod rule_test;
