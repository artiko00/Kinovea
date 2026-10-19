//! Renderer implementations: a recording backend for tests and the CPU backend.

pub mod cpu;
#[cfg(test)]
mod cpu_test;
pub mod recording;
#[cfg(test)]
mod recording_test;
