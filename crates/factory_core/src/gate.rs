use crate::planner::Plan;

pub struct Gate;

impl Gate {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_plan(&self, plan: &Plan) -> bool {
        if plan.tasks.is_empty() {
            return false;
        }

        if plan.tasks.len() > 100 {
            return false;
        }

        true
    }
}
