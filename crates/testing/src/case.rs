#[derive(Clone)]
pub struct TestCase {
    pub phase: String,
    pub failure: String,
    pub load: u32,
    pub dag_size: u32,
    pub seed: u64,
}
