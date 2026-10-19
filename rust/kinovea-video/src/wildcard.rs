//! Case-insensitive `*` and `?` matching of file names, the subset of `Directory.GetFiles`
//! patterns used by `VideoTypeManager.GetMostRecentSupportedVideo`.

/// True when `text` matches `pattern`, where `*` matches any run of characters (including none)
/// and `?` matches exactly one character.
pub fn wildcard_match(pattern: &str, text: &str) -> bool {
    let pattern: Vec<char> = pattern.to_lowercase().chars().collect();
    let text: Vec<char> = text.to_lowercase().chars().collect();

    let (mut p, mut t) = (0, 0);
    // Last `*` seen, with the text position it currently absorbs up to.
    let mut star: Option<(usize, usize)> = None;

    while t < text.len() {
        if p < pattern.len() && (pattern[p] == '?' || pattern[p] == text[t]) {
            p += 1;
            t += 1;
        } else if p < pattern.len() && pattern[p] == '*' {
            star = Some((p, t));
            p += 1;
        } else if let Some((star_p, star_t)) = star {
            // Let the last `*` absorb one more character and retry.
            p = star_p + 1;
            t = star_t + 1;
            star = Some((star_p, star_t + 1));
        } else {
            return false;
        }
    }

    pattern[p..].iter().all(|&c| c == '*')
}
