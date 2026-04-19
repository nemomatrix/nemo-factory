use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum LLMAction {
    CreateFile {
        path: String,
        content: String,
    },
    PatchFile {
        path: String,
        diff: String,
    },
    DeleteFile {
        path: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMResponse {
    pub actions: Vec<LLMAction>,
}
