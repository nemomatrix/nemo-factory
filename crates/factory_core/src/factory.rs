use crate::planner::Plan;
use crate::evaluator::Evaluator;
use crate::evolution::Evolver;
use crate::gate::Gate;

pub struct Factory {
    evaluator: Evaluator,
    evolver: Evolver,
    gate: Gate,
}

impl Factory {
    pub fn new() -> Self {
        Self {
            evaluator: Evaluator::new(),
            evolver: Evolver::new(),
            gate: Gate::new(),
        }
    }

    pub fn run_cycle(&mut self, plan: Plan) {
        // 1. Validate input plan
        if !self.gate.validate_plan(&plan) {
            panic!("INVALID PLAN BLOCKED");
        }

        // 2. Execute plan
        let result = self.evaluator.execute(&plan);

        // 3. Evaluate result via CI truth
        if !self.evaluator.verify(&result) {
            // 4. Auto-repair cycle
            let patch = self.evolver.generate_patch(&result);

            self.evolver.apply_patch(patch);

            return;
        }

        println!("CYCLE COMPLETED SUCCESSFULLY");
    }
}
