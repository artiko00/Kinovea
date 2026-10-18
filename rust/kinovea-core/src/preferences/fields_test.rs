use super::dom::Element;
use super::fields::xml_struct;
use super::helpers::XmlField;

#[derive(Default, PartialEq, Debug)]
struct Probe {
    a: i32,
    b: bool,
}

xml_struct!(Probe { a => "A", b => "B" });

#[test]
fn generated_mapping_round_trips_in_order() {
    let mut p = Probe::default();
    p.read(&Element::parse("<P><B>true</B><A>7</A><Z/></P>").unwrap());
    assert_eq!(p, Probe { a: 7, b: true });
    let e = p.write("P").unwrap();
    let names: Vec<&str> = e.children.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["A", "B"]);
}
