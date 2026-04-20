pub struct ArtifactGuard;

impl ArtifactGuard {
    pub fn validate(code: &str) -> Result<(), String> {
        if code.contains("fs::remove") {
            return Err("DANGEROUS CODE BLOCKED".into());
        }

        Ok(())
    }
}
