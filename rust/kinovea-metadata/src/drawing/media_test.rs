use crate::drawing::line_test::body;
use crate::drawing::{element, Bitmap, KvaDrawing, Svg};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::{PointF, RectangleF};
use uuid::Uuid;

const FADING: &str = "  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n";

#[test]
fn sticker_and_embedded_bitmaps_round_trip() {
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    let sticker = format!("<Bitmap id=\"00000000-0000-4000-8000-000000000001\" name=\"Sticker 1\">\n  <File />\n  <BoundingBox>10;20;64;64</BoundingBox>\n  <Sticker>true</Sticker>\n  <StickerReference>smile</StickerReference>\n{FADING}</Bitmap>");
    let b = Bitmap::read_kva(
        &Element::parse(&sticker).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    assert!(b.is_sticker);
    assert_eq!(body(element(&b, Filter::KVA, &refs).to_document()), sticker);
    let embedded = format!("<Bitmap id=\"00000000-0000-4000-8000-000000000002\" name=\"Image\">\n  <File />\n  <BoundingBox>1;2;3;4</BoundingBox>\n  <Sticker>false</Sticker>\n  <Bitmap>iVBORw0KGgo=</Bitmap>\n{FADING}</Bitmap>");
    let truncating = embedded
        .replace("1;2;3;4", "1.7;2.2;3.9;4.1")
        .replace("<File />", "<File>old.png</File>");
    let e = Bitmap::read_kva(
        &Element::parse(&truncating).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    assert_eq!(e.bounding_box, RectangleF::new(1.0, 2.0, 3.0, 4.0));
    assert_eq!(e.file, "", "an embedded payload clears the file name");
    assert_eq!(
        body(element(&e, Filter::KVA, &refs).to_document()),
        embedded
    );
    assert_ne!(e.content_hash(), b.content_hash());
}

#[test]
fn svg_keeps_only_the_file_and_fading() {
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    let frag = format!("<SVG id=\"00000000-0000-4000-8000-000000000003\" name=\"Logo\">\n  <File>C:\\logo.svg</File>\n{FADING}</SVG>");
    let s = Svg::read_kva(&Element::parse(&frag).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(s.file, "C:\\logo.svg");
    assert_eq!(body(element(&s, Filter::KVA, &refs).to_document()), frag);
    assert_eq!(s.content_hash(), 0);
}
