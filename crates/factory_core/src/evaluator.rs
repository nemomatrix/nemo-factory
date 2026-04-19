
use crate::planner::Plan;

pub struct EvaluationResult {
    pub success: bool,
    pub log: String,
}

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, plan: &Plan) -> EvaluationResult {
        for task in &plan.tasks {
            println!("EXEC TASK: {}", task);
        }

        EvaluationResult {
            success: false, // simulated CI failure for loop activation
            log: "build failed".into(),
        }
    }

    pub fn verify(&self, result: &EvaluationResult) -> bool {
        result.success
    }
}
