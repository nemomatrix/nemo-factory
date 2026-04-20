use crate::proposal::{Proposal, ChangeType};
use crate::metrics::Metrics;

pub struct Planner;

impl Planner {
    pub fn generate(metrics: Vec<(String, Metrics)>) -> Vec<Proposal> {
        let mut proposals = vec![];

        for (node_id, m) in metrics {
            if m.instability > 0.7 {
                proposals.push(Proposal {
                    change: ChangeType::SplitNode,
                    target: node_id,
                    score: m.instability,
                });
            }

            if m.performance < 0.3 {
                proposals.push(Proposal {
                    change: ChangeType::RemoveNode,
                    target: node_id,
                    score: m.performance,
                });
            }
        }

        proposals
    }
}
