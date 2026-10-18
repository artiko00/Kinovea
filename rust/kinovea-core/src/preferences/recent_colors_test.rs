use super::dom::Element;
use super::helpers::Color;
use super::recent_colors::RecentColors;

#[test]
fn capped_and_omitted_when_empty() {
    let mut rc = RecentColors::default();
    assert!(rc.write(12).is_none());
    rc.read(&Element::parse("<RecentColors><RecentColor>1;2;3</RecentColor><RecentColor>4;5;6</RecentColor></RecentColors>").unwrap());
    assert_eq!(rc.0, [Color::rgb(1, 2, 3), Color::rgb(4, 5, 6)]);
    assert_eq!(rc.write(1).unwrap().children.len(), 1);
}
