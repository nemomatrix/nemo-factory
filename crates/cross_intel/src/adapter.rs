use crate::capability::Capability;

pub struct Adapter;

impl Adapter {
    pub fn adapt(c: &Capability) -> String {
        format!("ENABLE_MODULE: {}", c.name)
    }
}
