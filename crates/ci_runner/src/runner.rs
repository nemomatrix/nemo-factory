use std::process::{Command, Output};

pub struct CiRunner {
    pub workspace: String,
}

impl CiRunner {
    pub fn new(workspace: &str) -> Self {
        Self {
            workspace: workspace.to_string(),
        }
    }

    pub fn run_build(&self) -> Result<Output, String> {
        let output = Command::new("cargo")
            .arg("build")
            .current_dir(&self.workspace)
            .output()
            .map_err(|e| e.to_string())?;

        Ok(output)
    }

    pub fn run_tests(&self) -> Result<Output, String> {
        let output = Command::new("cargo")
            .arg("test")
            .current_dir(&self.workspace)
            .output()
            .map_err(|e| e.to_string())?;

        Ok(output)
    }
}
