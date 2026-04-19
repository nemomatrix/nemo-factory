#[derive(Clone, Debug)]
pub struct ExecutionContext {
    pub phase: String,
    pub task_id: String,
    pub retry_count: u32,
    pub last_error: Option<String>,
}
