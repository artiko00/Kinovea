//! The Markdown report (`ExporterMarkdown` with no images in the original's rich text control
//! path): the video name, then per keyframe inside the selection its title, its time code, its
//! image when there are images, and its comment.

#[cfg(test)]
#[path = "markdown_test.rs"]
mod markdown_test;

use crate::measured_data::keyframe_name;
use kinovea_metadata::Metadata;
use std::path::Path;

const LINE_BREAK: &str = "  \r\n";
const PARAGRAPH_BREAK: &str = "\r\n\r\n";

/// `image_paths[i]` is the image of the i-th keyframe written; no paths means no images.
pub fn write_markdown(
    metadata: &Metadata,
    image_paths: &[String],
    timecode: impl Fn(i64) -> String,
    rtf_to_text: fn(&str) -> String,
) -> String {
    let video = &metadata.video;
    let stem = Path::new(&video.video_path)
        .file_stem()
        .map_or_else(String::new, |s| s.to_string_lossy().into_owned());
    let mut out = format!("# {stem}{LINE_BREAK}{PARAGRAPH_BREAK}");
    let selected = metadata
        .keyframes
        .iter()
        .filter(|k| k.timestamp >= video.selection_start && k.timestamp <= video.selection_end);
    for (index, keyframe) in selected.enumerate() {
        out += &format!("## {}{LINE_BREAK}", keyframe_name(keyframe, &timecode));
        out += &format!("*{}*{PARAGRAPH_BREAK}", timecode(keyframe.timestamp));
        if !image_paths.is_empty() {
            out += &format!("![]({}){{width=100%}}{PARAGRAPH_BREAK}", image_paths[index]);
        }
        let text = rtf_to_text(&keyframe.comments).replace('\n', LINE_BREAK);
        out += &format!("{text}{PARAGRAPH_BREAK}");
    }
    out
}
