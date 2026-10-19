//! Headless rendering of Kinovea annotations: a backend-neutral `Renderer` trait, one painter
//! per drawing kind, scene composition, hit testing and backends (recording, CPU).

pub mod backend;
#[cfg(test)]
mod backend_test;
pub mod compose;
#[cfg(test)]
mod compose_test;
pub mod hit;
#[cfg(test)]
mod hit_test;
#[cfg(test)]
mod lib_test;
pub mod paint;
#[cfg(test)]
mod paint_test;
pub mod renderer;
#[cfg(test)]
mod renderer_test;
pub mod style;
#[cfg(test)]
mod style_test;
pub mod viewport;
#[cfg(test)]
mod viewport_test;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
