//! Writes the three schema samples through the Rust reader and writer into
//! `tests/cases/kva/crossload/`, where the oracle loads them back (`crossload` case).

use kinovea_core::preferences::dom::Element;
use kinovea_core::xml::parse_size;
use kinovea_geometry::Size;
use kinovea_metadata::ids::normalize_ids;
use kinovea_metadata::{read_kva, write_kva, ExportFlags, Metadata};
use std::path::Path;

fn main() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for case in ["minimal", "simple", "complete"] {
        let input =
            std::fs::read_to_string(repo.join(format!("Tools/XML/Schema/samples/{case}.kva")))
                .unwrap();
        let root = Element::parse(&input).unwrap();
        let (w, h) = root
            .child_text("ImageSize")
            .map(parse_size)
            .unwrap_or((0, 0));
        let mut m = Metadata::new(Size::new(w, h));
        m.video.average_timestamps_per_frame = root
            .child_text("AverageTimeStampsPerFrame")
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or(1.0);
        read_kva(&mut m, &input).unwrap();
        let out = normalize_ids(&write_kva(&m, "0.0", false, ExportFlags::FULL).to_document());
        let target = repo.join(format!("tests/cases/kva/crossload/{case}.kva"));
        std::fs::write(&target, out).unwrap();
        println!("wrote {}", target.display());
    }
}
