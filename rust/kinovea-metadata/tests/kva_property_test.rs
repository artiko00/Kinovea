//! Idempotence properties of the KVA model. For every kind, a seeded random fragment is read,
//! written, read again and written again; both writes must match. No generator crate is used:
//! a fixed-seed LCG drives the fragments (`kva_property/gen.rs`).

mod kva_property {
    pub mod containers;
    pub mod containers_test;
    pub mod drawings_a;
    pub mod drawings_a_test;
    pub mod drawings_b;
    pub mod drawings_b_test;
    pub mod gen;
    pub mod gen_test;
    pub mod media_posture;
    pub mod media_posture_test;
    pub mod parts;
    pub mod parts_test;
    pub mod rounds;
    pub mod rounds_test;
    pub mod timed;
    pub mod timed_test;
}
