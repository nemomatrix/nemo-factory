use std::process::{Command, Stdio};

pub struct Sandbox;

impl Sandbox {
    pub fn run(cmd: &str) -> Result<(), String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err("SANDBOX EXECUTION FAILED".into());
        }

        Ok(())
    }
}
