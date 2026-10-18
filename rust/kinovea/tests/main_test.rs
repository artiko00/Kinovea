use std::process::Command;

#[test]
fn version_flag_prints_cargo_version() {
    let out = Command::new(env!("CARGO_BIN_EXE_kinovea"))
        .arg("--version")
        .output()
        .expect("binary runs");
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert_eq!(
        stdout.trim(),
        format!("kinovea {}", env!("CARGO_PKG_VERSION"))
    );
}
