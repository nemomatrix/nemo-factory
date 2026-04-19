pub struct LoopLimiter {
    pub max_attempts: u32,
    pub current: u32,
}

impl LoopLimiter {
    pub fn new(max: u32) -> Self {
        Self {
            max_attempts: max,
            current: 0,
        }
    }

    pub fn can_continue(&mut self) -> bool {
        if self.current >= self.max_attempts {
            return false;
        }
        self.current += 1;
        true
    }
}
