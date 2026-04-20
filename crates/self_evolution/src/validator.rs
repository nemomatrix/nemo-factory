pub fn validate_fix(fix: &str) -> bool {
    if fix.contains("delete core") {
        return false;
    }

    if fix.len() > 200 {
        return false;
    }

    true
}
