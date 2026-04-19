use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMInput {
    pub spec: String,
    pub context: String,
    pub phase: String,
}
