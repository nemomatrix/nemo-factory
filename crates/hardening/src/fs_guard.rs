pub struct FsGuard;

impl FsGuard {
    pub fn validate_path(path: &str) -> Result<(), String> {
        if path.contains("..") {
            return Err("PATH TRAVERSAL BLOCKED".into());
        }

        if path.starts_with("/etc") {
            return Err("SYSTEM PATH BLOCKED".into());
        }

        Ok(())
    }
}
