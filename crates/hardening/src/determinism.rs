use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_state(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

pub fn assert_same(a: &str, b: &str) {
    if hash_state(a) != hash_state(b) {
        panic!("NON-DETERMINISTIC BEHAVIOR DETECTED");
    }
}
