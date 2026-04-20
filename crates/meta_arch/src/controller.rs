use crate::history::HistoryStore;
use crate::analyzer::Analyzer;
use crate::planner::Planner;
use crate::validator::Validator;
use crate::simulator::Simulator;
use crate::commit::Commit;

pub struct MetaController;

impl MetaController {
    pub fn run(history: &HistoryStore) {
        let metrics = Analyzer::analyze(history);

        let proposals = Planner::generate(metrics);

        for p in proposals {
            if Validator::validate(&p).is_err() {
                continue;
            }

            let sim = Simulator::simulate(&p);

            if sim.improvement > 0.25 {
                Commit::apply(&p);
            }
        }
    }
}
