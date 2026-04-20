pub struct Rollback;

impl Rollback {
    pub fn revert(change_id: &str) {
        println!("ROLLBACK: {}", change_id);
    }
}
