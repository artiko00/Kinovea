//! Read-then-write functions per KVA kind, and the seeded loop that checks idempotence.

use super::gen::Rng;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Drawing;
use kinovea_metadata::keyframe::Keyframe;
use kinovea_metadata::kva_io::{identity, Filter, Scale};
use kinovea_metadata::multi::{NumberSequence, Spotlights};
use kinovea_metadata::singletons::{singleton_element, TestGrid};
use kinovea_metadata::{header_metadata, read_kva, write_kva, ExportFlags};
use uuid::Uuid;

pub const SEEDS: u64 = 64;

/// Generates one fragment per seed, then reads and writes it twice: the second write must
/// equal the first one. `round` maps a document to the document written back after a read.
pub fn check(generate: fn(&mut Rng) -> Element, round: &dyn Fn(&str) -> String) {
    for seed in 0..SEEDS {
        let fragment = generate(&mut Rng::new(seed)).to_document();
        let first = round(&fragment);
        let second = round(&first);
        assert_eq!(first, second, "seed {seed}\nfragment:\n{fragment}");
    }
}

fn parse(xml: &str) -> Element {
    Element::parse(xml).expect("well-formed fragment")
}

/// Trackable points of the drawings, keyed by drawing id, as the trackability manager holds them.
fn reference(drawings: &[Drawing]) -> Vec<(Uuid, String, PointF)> {
    drawings
        .iter()
        .flat_map(|d| {
            d.trackable_points()
                .into_iter()
                .map(move |(k, p)| (d.id(), k, p))
        })
        .collect()
}

fn lookup(table: &[(Uuid, String, PointF)], id: Uuid, key: &str) -> PointF {
    table
        .iter()
        .find(|(i, k, _)| *i == id && k == key)
        .map_or(PointF::EMPTY, |(_, _, p)| *p)
}

pub fn drawing_round(kind: &str, xml: &str) -> String {
    let d = Drawing::read_kva(&parse(xml), Scale::IDENTITY, &identity).expect("known kind");
    assert_eq!(d.kva_name(), kind);
    let table = reference(std::slice::from_ref(&d));
    let refs = |id: Uuid, key: &str| lookup(&table, id, key);
    d.to_element(Filter::KVA, &refs).to_document()
}

fn timecode(t: i64) -> String {
    t.to_string()
}

pub fn keyframe_round(xml: &str) -> String {
    let kf = Keyframe::read_kva(&parse(xml), Scale::IDENTITY, &identity);
    let table = reference(&kf.drawings);
    let refs = |id: Uuid, key: &str| lookup(&table, id, key);
    kf.to_element(Filter::KVA, &refs, timecode).to_document()
}

pub fn spotlights_round(xml: &str) -> String {
    let mut s = Spotlights::default();
    s.read_kva(&parse(xml), Scale::IDENTITY, &identity);
    let mut e = Element::new("Spotlights");
    e.children = s.write_kva(Filter::KVA);
    e.to_document()
}

pub fn numbers_round(xml: &str) -> String {
    let mut n = NumberSequence::default();
    n.read_kva(&parse(xml), Scale::IDENTITY, &identity);
    let mut e = Element::new("NumberSequence");
    e.children = n.write_kva(Filter::KVA);
    e.to_document()
}

/// Reads the fragment inside a whole document, with the tracker its drawing has in a file, and
/// writes the document back. The `CoordinateSystem` element is what the document writes for it.
pub fn coordinate_round(xml: &str) -> String {
    let fragment = parse(xml);
    let id = fragment.attribute("id").unwrap_or_default();
    let position = fragment.child_text("Position").unwrap_or("0;0");
    let doc = format!(
        "<KinoveaVideoAnalysis><FormatVersion>2.1</FormatVersion><ImageSize>640;480</ImageSize>{xml}\
         <Trackability><TrackableDrawing id=\"{id}\" name=\"\"><TrackablePoint key=\"0\">\
         <ReferenceTimestamp>0</ReferenceTimestamp><ReferenceValue>{position}</ReferenceValue>\
         </TrackablePoint></TrackableDrawing></Trackability></KinoveaVideoAnalysis>"
    );
    let mut m = header_metadata(&doc);
    read_kva(&mut m, &doc).expect("well-formed KVA");
    let written = write_kva(&m, "0.0", false, ExportFlags::FULL);
    written
        .children
        .iter()
        .find(|c| c.name == "CoordinateSystem")
        .expect("the document writes its coordinate system")
        .to_document()
}

pub fn test_grid_round(xml: &str) -> String {
    let mut grid = TestGrid::default();
    grid.read_kva(&parse(xml));
    let children = grid.write_kva(Filter::KVA);
    singleton_element(TestGrid::KVA_NAME, &grid.header, children).to_document()
}
