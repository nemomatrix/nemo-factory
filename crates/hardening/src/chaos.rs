use crate::failure_injector::{FailureInjector, FailureMode};

pub struct Chaos;

impl Chaos {
    pub fn run() {
        FailureInjector::inject(FailureMode::CompileError);
        FailureInjector::inject(FailureMode::Timeout);
    }
}
