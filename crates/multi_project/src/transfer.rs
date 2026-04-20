use crate::knowledge::Pattern;

pub struct TransferEngine;

impl TransferEngine {
    pub fn apply(pattern: &Pattern, target: &str) -> String {
        format!("// Applied pattern {}\n{}", pattern.signature, target)
    }
}
