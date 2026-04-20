use crate::parser::LogEntry;

pub struct Anomaly;

impl Anomaly {
    pub fn detect(entries: &[LogEntry]) -> Vec<LogEntry> {
        entries
            .iter()
            .filter(|e| e.status != "OK")
            .cloned()
            .collect()
    }
}
