use crate::version::ThreePartsVersion;

#[test]
fn ordering() {
    let a: ThreePartsVersion = "2023.1".parse().unwrap();
    let b: ThreePartsVersion = "2023.1.1".parse().unwrap();
    let c: ThreePartsVersion = "2024.1".parse().unwrap();
    assert!(a < b);
    assert!(b < c);
    assert!(a < c);
    assert!(c > a);
    assert_eq!(a, ThreePartsVersion::new(2023, 1, 0));
}

#[test]
fn two_part_parse_and_text() {
    let v: ThreePartsVersion = "2026.1".parse().unwrap();
    assert_eq!((v.major, v.minor, v.revision), (2026, 1, 0));
    assert_eq!(v.to_string(), "2026.1");
    assert_eq!(ThreePartsVersion::new(2026, 1, 3).to_string(), "2026.1.3");
    assert!("2026".parse::<ThreePartsVersion>().is_err());
    assert!("".parse::<ThreePartsVersion>().is_err());
}
