use crate::migration::migrate_legacy;
use kinovea_core::preferences::dom::Element;

fn convert(drawing: &str) -> Element {
    let xml = format!("<KinoveaVideoAnalysis><Keyframes><Keyframe><Position>1</Position><Title>t</Title><Drawings>{drawing}</Drawings></Keyframe></Keyframes></KinoveaVideoAnalysis>");
    let out = migrate_legacy(&Element::parse(&xml).unwrap());
    out.child("Keyframes").unwrap().children[0].child("Drawings").unwrap().children[0].clone()
}

fn style_value(e: &Element, tag: &str) -> String {
    e.child("DrawingStyle").unwrap().children.iter().find(|c| c.name == tag).unwrap().child_text("Value").unwrap().to_owned()
}

#[test]
fn line_and_label() {
    let line = convert("<Drawing Type=\"DrawingLine2D\"><m_StartPoint>1;2</m_StartPoint><m_EndPoint>3;4</m_EndPoint><MeasureIsVisible>True</MeasureIsVisible><LineStyle><Size>3</Size><LineShape>DoubleArrow</LineShape><ColorRGB>1;2;3</ColorRGB></LineStyle><InfosFading><Enabled>True</Enabled></InfosFading></Drawing>");
    assert_eq!(line.name, "Line");
    assert_eq!((line.child_text("Start"), line.child_text("End"), line.child_text("ExtraData")), (Some("1;2"), Some("3;4"), Some("TotalDistance")));
    assert_eq!(line.child("MeasureLabel").unwrap().child_text("SpacePosition"), Some("1;2"));
    assert_eq!((style_value(&line, "Color"), style_value(&line, "LineSize"), style_value(&line, "Arrows")), ("1;2;3".into(), "3".into(), "DoubleArrow".into()));
    assert!(line.child("InfosFading").is_some());
    let label = convert("<Drawing Type=\"DrawingText\"><Text>Hi</Text><Position>5;6</Position><TextDecoration><BackColor>7;8;9</BackColor><FontSize>12</FontSize></TextDecoration></Drawing>");
    assert_eq!(label.attribute("name"), Some("Hi"));
    assert_eq!((label.child_text("ArrowVisible"), label.child_text("ArrowEnd")), (Some("false"), Some("5;6")));
    assert_eq!(style_value(&label, "FontSize"), "12");
}

#[test]
fn empty_value_of_elements_keep_an_end_tag() {
    let line = convert("<Drawing Type=\"DrawingLine2D\"><m_StartPoint/><m_EndPoint/><LineStyle><ColorRGB/></LineStyle></Drawing>");
    let doc = line.to_document();
    for tag in ["<Start></Start>", "<End></End>", "<SpacePosition></SpacePosition>", "<Value></Value>"] {
        assert!(doc.contains(tag), "{tag} missing in {doc}");
    }
    let label = convert("<Drawing Type=\"DrawingText\"><Text>Hi</Text><Position/></Drawing>");
    assert!(label.to_document().contains("<ArrowEnd></ArrowEnd>"));
}

#[test]
fn angle_circle_cross_pencil_and_passthrough() {
    let angle = convert("<Drawing Type=\"DrawingAngle2D\"><PointO>0;0</PointO><PointA>1;0</PointA><PointB>0;1</PointB><TextDecoration><BackColor>1;1;1</BackColor></TextDecoration><Measure>x</Measure></Drawing>");
    assert_eq!((angle.child_text("Signed"), angle.child_text("CCW"), angle.child_text("Supplementary")), (Some("false"), Some("true"), Some("false")));
    assert_eq!(angle.children.last().unwrap().name, "Measure");
    let circle = convert("<Drawing Type=\"DrawingCircle\"><Origin>4;4</Origin><Radius>9</Radius><LineStyle><Size>2</Size><ColorRGB>0;0;0</ColorRGB></LineStyle></Drawing>");
    assert_eq!((circle.child_text("ExtraData"), style_value(&circle, "PenShape").as_str()), (Some("None"), "Solid"));
    assert_eq!(circle.child("MeasureLabel").unwrap().child_text("SpacePosition"), Some("4;4"));
    let cross = convert("<Drawing Type=\"DrawingCross2D\"><CenterPoint>2;2</CenterPoint><CoordinatesVisible>False</CoordinatesVisible><LineStyle><ColorRGB>5;5;5</ColorRGB></LineStyle><Coordinates/></Drawing>");
    assert_eq!((cross.name.as_str(), cross.child_text("ExtraData"), style_value(&cross, "Color").as_str()), ("CrossMark", Some("None"), "5;5;5"));
    assert!(cross.child("Coordinates").is_some());
    let pencil = convert("<Drawing Type=\"DrawingPencil\"><PointList Count=\"1\"><Point>1;1</Point></PointList><LineStyle><Size>4</Size><ColorRGB>9;9;9</ColorRGB></LineStyle></Drawing>");
    assert_eq!(pencil.child("PointList").unwrap().children.len(), 1);
    assert_eq!(style_value(&pencil, "PenSize"), "4");
    let other = convert("<Drawing Type=\"DrawingPlane\"><Foo>1</Foo></Drawing>");
    assert_eq!((other.name.as_str(), other.attribute("Type")), ("Drawing", Some("DrawingPlane")));
}
