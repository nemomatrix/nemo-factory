use std::sync::{Mutex, Arc};

pub struct LockManager {
    lock: Arc<Mutex<()>>,
}

impl LockManager {
    pub fn new() -> Self {
        Self {
            lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn execute<F: FnOnce()>(&self, f: F) {
        let _guard = self.lock.lock().unwrap();
        f();
    }
}
