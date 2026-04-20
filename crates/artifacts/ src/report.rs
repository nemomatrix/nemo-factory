pub struct ReportBuilder;

impl ReportBuilder {
    pub fn render(code: &str) {
        println!("FORMATTED OUTPUT:");
        println!("{}", code);
    }
}
