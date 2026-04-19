use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionType {
    CreateFile,
    PatchFile,
    DeleteFile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub action: ActionType,
    pub path: String,
    pub content: Option<String>,
    pub diff: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMOutput {
    pub actions: Vec<Action>,
}
