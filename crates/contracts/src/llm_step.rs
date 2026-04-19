use crate::llm_contract::LLMResponse;
use crate::validator::validate;
use crate::response_parser::Compiler;
use serde_json;

pub struct LLMClient {
    pub endpoint: String,
    pub api_key: String,
}

impl LLMClient {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        Self {
            endpoint: endpoint.into(),
            api_key: api_key.into(),
        }
    }

    pub fn call(&self, spec: &str) -> String {
        // This is a placeholder for the actual LLM API call logic
        // It returns a raw JSON string that matches the LLMResponse contract
        String::from(r#"{"actions": []}"#)
    }
}

pub fn llm_step(spec: &str) {
    let client = LLMClient::new("https://api.nemo-matrix.ai/v1", "NEMO_SECRET_KEY");

    let raw_response = client.call(spec);

    let parsed: LLMResponse = serde_json::from_str(&raw_response)
        .expect("NEMO_ERROR: FAILED_TO_PARSE_LLM_JSON");

    validate(&parsed).expect("NEMO_ERROR: CONTRACT_VALIDATION_FAILED");

    let instructions = Compiler::compile(parsed).expect("NEMO_ERROR: COMPILATION_FAILED");

    for instruction in instructions {
        println!("NEMO_EXEC: {}", instruction);
    }
}
