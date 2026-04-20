use crate::capability::Capability;

pub struct Validator;

impl Validator {
    pub fn validate(c: &Capability) -> bool {
        if c.cost > 0.9 {
            return false;
        }

        if c.latency > 1000 {
            return false;
        }

        true
    }
}
