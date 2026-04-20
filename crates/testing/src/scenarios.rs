use crate::case::TestCase;
use hardening::failure_injector::{FailureInjector, FailureMode};

pub fn apply_failure(case: &TestCase) {
    match case.failure.as_str() {
        "compile" => FailureInjector::inject(FailureMode::CompileError),
        "timeout" => FailureInjector::inject(FailureMode::Timeout),
        "corrupt" => FailureInjector::inject(FailureMode::CorruptOutput),
        "partial" => FailureInjector::inject(FailureMode::PartialWrite),
        _ => {}
    }
}
