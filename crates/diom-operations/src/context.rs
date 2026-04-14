#[derive(Debug, Clone)]
pub struct OpContext {
    /// The (monotonic) timestamp at which this object was enqueued for application.
    pub timestamp: jiff::Timestamp,
    /// The Raft log index. This is monotonically-increasing with every commit.
    pub log_index: u64,
    /// The raft term. This is monotonically-increasing with every leadership change.
    pub term: u64,
    /// Seed for deterministic random number generation inside `apply_real`.
    ///
    /// Generated once at request creation time and replicated through Raft,
    /// so every replica derives the same sequence of random values.
    pub rng_seed: u64,
}

impl OpContext {
    /// Returns a deterministic RNG seeded from this context.
    ///
    /// Use this for any randomness needed inside `apply_real` instead of
    /// calling `rand::rng()` directly.
    ///
    /// Uses `ChaCha8Rng` rather than `StdRng` because `StdRng`'s algorithm
    /// may change between `rand` versions, which would break determinism
    /// across nodes during rolling upgrades. `ChaCha8Rng` guarantees a
    /// stable output for a given seed.
    pub fn rng(&self) -> rand::rngs::ChaCha8Rng {
        rand::SeedableRng::seed_from_u64(self.rng_seed)
    }
}
