use crate::capability::Capability;

pub struct Selector;

impl Selector {
    pub fn select(mut caps: Vec<Capability>) -> Vec<Capability> {
        caps.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());
        caps.into_iter().take(3).collect()
    }
}
