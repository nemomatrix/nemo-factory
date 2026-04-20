pub struct Report;

impl Report {
    pub fn print(total: usize, passed: usize) {
        println!("TOTAL: {}", total);
        println!("PASSED: {}", passed);
        println!("FAILED: {}", total - passed);
    }
}
