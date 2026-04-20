pub fn assert_ci_success(success: bool) {
    if !success {
        panic!("CI FAILURE DETECTED");
    }
}

pub fn assert_no_panic(log: &str) {
    if log.contains("panic") {
        panic!("PANIC DETECTED");
    }
}

pub fn assert_determinism(a: &str, b: &str) {
    if a != b {
        panic!("NON-DETERMINISTIC OUTPUT");
    }
}
