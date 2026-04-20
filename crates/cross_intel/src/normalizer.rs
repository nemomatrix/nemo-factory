use crate::capability::Capability;

pub struct Normalizer;

impl Normalizer {
    pub fn normalize(c: &mut Capability) {
        c.strength = c.strength.min(1.0);
        c.cost = c.cost.max(0.0);
    }
}
