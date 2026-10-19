use super::*;

#[test]
fn codes_follow_the_original_integers() {
    assert_eq!(Hit::Miss.code(), -1);
    assert_eq!(Hit::Body.code(), 0);
    assert_eq!(Hit::Handle(1).code(), 1);
    assert_eq!(Hit::Handle(3).code(), 3);
}
