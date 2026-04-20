pub fn run_cycles(n: u32) {
    for i in 0..n {
        println!("CYCLE {}", i);

        let result = crate::executor::execute();

        crate::log_collector::collect(result);
    }
}
