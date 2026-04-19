use crate::contract::LLMOutput;
use crate::guard::SecurityGate;

pub struct Compiler;

impl Compiler {
    pub fn compile(output: LLMOutput) -> Result<Vec<String>, String> {
        SecurityGate::pre_check(&output)?;

        let mut instructions = vec![];

        for action in output.actions {
            match action.action {
                crate::contract::ActionType::CreateFile => {
                    instructions.push(format!("CREATE: {}", action.path));
                }

                crate::contract::ActionType::PatchFile => {
                    instructions.push(format!("PATCH: {}", action.path));
                }

                crate::contract::ActionType::DeleteFile => {
                    instructions.push(format!("DELETE: {}", action.path));
                }
            }
        }

        Ok(instructions)
    }
}
