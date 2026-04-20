use crate::capability::Capability;

pub struct Evaluator;

impl Evaluator {
    pub fn score(c: &Capability) -> f32 {
        c.strength - (c.cost * 0.5) - (c.latency as f32 / 1000.0)
    }
}
