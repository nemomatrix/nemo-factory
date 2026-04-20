pub struct Audit;

impl Audit {
    pub fn log(event: &str) {
        println!("[AUDIT] {}", event);
    }
}
