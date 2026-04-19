use reqwest;

pub struct LLMClient {
    pub endpoint: String,
    pub api_key: String,
}

impl LLMClient {
    pub fn call(&self, prompt: &str) -> String {
        // ⚠️ output MUST be JSON only
        let response = "{
            \"actions\": [
                {
                    \"action\": \"CreateFile\",
                    \"path\": \"src/generated.rs\",
                    \"content\": \"fn main() {}\"
                }
            ]
        }";

        response.to_string()
    }
}
