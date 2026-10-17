//! Kinovea entry point. The UI shell arrives in a later change; today it only reports its version.

#[cfg(test)]
mod main_test;

/// Resolves the CLI arguments into the text to print and the exit code.
pub fn run(args: &[String]) -> (String, i32) {
    match args.first().map(String::as_str) {
        Some("--version") | Some("-V") => (format!("kinovea {}", env!("CARGO_PKG_VERSION")), 0),
        Some(other) => (format!("unknown argument: {other}"), 2),
        None => (
            format!("kinovea {} (no UI yet)", env!("CARGO_PKG_VERSION")),
            0,
        ),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (text, code) = run(&args);
    if code == 0 {
        println!("{text}");
    } else {
        eprintln!("{text}");
    }
    std::process::exit(code);
}
