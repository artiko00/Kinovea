use super::*;

const SAMPLE: &str = "container,index,element,timestamp,scale,x,y,result\n\
k0,1,Line,0,0.5,4,6,0\n\
k0,1,Line,0,0.5,4,8,2\n\
chronos,0,Chrono,400,1,10,10,exception\n\
k0,1,Line,0,1,4,6,1\n";

fn line_key() -> DrawingKey {
    DrawingKey {
        container: "k0".into(),
        index: 1,
        element: "Line".into(),
    }
}

#[test]
fn parses_rows_and_marks_exceptions() {
    let rows = parse(SAMPLE);
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0].key.element, "Line");
    assert_eq!(rows[0].scale, 0.5);
    assert_eq!(rows[2].result, None);
}

#[test]
fn expected_keeps_only_the_requested_scale() {
    let rows = parse(SAMPLE);
    let at_half = expected(&rows, &line_key(), 0.5);
    assert_eq!(at_half.hits.len(), 2);
    assert_eq!(at_half.hits[&(4, 8)], 2);
    assert!(!at_half.exception);
    assert_eq!(expected(&rows, &line_key(), 2.0).hits.len(), 0);
}

#[test]
fn an_exception_row_marks_the_drawing_at_its_scale() {
    let rows = parse("container,index,element,timestamp,scale,x,y,result\nchronos,0,Chrono,400,1,10,10,exception\n");
    let key = DrawingKey {
        container: "chronos".into(),
        index: 0,
        element: "Chrono".into(),
    };
    assert!(expected(&rows, &key, 1.0).exception);
}
