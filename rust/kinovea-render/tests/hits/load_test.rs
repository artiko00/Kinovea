use super::*;

#[test]
fn keyframe_keys_resolve_to_the_drawing_of_that_index() {
    let m = metadata("tests/cases/render/inputs/geometry.kva");
    let key = DrawingKey {
        container: "k0".into(),
        index: 0,
        element: "CrossMark".into(),
    };
    assert!(matches!(drawing(&m, &key), Drawing::CrossMark(_)));
}
