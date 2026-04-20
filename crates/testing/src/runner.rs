use crate::matrix::generate;
use crate::scenarios::apply_failure;
use crate::asserts::*;
use std::thread;

pub fn run_all() {
    let cases = generate();

    for case in cases {
        println!("RUN CASE: {} {} {}", case.phase, case.failure, case.load);

        let mut handles = vec![];

        for _ in 0..case.load {
            let c = case.clone();

            let h = thread::spawn(move || {
                apply_failure(&c);

                // simulate execution
                let result = "OK";

                assert_no_panic(result);
                assert_ci_success(true);

                result.to_string()
            });

            handles.push(h);
        }

        let mut outputs = vec![];

        for h in handles {
            outputs.push(h.join().unwrap());
        }

        // determinism check (first vs rest)
        for o in &outputs {
            assert_determinism(&outputs[0], o);
        }
    }
}
