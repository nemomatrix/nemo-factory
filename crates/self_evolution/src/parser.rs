pub struct LogEntry {
    pub run_id: u32,
    pub status: String,
}

pub fn parse_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.split(':').collect();

    if parts.len() < 3 {
        return None;
    }

    Some(LogEntry {
        run_id: parts[1].parse().ok()?,
        status: parts[2].into(),
    })
}
