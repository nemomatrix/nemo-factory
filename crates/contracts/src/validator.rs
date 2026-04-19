use crate::llm_contract::{LLMAction, LLMResponse};

pub fn validate(response: &LLMResponse) -> Result<(), String> {
    if response.actions.is_empty() {
        return Err("Empty action list".into());
    }

    for action in &response.actions {
        match action {
            LLMAction::CreateFile { path, content } => {
                validate_path(path)?;
                validate_content(content)?;
            }
            LLMAction::PatchFile { path, diff } => {
                validate_path(path)?;
                validate_diff(diff)?;
            }
            LLMAction::DeleteFile { path } => {
                validate_path(path)?;
            }
        }
    }

    Ok(())
}
