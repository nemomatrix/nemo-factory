use crate::capability::Capability;

pub fn compare(a: &Capability, b: &Capability) -> bool {
    a.strength > b.strength
}
