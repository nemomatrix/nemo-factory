#[derive(Clone)]
pub struct HistoryRecord {
    pub node_id: String,
    pub success_rate: f32,
    pub avg_latency: u64,
    pub failure_count: u32,
}

pub struct HistoryStore {
    pub records: Vec<HistoryRecord>,
}

impl HistoryStore {
    pub fn get_node_stats(&self, node_id: &str) -> Option<&HistoryRecord> {
        self.records.iter().find(|r| r.node_id == node_id)
    }
}
