use rand::{SeedableRng, rngs::StdRng};

pub fn seeded() -> StdRng {
    StdRng::seed_from_u64(42)
}
