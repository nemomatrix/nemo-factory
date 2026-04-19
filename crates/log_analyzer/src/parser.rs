// NEMO - Error Parsing Structure
pub struct ParsedError {
    pub file: Option<String>,
    pub error_type: String,
    pub message: String,
}
