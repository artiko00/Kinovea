//! Crate-level test and the seeded generator shared by property-style tests.

/// Tiny xorshift generator so property tests are deterministic without a dependency.
pub(crate) struct Rng(u64);

impl Rng {
    pub(crate) fn new(seed: u64) -> Self {
        Rng(seed.max(1))
    }

    /// Uniform value in `[0, 1)`.
    pub(crate) fn unit(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Uniform value in `[lo, hi)`.
    pub(crate) fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.unit()
    }
}

#[test]
fn crate_name_matches_package() {
    assert_eq!(crate::crate_name(), "kinovea-geometry");
}

#[test]
fn rng_is_deterministic_and_bounded() {
    let (mut a, mut b) = (Rng::new(7), Rng::new(7));
    for _ in 0..100 {
        let x = a.unit();
        assert_eq!(x, b.unit());
        assert!((0.0..1.0).contains(&x));
    }
}
