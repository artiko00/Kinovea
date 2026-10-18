use super::dom::Element;

const DOC: &str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<Root>\n  <A>1 &amp; 2</A>\n  <Empty />\n  <List>\n    <Item key=\"x\">v</Item>\n  </List>\n</Root>";

#[test]
fn parse_then_write_is_identity() {
    let root = Element::parse(DOC).unwrap();
    assert_eq!(root.name, "Root");
    assert_eq!(root.child_text("A"), Some("1 & 2"));
    assert_eq!(root.child("Empty").unwrap().text, "");
    let item = root.child("List").unwrap().child("Item").unwrap();
    assert_eq!(item.attribute("key"), Some("x"));
    assert_eq!(root.to_document(), DOC);
}

#[test]
fn building_elements() {
    let mut root = Element::new("Root");
    root.push_leaf("Culture", "").push_leaf("Flag", "true");
    assert_eq!(
        root.to_document(),
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<Root>\n  <Culture />\n  <Flag>true</Flag>\n</Root>"
    );
}

#[test]
fn malformed_input_is_an_error() {
    assert!(Element::parse("<Root><A></Root>").is_err());
    assert!(Element::parse("").is_err());
}

#[test]
fn text_content_keeps_inner_and_edge_spaces_but_drops_layout_whitespace() {
    let root = crate::preferences::dom::Element::parse("<R>\n  <A> a  b </A>\n  <B>x</B>\n</R>").unwrap();
    assert_eq!(root.child_text("A"), Some(" a  b "));
    assert_eq!(root.text, "");
    assert_eq!(root.children.len(), 2);
}
