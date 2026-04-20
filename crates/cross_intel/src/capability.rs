#[derive(Clone)]
pub struct Capability {
    pub name: String,
    pub domain: String,
    pub strength: f32,
    pub cost: f32,
    pub latency: u32,
}
