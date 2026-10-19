//! Kinovea `tracking` crate. Behavior is added by later OpenSpec changes.

pub mod ncc;
mod ncc_integral;
mod ncc_map;
mod ncc_masked;
mod ncc_unmasked;
pub mod refine;

#[cfg(test)]
mod lib_test;
#[cfg(test)]
mod ncc_integral_test;
#[cfg(test)]
mod ncc_map_test;
#[cfg(test)]
mod ncc_masked_test;
#[cfg(test)]
mod ncc_reference_test;
#[cfg(test)]
mod ncc_test;
#[cfg(test)]
mod ncc_unmasked_test;
#[cfg(test)]
mod refine_test;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
