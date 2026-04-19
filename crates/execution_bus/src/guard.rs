
use crate::context::ExecutionContext;

pub struct Guard;

impl Guard {
    pub fn validate(ctx: &ExecutionContext) -> Result<(), String> {
        if ctx.retry_count > 20 {
            return Err("Execution halted: retry limit exceeded".into());
        }

        if ctx.phase == "LOCKED" && ctx.task_id.contains("override") {
            return Err("Illegal override attempt".into());
        }

        Ok(())
    }
}
