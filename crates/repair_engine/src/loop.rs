use crate::ci_runner::runner::CiRunner;
use crate::log_analyzer::classifier::classify_error;
use crate::patch_builder::build_patch;
use crate::limiter::LoopLimiter;

pub fn run_self_healing(workspace: &str) {
    let runner = CiRunner::new(workspace);
    let mut limiter = LoopLimiter::new(10);

    loop {
        if !limiter.can_continue() {
            panic!("Repair loop exceeded safety limit");
        }

        let build = runner.run_build().unwrap();

        if build.status.success() {
            let test = runner.run_tests().unwrap();

            if test.status.success() {
                println!("SYSTEM STABLE");
                break;
            }
        }

        let error_output = String::from_utf8_lossy(&build.stderr);
        let parsed = classify_error(&error_output);

        let patch = build_patch(parsed);

        println!("PATCH:\n{}", patch);

        // لاحقاً: FS engine apply_patch()
    }
}
