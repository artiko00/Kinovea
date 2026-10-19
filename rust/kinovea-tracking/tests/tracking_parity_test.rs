//! Replays every `tracking` oracle case on the Rust trackers, with the tolerances of design D6:
//! template positions within 0.5 px and similarity within 0.02, circle center and radius within 1 px,
//! matched flags exact.

mod tracking_cases;
