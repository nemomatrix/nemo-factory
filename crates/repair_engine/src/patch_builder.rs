use crate::log_analyzer::parser::ParsedError;

pub fn build_patch(error: ParsedError) -> String {
    match error.error_type.as_str() {
        "RUST_BORROW" => "// fix borrow issue".into(),
        "MISSING_DEP" => "// add missing dependency".into(),
        _ => "// generic fix".into(),
    }
}
