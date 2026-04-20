use crate::adapter::Adapter;
use crate::validator::Validator;
use crate::capability::Capability;

pub struct Integrator;

impl Integrator {
    pub fn integrate(caps: Vec<Capability>) {
        for c in caps {
            if Validator::validate(&c) {
                let module = Adapter::adapt(&c);
                println!("INTEGRATED: {}", module);
            }
        }
    }
}
