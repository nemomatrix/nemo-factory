// NEMO - Decision Making Layer
use nemo_ci_runner::parser::ParsedError;

pub fn decide_fix(error: &ParsedError) -> &'static str {
    match error.error_type.as_str() {
        "RUST_BORROW" => "LOCAL_REFACTOR",
        "MISSING_DEP" => "DEPENDENCY_FIX",
        _ => "ISOLATED_PATCH",
    }
}
