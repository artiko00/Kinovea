//! Kinovea `geometry` crate. Behavior is added by later OpenSpec changes.

#[cfg(test)]
mod lib_test;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
