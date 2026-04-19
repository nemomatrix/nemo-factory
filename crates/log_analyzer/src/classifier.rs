use crate::parser::ParsedError;

pub fn classify_error(raw: &str) -> ParsedError {
    if raw.contains("borrow checker") {
        return ParsedError {
            file: None,
            error_type: "RUST_BORROW".into(),
            message: raw.to_string(),
        };
    }

    if raw.contains("not found") {
        return ParsedError {
            file: None,
            error_type: "MISSING_DEP".into(),
            message: raw.to_string(),
        };
    }

    ParsedError {
        file: None,
        error_type: "UNKNOWN".into(),
        message: raw.to_string(),
    }
}
