//! Generic posture tools (`GenericPosture*`): data model parsed from the custom tool
//! descriptions, the embedded catalog of shipped tools and the constraint engine that moves
//! handles.

pub mod catalog;
#[cfg(test)]
mod catalog_test;
pub mod engine;
#[cfg(test)]
mod engine_test;
pub mod model;
#[cfg(test)]
mod model_test;
pub mod parser;
#[cfg(test)]
mod parser_test;

pub use catalog::{custom_postures, posture_by_id};
pub use engine::move_handle;
pub use model::*;
pub use parser::parse_posture;
