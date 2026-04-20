use std::collections::HashMap;
use crate::knowledge::Pattern;

pub struct LogEntry {
    pub status: String,
    pub message: String,
}

pub struct Filter;

impl Filter {
    pub fn allow(p: &Pattern) -> bool {
        p.success_rate > 0.6
    }

    pub fn cluster(entries: Vec<LogEntry>) -> HashMap<String, usize> {
        let mut map = HashMap::new();

        for e in entries {
            *map.entry(e.status).or_insert(0) += 1;
        }

        map
    }
}
