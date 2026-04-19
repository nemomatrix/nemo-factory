use crate::node::Node;

pub struct Scorer;

impl Scorer {
    pub fn score(node: &Node, ci_success: bool, latency: u64) -> f32 {
        let base = if ci_success { 1.0 } else { 0.0 };

        let performance_penalty = latency as f32 / 1000.0;

        base - performance_penalty + node.weight
    }
}
