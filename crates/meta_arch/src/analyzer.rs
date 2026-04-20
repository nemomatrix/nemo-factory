use crate::history::HistoryStore;
use crate::metrics::Metrics;

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(history: &HistoryStore) -> Vec<(String, Metrics)> {
        let mut result = vec![];

        for record in &history.records {
            let metrics = Metrics::compute(record.success_rate, record.avg_latency);
            result.push((record.node_id.clone(), metrics));
        }

        result
    }
}
