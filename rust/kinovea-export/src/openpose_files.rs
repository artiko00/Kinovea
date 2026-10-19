//! File discovery for an OpenPose folder: the prefix of a sequence, the frame number of each file
//! name, and the files of a sequence in the original's order.

use crate::import::ImportError;
use crate::openpose_sort::compare;
use std::path::Path;

/// `GetFilePrefix`: the name before `_<frame>_keypoints.json`, or `None` for other names.
pub(crate) fn keypoints_prefix(name: &str) -> Option<&str> {
    let tokens: Vec<&str> = name.split('_').collect();
    if tokens.len() < 3 || tokens[tokens.len() - 1] != "keypoints.json" {
        return None;
    }
    let suffix = tokens[tokens.len() - 1].len() + tokens[tokens.len() - 2].len() + 2;
    name.get(..name.len().checked_sub(suffix)?)
}

/// Files of `folder` whose name starts with `prefix`, in the order of the original (`AlphanumComparator`).
pub(crate) fn frame_files(folder: &Path, prefix: &str) -> Result<Vec<String>, ImportError> {
    let mut names = Vec::new();
    for entry in std::fs::read_dir(folder)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(prefix) && entry.file_type()?.is_file() {
            names.push(name);
        }
    }
    names.sort_by(|a, b| compare(a, b));
    Ok(names)
}

/// The token before `keypoints.json`, read as a frame number.
pub(crate) fn frame_number(name: &str) -> Option<i32> {
    name.split('_').rev().nth(1)?.parse().ok()
}
