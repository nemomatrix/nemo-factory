use crate::proposal::Proposal;

pub struct SimulationResult {
    pub improvement: f32,
}

pub struct Simulator;

impl Simulator {
    pub fn simulate(p: &Proposal) -> SimulationResult {
        // deterministic simulation (placeholder logic)
        let improvement = match p.change {
            crate::proposal::ChangeType::SplitNode => 0.4,
            crate::proposal::ChangeType::RemoveNode => 0.2,
            _ => 0.1,
        };

        SimulationResult { improvement }
    }
}
