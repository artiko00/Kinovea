//! Port of `AlphanumComparator` (Kinovea.Services): names compare chunk by chunk, digit runs
//! of the same kind sorting by value, on the file name without its extension.

use std::cmp::Ordering;
use std::path::Path;

/// `Path.GetFileNameWithoutExtension`.
fn stem(name: &str) -> &str {
    Path::new(name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
}

/// Next chunk of `chars` from `pos`: a digit run or a run of non-digits.
fn chunk<'a>(chars: &'a [char], pos: &mut usize) -> &'a [char] {
    let start = *pos;
    let numeric = chars[start].is_ascii_digit();
    *pos += 1;
    while *pos < chars.len() && chars[*pos].is_ascii_digit() == numeric {
        *pos += 1;
    }
    &chars[start..*pos]
}

pub fn compare(x: &str, y: &str) -> Ordering {
    let a: Vec<char> = stem(x).chars().collect();
    let b: Vec<char> = stem(y).chars().collect();
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if i >= a.len() {
            return Ordering::Less;
        }
        if j >= b.len() {
            return Ordering::Greater;
        }
        let (ca, cb) = (chunk(&a, &mut i), chunk(&b, &mut j));
        let order = if ca[0].is_ascii_digit() && cb[0].is_ascii_digit() {
            // The original compares lengths first, then the digits of equal-length runs.
            ca.len().cmp(&cb.len()).then_with(|| ca.cmp(cb))
        } else {
            ca.iter().cmp(cb.iter())
        };
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}
