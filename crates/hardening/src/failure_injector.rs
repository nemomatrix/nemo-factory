pub enum FailureMode {
    CompileError,
    Timeout,
    CorruptOutput,
    PartialWrite,
}

pub struct FailureInjector;

impl FailureInjector {
    pub fn inject(mode: FailureMode) {
        match mode {
            FailureMode::CompileError => {
                panic!("Injected compile failure");
            }
            FailureMode::Timeout => {
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
            FailureMode::CorruptOutput => {
                println!("CORRUPTED DATA");
            }
            FailureMode::PartialWrite => {
                println!("WRITE INTERRUPTED");
            }
        }
    }
}
