pub struct Metrics {
    pub instability: f32,
    pub cost: f32,
    pub performance: f32,
}

impl Metrics {
    pub fn compute(success_rate: f32, latency: u64) -> Self {
        Self {
            instability: 1.0 - success_rate,
            cost: latency as f32 / 1000.0,
            performance: success_rate - (latency as f32 / 2000.0),
        }
    }
}
