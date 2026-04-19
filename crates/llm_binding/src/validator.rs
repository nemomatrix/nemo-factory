use crate::contract::{LLMOutput, ActionType};

pub fn validate(output: &LLMOutput) -> Result<(), String> {
    if output.actions.is_empty() {
        return Err("EMPTY ACTION SET".into());
    }

    for action in &output.actions {
        if action.path.contains("..") {
            return Err("PATH TRAVERSAL DETECTED".into());
        }

        match action.action {
            ActionType::CreateFile => {
                if action.content.is_none() {
                    return Err("CREATE FILE WITHOUT CONTENT".into());
                }
            }

            ActionType::PatchFile => {
                if action.diff.is_none() {
                    return Err("PATCH WITHOUT DIFF".into());
                }
            }

            ActionType::DeleteFile => {
                if action.path.contains("core") {
                    return Err("CORE DELETION BLOCKED".into());
                }
            }
        }
    }

    Ok(())
}
