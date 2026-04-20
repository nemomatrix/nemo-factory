use crate::knowledge::Pattern;

pub struct PatternExtractor;

impl PatternExtractor {
    pub fn extract(code: &str) -> Pattern {
        Pattern {
            signature: code.lines().take(3).collect(),
            success_rate: 0.8,
        }
    }
}
