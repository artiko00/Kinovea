use crate::run;

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn version_flag() {
    let (text, code) = run(&args(&["--version"]));
    assert_eq!(code, 0);
    assert_eq!(text, format!("kinovea {}", env!("CARGO_PKG_VERSION")));
}

#[test]
fn unknown_argument_exits_2() {
    let (text, code) = run(&args(&["--bogus"]));
    assert_eq!(code, 2);
    assert!(text.contains("--bogus"));
}
