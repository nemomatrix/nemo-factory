use crate::contract::LLMOutput;

pub struct SecurityGate;

impl SecurityGate {
    pub fn pre_check(output: &LLMOutput) -> Result<(), String> {
        if output.actions.len() > 50 {
            return Err("TOO MANY ACTIONS".into());
        }

        for action in &output.actions {
            if action.path.starts_with("/etc") {
                return Err("SYSTEM PATH BLOCKED".into());
            }

            if action.path.contains("secret") {
                return Err("SECRET ACCESS BLOCKED".into());
            }
        }

        Ok(())
    }
}
