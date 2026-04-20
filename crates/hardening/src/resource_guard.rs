pub struct ResourceGuard;

impl ResourceGuard {
    pub fn check(memory_mb: u64, cpu_percent: u8) -> Result<(), String> {
        if memory_mb > 1024 {
            return Err("MEMORY LIMIT EXCEEDED".into());
        }

        if cpu_percent > 80 {
            return Err("CPU LIMIT EXCEEDED".into());
        }

        Ok(())
    }
}
