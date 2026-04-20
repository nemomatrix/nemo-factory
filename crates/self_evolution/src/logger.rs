use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger;

impl Logger {
    pub fn log(run_id: u32, content: &str) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("system.log")
            .unwrap();

        writeln!(file, "[{}] {}", run_id, content).unwrap();
    }
}
